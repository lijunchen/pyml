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
        // ExprLiteral = 'int' | 'true' | 'false'
        TokenKind::Int32 | TokenKind::TrueKeyword | TokenKind::FalseKeyword => {
            let m = p.open();
            p.advance();
            p.close(m, MySyntaxKind::ExprLiteral)
        }
        // ExprName = 'name'
        TokenKind::Lident => {
            let m = p.open();
            p.advance();
            p.close(m, MySyntaxKind::ExprName)
        }
        TokenKind::Uident => {
            let m = p.open();
            p.advance();
            p.close(m, MySyntaxKind::ConstructorName)
        }
        // ExprParen = '( Expr ')'
        TokenKind::LParen => {
            let m = p.open();
            p.expect(TokenKind::LParen);
            if p.at(TokenKind::RParen) {
                p.expect(TokenKind::RParen);
                p.close(m, MySyntaxKind::ExprUnit)
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
                    p.close(m, MySyntaxKind::ExprTuple)
                } else {
                    p.expect(TokenKind::RParen);
                    p.close(m, MySyntaxKind::ExprTuple)
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
            p.close(m, MySyntaxKind::ExprMatch)
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

fn prefix_binding_power(op: TokenKind) -> Option<((), u8)> {
    match op {
        _ => None,
    }
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
    if token == TokenKind::LetKeyword {
        let_expr(p);
        return;
    }
    expr_bp(p, 0)
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
    expr(p);
    p.expect(TokenKind::InKeyword);
    if !p.at_any(EXPR_FIRST) {
        p.advance_with_error("let .. in [_] expected an expression");
        return;
    }
    expr(p);
    p.close(m, MySyntaxKind::ExprLet);
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
                lhs = m.completed(p, MySyntaxKind::ExprCall)
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
                lhs = m.completed(p, MySyntaxKind::ExprBinary);
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
    p.close(m, MySyntaxKind::ArgList);
}

// Arg = Expr ','?
fn arg(p: &mut Parser) {
    let m = p.open();
    expr(p);
    if !p.at(TokenKind::RParen) {
        p.expect(TokenKind::Comma);
    }
    p.close(m, MySyntaxKind::Arg);
}
