use lexer::TokenKind;

use crate::{
    expr::{EXPR_FIRST, expr},
    parser::Parser,
    syntax::MySyntaxKind,
};

pub fn file(p: &mut Parser) {
    let m = p.open();
    while !p.eof() {
        if p.at(TokenKind::FnKeyword) {
            func(p)
        } else if p.at(TokenKind::EnumKeyword) {
            enum_def(p)
        } else if p.at_any(EXPR_FIRST) {
            expr(p)
        } else {
            p.advance_with_error("expected a function")
        }
    }
    p.close(m, MySyntaxKind::FILE);
}

fn func(p: &mut Parser) {
    assert!(p.at(TokenKind::FnKeyword));
    let m = p.open();
    p.expect(TokenKind::FnKeyword);
    p.expect(TokenKind::Lident);
    if p.at(TokenKind::LParen) {
        param_list(p);
    }
    if p.eat(TokenKind::Arrow) {
        type_expr(p);
    }
    if p.at(TokenKind::LBrace) {
        block(p);
    }
    p.close(m, MySyntaxKind::FN);
}

fn enum_def(p: &mut Parser) {
    assert!(p.at(TokenKind::EnumKeyword));
    let m = p.open();
    p.expect(TokenKind::EnumKeyword);
    p.expect(TokenKind::Uident);
    if p.at(TokenKind::LBrace) {
        variant_list(p);
    }
    p.close(m, MySyntaxKind::ENUM);
}

fn variant_list(p: &mut Parser) {
    assert!(p.at(TokenKind::LBrace));
    p.expect(TokenKind::LBrace);
    let m = p.open();
    while !p.at(TokenKind::RBrace) && !p.eof() {
        if p.at(TokenKind::Uident) {
            variant(p);
            p.eat(TokenKind::Comma);
        } else {
            p.advance_with_error("expected a variant");
        }
    }
    p.expect(TokenKind::RBrace);
    p.close(m, MySyntaxKind::VARIANT_LIST);
}

fn variant(p: &mut Parser) {
    assert!(p.at(TokenKind::Uident));
    let m = p.open();
    p.expect(TokenKind::Uident);
    if p.at(TokenKind::LParen) {
        type_list(p);
    }
    p.close(m, MySyntaxKind::VARIANT);
}

const TYPE_FIRST: &[TokenKind] = &[
    TokenKind::UnitKeyword,
    TokenKind::BoolKeyword,
    TokenKind::IntKeyword,
    TokenKind::LParen,
    TokenKind::Uident,
];

fn type_list(p: &mut Parser) {
    assert!(p.at(TokenKind::LParen));
    let m = p.open();
    p.expect(TokenKind::LParen);
    while !p.at(TokenKind::RParen) && !p.eof() {
        if p.at_any(TYPE_FIRST) {
            type_expr(p);
            p.eat(TokenKind::Comma);
        } else {
            p.advance_with_error("expected a type");
        }
    }
    p.expect(TokenKind::RParen);
    p.close(m, MySyntaxKind::TYPE_LIST);
}

// ParamList: '(' Param* ')'
const PARAM_LIST_RECOVERY: &[TokenKind] =
    &[TokenKind::Arrow, TokenKind::LBrace, TokenKind::FnKeyword];
fn param_list(p: &mut Parser) {
    assert!(p.at(TokenKind::LParen));
    let m = p.open();

    p.expect(TokenKind::LParen);
    while !p.at(TokenKind::RParen) && !p.eof() {
        if p.at(TokenKind::Lident) {
            param(p);
        } else {
            if p.at_any(PARAM_LIST_RECOVERY) {
                break;
            }
            p.advance_with_error("expected a parameter");
        }
    }
    p.expect(TokenKind::RParen);
    p.close(m, MySyntaxKind::PARAM_LIST);
}

// Param = 'name' ':' TypeExpr ','?
fn param(p: &mut Parser) {
    assert!(p.at(TokenKind::Lident));
    let m = p.open();
    p.expect(TokenKind::Lident);
    p.expect(TokenKind::Colon);
    type_expr(p);
    if !p.at(TokenKind::RParen) {
        p.expect(TokenKind::Comma);
    }
    p.close(m, MySyntaxKind::PARAM);
}

// TypeExpr: 'Uident'
// Bool, Unit, Int
// tuple: '(' TypeExpr, TypeExpr, ... ')'
// enum: Uident
// fn: '(' ParamList ')' '->' TypeExpr //todo
fn type_expr(p: &mut Parser) {
    let m = p.open();
    if p.at(TokenKind::UnitKeyword) {
        p.advance();
        p.close(m, MySyntaxKind::TYPE_UNIT);
    } else if p.at(TokenKind::BoolKeyword) {
        p.advance();
        p.close(m, MySyntaxKind::TYPE_BOOL);
    } else if p.at(TokenKind::IntKeyword) {
        p.advance();
        p.close(m, MySyntaxKind::TYPE_INT);
    } else if p.at(TokenKind::LParen) {
        type_list(p);
        p.close(m, MySyntaxKind::TYPE_TUPLE);
    } else if p.at(TokenKind::Uident) {
        p.advance();
        p.close(m, MySyntaxKind::TYPE_ENUM);
    } else {
        p.advance_with_error("expected a type");
    }
}

pub fn block(p: &mut Parser) {
    assert!(p.at(TokenKind::LBrace));
    let m = p.open();
    p.expect(TokenKind::LBrace);
    expr(p);
    p.expect(TokenKind::RBrace);
    p.close(m, MySyntaxKind::BLOCK);
}
