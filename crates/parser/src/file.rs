use lexer::{T, TokenKind};

use crate::{
    expr::{EXPR_FIRST, expr},
    parser::Parser,
    syntax::MySyntaxKind,
};

pub fn file(p: &mut Parser) {
    let m = p.open();
    while !p.eof() {
        if p.at(T![fn]) {
            func(p)
        } else if p.at(T![enum]) {
            enum_def(p)
        } else if p.at(T![trait]) {
            trait_def(p)
        } else if p.at(T![impl]) {
            impl_block(p)
        } else if p.at_any(EXPR_FIRST) {
            expr(p)
        } else {
            p.advance_with_error("expected a function")
        }
    }
    p.close(m, MySyntaxKind::FILE);
}

fn func(p: &mut Parser) {
    assert!(p.at(T![fn]));
    let m = p.open();
    p.expect(T![fn]);
    p.expect(T![lident]);
    if p.at(T!['[']) {
        generic_list(p);
    }
    if p.at(T!['(']) {
        param_list(p);
    }
    if p.eat(T![->]) {
        type_expr(p);
    }
    if p.at(T!['{']) {
        block(p);
    }
    p.close(m, MySyntaxKind::FN);
}

fn impl_block(p: &mut Parser) {
    assert!(p.at(T![impl]));
    let m = p.open();
    p.expect(T![impl]);
    p.expect(T![uident]);
    p.expect(T![for]);
    type_expr(p);
    if p.at(T!['{']) {
        p.advance();
        while !p.at(T!['}']) && !p.eof() {
            if p.at(T![fn]) {
                func(p);
            } else {
                p.advance_with_error("expected a function");
            }
        }
        p.expect(T!['}']);
    }
    p.close(m, MySyntaxKind::IMPL);
}

fn trait_def(p: &mut Parser) {
    assert!(p.at(T![trait]));
    let m = p.open();
    p.expect(T![trait]);
    p.expect(T![uident]);
    if p.at(T!['[']) {
        generic_list(p);
    }
    if p.at(T!['{']) {
        trait_method_list(p);
    }
    p.close(m, MySyntaxKind::TRAIT);
}

// {
//   fn foo(x: Int) -> Int;
//   fn bar(x: Int) -> Int;
// }

fn trait_method_list(p: &mut Parser) {
    assert!(p.at(T!['{']));
    p.expect(T!['{']);
    let m = p.open();
    while !p.at(T!['}']) && !p.eof() {
        if p.at(T![fn]) {
            trait_method(p);
            p.eat(T![;]);
        } else {
            p.advance_with_error("expected a method");
        }
    }
    p.expect(T!['}']);
    p.close(m, MySyntaxKind::TRAIT_METHOD_SIG_LIST);
}

fn trait_method(p: &mut Parser) {
    assert!(p.at(T![fn]));
    let m = p.open();
    p.expect(T![fn]);
    p.expect(T![lident]);
    if p.at(T!['(']) {
        type_list(p);
    }
    if p.eat(T![->]) {
        type_expr(p);
    }
    p.close(m, MySyntaxKind::TRAIT_METHOD_SIG);
}

fn enum_def(p: &mut Parser) {
    assert!(p.at(T![enum]));
    let m = p.open();
    p.expect(T![enum]);
    p.expect(T![uident]);
    if p.at(T!['[']) {
        generic_list(p);
    }
    if p.at(T!['{']) {
        variant_list(p);
    }
    p.close(m, MySyntaxKind::ENUM);
}

fn variant_list(p: &mut Parser) {
    assert!(p.at(T!['{']));
    p.expect(T!['{']);
    let m = p.open();
    while !p.at(T!['}']) && !p.eof() {
        if p.at(T![uident]) {
            variant(p);
            p.eat(T![,]);
        } else {
            p.advance_with_error("expected a variant");
        }
    }
    p.expect(T!['}']);
    p.close(m, MySyntaxKind::VARIANT_LIST);
}

