use lexer::{T, TokenKind};

use crate::{
    parser::{MarkerClosed, Parser},
    syntax::MySyntaxKind,
};

pub const PATTERN_FIRST: &[TokenKind] =
    &[T![true], T![false], T![uident], T![lident], T!['('], T![_]];

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
        T![true] | T![false] => {
            let m = p.open();
            p.advance();
            p.close(m, MySyntaxKind::PATTERN_BOOL)
        }
        T![_] => {
            let m = p.open();
            p.advance();
            p.close(m, MySyntaxKind::PATTERN_WILDCARD)
        }
        T!['('] => {
            let m = p.open();
            p.advance();
            if p.at(T![')']) {
                p.expect(T![')']);
                return p.close(m, MySyntaxKind::PATTERN_UNIT);
            }

            while p.at_any(PATTERN_FIRST) {
                pattern(p);
                p.eat(T![,]);
            }

            p.expect(T![')']);
            p.close(m, MySyntaxKind::PATTERN_TUPLE)
        }
        T![lident] => {
            let m = p.open();
            p.advance();
            p.close(m, MySyntaxKind::PATTERN_VARIABLE)
        }
        T![uident] => {
            let m = p.open();
            p.advance();
            if p.at(T!['(']) {
                p.expect(T!['(']);
                while p.at_any(PATTERN_FIRST) {
                    pattern(p);
                    p.eat(T![,]);
                }
                p.expect(T![')']);
            }
            p.close(m, MySyntaxKind::PATTERN_CONSTR)
        }
        _ => unreachable!(),
    }
}
