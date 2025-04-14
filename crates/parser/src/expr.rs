use crate::{
    file::block,
    parser::{MarkerClosed, Parser},
    pattern,
    syntax::MySyntaxKind,
};
use lexer::TokenKind;

pub const EXPR_FIRST: &[TokenKind] = &[
    TokenKind::Int32,
    TokenKind::Lident,
    TokenKind::Uident,
    TokenKind::TrueKeyword,
    TokenKind::FalseKeyword,
    TokenKind::LParen,
    TokenKind::IfKeyword,
    TokenKind::LetKeyword,
    TokenKind::MatchKeyword,
];

fn atom(p: &mut Parser) -> Option<MarkerClosed> {
    let result = match p.peek() {
        TokenKind::Int32 => {
            let m = p.open();
            p.advance();
            p.close(m, MySyntaxKind::EXPR_INT)
        }
        TokenKind::TrueKeyword | TokenKind::FalseKeyword => {
            let m = p.open();
            p.advance();
            p.close(m, MySyntaxKind::EXPR_BOOL)
        }
        // ExprName = 'name'
        TokenKind::Lident => {
            let m = p.open();
            p.advance();
            p.close(m, MySyntaxKind::EXPR_LIDENT)
        }
        TokenKind::Uident => {
            let m = p.open();
            p.advance();
            p.close(m, MySyntaxKind::EXPR_UIDENT)
        }
        // ExprParen = '( Expr ')'
        TokenKind::LParen => {
            let m = p.open();
            p.expect(TokenKind::LParen);
            if p.at(TokenKind::RParen) {
                p.expect(TokenKind::RParen);
                p.close(m, MySyntaxKind::EXPR_UNIT)
            } else {
                expr(p);
                if p.at(TokenKind::Comma) {
                    while p.at(TokenKind::Comma) {
                        p.expect(TokenKind::Comma);
                        if p.at_any(EXPR_FIRST) {
                            expr(p);
                        }
                    }
                    p.expect(TokenKind::RParen);
                    p.close(m, MySyntaxKind::EXPR_TUPLE)
                } else {
                    p.expect(TokenKind::RParen);
                    p.close(m, MySyntaxKind::EXPR_TUPLE)
                }
            }
        }
        TokenKind::MatchKeyword => {
            let m = p.open();
            p.expect(TokenKind::MatchKeyword);
            expr(p);
            if p.at(TokenKind::LBrace) {
                match_arm_list(p);
            }
            p.close(m, MySyntaxKind::EXPR_MATCH)
        }
        _ => {
            dbg!(&p.peek());
            dbg!(&EXPR_FIRST);
            dbg!(EXPR_FIRST.contains(&p.peek()));
            assert!(!p.at_any(EXPR_FIRST));
            return None;
        }
    };
    Some(result)
}

pub fn match_arm_list(p: &mut Parser) {
    assert!(p.at(TokenKind::LBrace));
    let m = p.open();
    p.expect(TokenKind::LBrace);
    while !p.eof() && !p.at(TokenKind::RBrace) {
        match_arm(p);
        p.eat(TokenKind::Comma);
    }
    p.expect(TokenKind::RBrace);
    p.close(m, MySyntaxKind::MATCH_ARM_LIST);
}

fn match_arm(p: &mut Parser) {
    let m = p.open();
    super::pattern::pattern(p);
    p.expect(TokenKind::FatArrow);
    if p.at(TokenKind::LBrace) {
        block(p);
    } else {
        expr(p);
    }
    p.close(m, MySyntaxKind::MATCH_ARM);
}

fn prefix_binding_power(_op: TokenKind) -> Option<((), u8)> {
    None
}

fn postfix_binding_power(op: TokenKind) -> Option<(u8, ())> {
    match op {
        TokenKind::LParen => Some((7, ())),
        _ => None,
    }
}

fn infix_binding_power(op: TokenKind) -> Option<(u8, u8)> {
    match op {
        TokenKind::Plus | TokenKind::Minus => Some((13, 14)),
        TokenKind::Star | TokenKind::Slash => Some((15, 16)),
        TokenKind::Dot => Some((23, 24)),
        _ => None,
    }
}

pub fn expr(p: &mut Parser) {
    let token = p.peek();
    // let m = p.open();
    if token == TokenKind::LetKeyword {
        let_expr(p);
        // p.close(m, MySyntaxKind::EXPR);
        return;
    }
    expr_bp(p, 0);
    // p.close(m, MySyntaxKind::EXPR);
}

fn let_expr(p: &mut Parser) {
    assert!(p.at(TokenKind::LetKeyword));
    let m = p.open();
    p.expect(TokenKind::LetKeyword);
    pattern::pattern(p);
    p.expect(TokenKind::Eq);
    if !p.at_any(EXPR_FIRST) {
        p.advance_with_error("let [_] expected an expression");
        return;
    }
    {
        let n = p.open();
        expr(p);
        p.close(n, MySyntaxKind::EXPR_LET_VALUE);
    }
    p.expect(TokenKind::InKeyword);
    if !p.at_any(EXPR_FIRST) {
        p.advance_with_error("let .. in [_] expected an expression");
        return;
    }
    {
        let n = p.open();
        expr(p);
        p.close(n, MySyntaxKind::EXPR_LET_BODY);
    }
    p.close(m, MySyntaxKind::EXPR_LET);
}

fn expr_bp(p: &mut Parser, min_bp: u8) {
    let op = p.peek();

    let lhs = if let Some(((), r_bp)) = prefix_binding_power(op) {
        let _m = p.open();
        p.advance();
        expr_bp(p, r_bp);
        todo!()
    } else {
        atom(p)
    };

    if lhs.is_none() {
        return;
    }
    let mut lhs = lhs.unwrap();

    loop {
        if p.eof() {
            break;
        }

        let op = p.peek();

        if let Some((l_bp, ())) = postfix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            if p.at(TokenKind::LParen) {
                let m = lhs.precede(p);
                arg_list(p);
                lhs = m.completed(p, MySyntaxKind::EXPR_CALL)
            } else {
                let op = p.peek();
                p.advance_with_error(&format!("unexpected postfix operator {:?}", op));
            }
            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            let m = lhs.precede(p);
            if op == TokenKind::Colon {
                todo!()
            } else {
                p.advance();
                expr_bp(p, r_bp);
                lhs = m.completed(p, MySyntaxKind::EXPR_BINARY);
            }
            continue;
        }
        break;
    }
}

// ArgList = '(' Arg* ')'
pub fn arg_list(p: &mut Parser) {
    assert!(p.at(TokenKind::LParen));
    let m = p.open();
    p.expect(TokenKind::LParen);
    while !p.at(TokenKind::RParen) && !p.eof() {
        if p.at_any(EXPR_FIRST) {
            arg(p);
        } else {
            break;
        }
    }
    p.expect(TokenKind::RParen);
    p.close(m, MySyntaxKind::ARG_LIST);
}

// Arg = Expr ','?
fn arg(p: &mut Parser) {
    let m = p.open();
    expr(p);
    if !p.at(TokenKind::RParen) {
        p.expect(TokenKind::Comma);
    }
    p.close(m, MySyntaxKind::ARG);
}
