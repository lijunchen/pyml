use lexer::TokenKind;

use crate::{
    expr::{EXPR_FIRST, expr},
    parser::Parser,
    syntax::SyntaxKind,
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
    p.close(m, SyntaxKind::File);
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
    p.close(m, SyntaxKind::Fn);
}

fn enum_def(p: &mut Parser) {
    assert!(p.at(TokenKind::EnumKeyword));
    let m = p.open();
    p.expect(TokenKind::EnumKeyword);
    p.expect(TokenKind::Uident);
    if p.at(TokenKind::LBrace) {
        variant_list(p);
    }
    p.close(m, SyntaxKind::EnumDef);
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
    p.close(m, SyntaxKind::VariantList);
}

fn variant(p: &mut Parser) {
    assert!(p.at(TokenKind::Uident));
    let m = p.open();
    p.expect(TokenKind::Uident);
    if p.at(TokenKind::LParen) {
        type_list(p);
    }
    p.close(m, SyntaxKind::Variant);
}

fn type_list(p: &mut Parser) {
    assert!(p.at(TokenKind::LParen));
    let m = p.open();
    p.expect(TokenKind::LParen);
    while !p.at(TokenKind::RParen) && !p.eof() {
        if p.at(TokenKind::Uident) {
            type_expr(p);
            p.eat(TokenKind::Comma);
        } else {
            p.advance_with_error("expected a type");
        }
    }
    p.close(m, SyntaxKind::TypeList);
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
    p.close(m, SyntaxKind::ParamList);
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
    p.close(m, SyntaxKind::Param);
}

// TypeExpr = 'name'
fn type_expr(p: &mut Parser) {
    let m = p.open();
    p.expect(TokenKind::Uident);
    p.close(m, SyntaxKind::TypeExpr);
}

// Block = '{' Stmt* '}'
// Stmt = StmtLet | StmtReturn | StmtExpr

pub fn block(p: &mut Parser) {
    assert!(p.at(TokenKind::LBrace));
    let m = p.open();
    expr(p);
    p.expect(TokenKind::RBrace);
    p.close(m, SyntaxKind::Block);
}
