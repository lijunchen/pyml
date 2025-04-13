use lexer::TokenKind;

use crate::{
    parser::{MarkerClosed, Parser},
    syntax::SyntaxKind,
};

pub const PATTERN_FIRST: &[TokenKind] = &[
    TokenKind::TrueKeyword,
    TokenKind::FalseKeyword,
    TokenKind::Uident,
    TokenKind::Lident,
    TokenKind::LParen,
    TokenKind::WildcardKeyword,
];

pub fn pattern(p: &mut Parser) {
    let _ = simple_pattern(p);
}

fn simple_pattern(p: &mut Parser) -> MarkerClosed {
    if !p.at_any(PATTERN_FIRST) {
        dbg!(&p.filename);
        dbg!(&p.peek());
    }
    assert!(p.at_any(PATTERN_FIRST));
    match p.peek() {
        TokenKind::TrueKeyword | TokenKind::FalseKeyword => {
            let m = p.open();
            p.advance();
            p.close(m, SyntaxKind::Pattern)
        }
        TokenKind::WildcardKeyword => {
            let m = p.open();
            p.advance();
            p.close(m, SyntaxKind::Pattern)
        }
        TokenKind::LParen => {
            let m = p.open();
            p.advance();

            while p.at_any(PATTERN_FIRST) {
                pattern(p);
                p.eat(TokenKind::Comma);
            }

            p.expect(TokenKind::RParen);
            p.close(m, SyntaxKind::PatternTuple)
        }
        TokenKind::Lident => {
            let m = p.open();
            p.advance();
            p.close(m, SyntaxKind::PatternVariable)
        }
        TokenKind::Uident => {
            let m = p.open();
            p.advance();
            if p.at(TokenKind::LParen) {
                p.expect(TokenKind::LParen);
                while p.at_any(PATTERN_FIRST) {
                    pattern(p);
                    p.eat(TokenKind::Comma);
                }
                p.expect(TokenKind::RParen);
            }
            p.close(m, SyntaxKind::PatternConstr)
        }
        _ => unreachable!(),
    }
}
