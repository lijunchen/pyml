use crate::{
    parser::{MarkerClosed, Parser},
    syntax::SyntaxKind,
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
];

fn atom(p: &mut Parser) -> Option<MarkerClosed> {
    let result = match p.peek() {
        // ExprLiteral = 'int' | 'true' | 'false'
        TokenKind::Int32 | TokenKind::TrueKeyword | TokenKind::FalseKeyword => {
            let m = p.open();
            p.advance();
            p.close(m, SyntaxKind::ExprLiteral)
        }
        // ExprName = 'name'
        TokenKind::Lident => {
            let m = p.open();
            p.advance();
            p.close(m, SyntaxKind::ExprName)
        }
        // ExprParen = '( Expr ')'
        TokenKind::LParen => {
            let m = p.open();
            p.expect(TokenKind::LParen);
            expr(p);
            p.expect(TokenKind::RParen);
            p.close(m, SyntaxKind::ExprParen)
        }
        _ => {
            assert!(!p.at_any(EXPR_FIRST));
            return None;
        }
    };
    Some(result)
}

#[allow(unused)]
fn prefix_binding_power(op: TokenKind) -> Option<((), u8)> {
    match op {
        _ => None,
    }
}

#[allow(unused)]
fn postfix_binding_power(op: TokenKind) -> (u8, ()) {
    todo!()
}

fn infix_binding_power(op: TokenKind) -> Option<(u8, u8)> {
    match op {
        TokenKind::Plus | TokenKind::Minus => Some((1, 2)),
        TokenKind::Star | TokenKind::Slash => Some((3, 4)),
        _ => None,
    }
}

pub fn expr(p: &mut Parser) {
    expr_bp(p, 0)
}

fn expr_bp(p: &mut Parser, min_bp: u8) {
    let Some(mut lhs) = atom(p) else {
        return;
    };

    while p.at(TokenKind::LParen) {
        let m = lhs.precede(p);
        arg_list(p);
        lhs = m.completed(p, SyntaxKind::ExprCall);
    }

    loop {
        if p.eof() {
            break;
        }

        let op = p.peek();

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            let m = lhs.precede(p);
            p.advance();
            expr_bp(p, r_bp);
            lhs = m.completed(p, SyntaxKind::ExprBinary);
        } else {
            break;
        }
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
    p.close(m, SyntaxKind::ArgList);
}

// Arg = Expr ','?
fn arg(p: &mut Parser) {
    let m = p.open();
    expr(p);
    if !p.at(TokenKind::RParen) {
        p.expect(TokenKind::Comma);
    }
    p.close(m, SyntaxKind::Arg);
}
