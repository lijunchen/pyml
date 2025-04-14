use lexer::TokenKind;

use crate::{
    parser::{MarkerClosed, Parser},
    syntax::MySyntaxKind,
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
            p.close(m, MySyntaxKind::PATTERN_BOOL)
        }
        TokenKind::WildcardKeyword => {
            let m = p.open();
            p.advance();
            p.close(m, MySyntaxKind::PATTERN_WILDCARD)
        }
        TokenKind::LParen => {
            let m = p.open();
            p.advance();

            while p.at_any(PATTERN_FIRST) {
                pattern(p);
                p.eat(TokenKind::Comma);
            }

            p.expect(TokenKind::RParen);
            p.close(m, MySyntaxKind::PATTERN_TUPLE)
        }
        TokenKind::Lident => {
            let m = p.open();
            p.advance();
            p.close(m, MySyntaxKind::PATTERN_VARIABLE)
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
            p.close(m, MySyntaxKind::PATTERN_CONSTR)
        }
        _ => unreachable!(),
    }
}