fn variant(p: &mut Parser) {
    assert!(p.at(T![uident]));
    let m = p.open();
    p.expect(T![uident]);
    if p.at(T!['(']) {
        type_list(p);
    }
    p.close(m, MySyntaxKind::VARIANT);
}

const TYPE_FIRST: &[TokenKind] = &[T![Unit], T![Bool], T![Int], T!['('], T![uident]];

fn type_list(p: &mut Parser) {
    assert!(p.at(T!['(']));
    let m = p.open();
    p.expect(T!['(']);
    while !p.at(T![')']) && !p.eof() {
        if p.at_any(TYPE_FIRST) {
            type_expr(p);
            p.eat(T![,]);
        } else {
            p.advance_with_error("expected a type");
        }
    }
    p.expect(T![')']);
    p.close(m, MySyntaxKind::TYPE_LIST);
}

fn generic(p: &mut Parser) {
    assert!(p.at(T![uident]));
    let m = p.open();
    p.expect(T![uident]);
    if p.at(T!['[']) {
        generic_list(p);
    }
    p.close(m, MySyntaxKind::GENERIC);
}

fn generic_list(p: &mut Parser) {
    assert!(p.at(T!['[']));
    let m = p.open();
    p.expect(T!['[']);
    while !p.at(T![']']) && !p.eof() {
        if p.at(T![uident]) {
            generic(p);
            p.eat(T![,]);
        } else {
            p.advance_with_error("expected a generic");
        }
    }
    p.expect(T![']']);
    p.close(m, MySyntaxKind::GENERIC_LIST);
}

const PARAM_LIST_RECOVERY: &[TokenKind] = &[T![->], T!['{'], T![fn]];
fn param_list(p: &mut Parser) {
    assert!(p.at(T!['(']));
    let m = p.open();

    p.expect(T!['(']);
    while !p.at(T![')']) && !p.eof() {
        if p.at(T![lident]) {
            param(p);
        } else {
            if p.at_any(PARAM_LIST_RECOVERY) {
                break;
            }
            p.advance_with_error("expected a parameter");
        }
    }
    p.expect(T![')']);
    p.close(m, MySyntaxKind::PARAM_LIST);
}

fn param(p: &mut Parser) {
    assert!(p.at(T![lident]));
    let m = p.open();
    p.expect(T![lident]);
    p.expect(T![:]);
    type_expr(p);
    if !p.at(T![')']) {
        p.expect(T![,]);
    }
    p.close(m, MySyntaxKind::PARAM);
}

fn type_expr(p: &mut Parser) {
    let m = p.open();
    if p.at(T![Unit]) {
        p.advance();
        p.close(m, MySyntaxKind::TYPE_UNIT);
    } else if p.at(T![Bool]) {
        p.advance();
        p.close(m, MySyntaxKind::TYPE_BOOL);
    } else if p.at(T![Int]) {
        p.advance();
        p.close(m, MySyntaxKind::TYPE_INT);
    } else if p.at(T![String]) {
        p.advance();
        p.close(m, MySyntaxKind::TYPE_STRING);
    } else if p.at(T!['(']) {
        type_list(p);
        p.close(m, MySyntaxKind::TYPE_TUPLE);
    } else if p.at(T![uident]) {
        p.advance();
        if p.at(T!['[']) {
            type_param_list(p);
        }
        p.close(m, MySyntaxKind::TYPE_TAPP);
    } else {
        p.advance_with_error("expected a type");
    }
}

fn type_param_list(p: &mut Parser) {
    assert!(p.at(T!['[']));
    let m = p.open();
    p.expect(T!['[']);
    while !p.at(T![']']) && !p.eof() {
        if p.at_any(TYPE_FIRST) {
            type_expr(p);
            p.eat(T![,]);
        } else {
            p.advance_with_error("expected a type");
        }
    }
    p.expect(T![']']);
    p.close(m, MySyntaxKind::TYPE_PARAM_LIST);
}

pub fn block(p: &mut Parser) {
    assert!(p.at(T!['{']));
    let m = p.open();
    p.expect(T!['{']);
    expr(p);
    p.expect(T!['}']);
    p.close(m, MySyntaxKind::BLOCK);
}
