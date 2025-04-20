use cst::cst::CstNode;
use parser::syntax::{MySyntaxKind, MySyntaxNode};

use crate::{env::Env, tast};

pub fn hover_type(src: &str, line: u32, col: u32) -> Option<String> {
    let result = parser::parse(&std::path::PathBuf::from("dummy"), src);
    let root = MySyntaxNode::new_root(result.green_node);
    let cst = cst::cst::File::cast(root).unwrap();

    let line_index = line_index::LineIndex::new(src);
    let offset = line_index
        .offset(line_index::LineCol { line, col })
        .unwrap();
    let node = cst.syntax().token_at_offset(offset);
    let range = match node {
        rowan::TokenAtOffset::None => None,
        rowan::TokenAtOffset::Single(x) => {
            if x.kind() == MySyntaxKind::Lident {
                Some(x.text_range())
            } else {
                None
            }
        }
        rowan::TokenAtOffset::Between(x, y) => {
            if x.kind() == MySyntaxKind::Lident {
                Some(x.text_range())
            } else if y.kind() == MySyntaxKind::Lident {
                Some(y.text_range())
            } else {
                None
            }
        }
    }?;

    let ast = ast::lower::lower(cst).unwrap();
    let (tast, env) = crate::typer::check_file(ast);

    let ty = find_type(&env, &tast, &range);

    ty.map(|node| node.to_string())
}

fn find_type(env: &Env, tast: &tast::File, range: &rowan::TextRange) -> Option<String> {
    for item in &tast.toplevels {
        match item {
            tast::Item::ImplBlock(impl_block) => {
                for item in impl_block.methods.iter() {
                    if let Some(t) = find_type_fn(env, item, range) {
                        return Some(t.clone());
                    }
                }
            }
            tast::Item::Fn(f) => {
                if let Some(t) = find_type_fn(env, f, range) {
                    return Some(t.clone());
                }
            }
        }
    }
    None
}

fn find_type_fn(env: &Env, tast: &tast::Fn, range: &rowan::TextRange) -> Option<String> {
    find_type_expr(env, &tast.body, range)
}

fn find_type_expr(env: &Env, tast: &tast::Expr, range: &rowan::TextRange) -> Option<String> {
    match tast {
        tast::Expr::EVar {
            name: _,
            ty: _,
            astptr,
        } => {
            if astptr.unwrap().text_range().contains_range(*range) {
                return Some(tast.get_ty().to_pretty(env, 80));
            }
            None
        }
        tast::Expr::EUnit { ty: _ } => None,
        tast::Expr::EBool { value: _, ty: _ } => None,
        tast::Expr::EInt { value: _, ty: _ } => None,
        tast::Expr::EString { value: _, ty: _ } => None,
        tast::Expr::EConstr {
            index: _,
            args: _,
            ty: _,
        } => None,
        tast::Expr::ETuple { items, ty: _ } => {
            for item in items {
                if let Some(expr) = find_type_expr(env, item, range) {
                    return Some(expr);
                }
            }
            None
        }
        tast::Expr::ELet {
            pat,
            value,
            body,
            ty: _,
        } => {
            if let Some(expr) = find_type_pat(env, pat, range) {
                return Some(expr);
            }
            if let Some(expr) = find_type_expr(env, value, range) {
                return Some(expr);
            }
            find_type_expr(env, body, range)
        }
        tast::Expr::EMatch { expr, arms, ty: _ } => {
            if let Some(expr) = find_type_expr(env, expr, range) {
                return Some(expr);
            }
            for arm in arms {
                if let Some(expr) = find_type_pat(env, &arm.pat, range) {
                    return Some(expr);
                }
                if let Some(expr) = find_type_expr(env, &arm.body, range) {
                    return Some(expr);
                }
            }
            None
        }
        tast::Expr::ECall {
            func: _,
            args,
            ty: _,
        } => {
            for arg in args {
                if let Some(expr) = find_type_expr(env, arg, range) {
                    return Some(expr);
                }
            }
            None
        }
        tast::Expr::EProj {
            tuple,
            index: _,
            ty: _,
        } => {
            if let Some(expr) = find_type_expr(env, tuple, range) {
                return Some(expr);
            }
            None
        }
    }
}

fn find_type_pat(env: &Env, tast: &tast::Pat, range: &rowan::TextRange) -> Option<String> {
    match tast {
        tast::Pat::PVar {
            name: _,
            ty: _,
            astptr,
        } => {
            if astptr.unwrap().text_range().contains_range(*range) {
                return Some(tast.get_ty().to_pretty(env, 80));
            }
            None
        }
        tast::Pat::PUnit { ty: _ } => None,
        tast::Pat::PBool { value: _, ty: _ } => None,
        tast::Pat::PConstr {
            index: _,
            args,
            ty: _,
        } => {
            for arg in args {
                if let Some(expr) = find_type_pat(env, arg, range) {
                    return Some(expr);
                }
            }
            None
        }
        tast::Pat::PTuple { items, ty: _ } => {
            for item in items {
                if let Some(expr) = find_type_pat(env, item, range) {
                    return Some(expr);
                }
            }
            None
        }
        tast::Pat::PWild { ty: _ } => None,
    }
}
