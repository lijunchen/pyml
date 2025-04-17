use crate::ast;

use ::cst::cst::CstNode;
use cst::cst;
use parser::syntax::MySyntaxNodePtr;

pub fn lower(node: cst::File) -> Option<ast::File> {
    let items = node.items().flat_map(lower_item).collect();
    let ast = ast::File { toplevels: items };
    Some(ast)
}

fn lower_item(node: cst::Item) -> Option<ast::Item> {
    match node {
        cst::Item::Enum(it) => Some(ast::Item::EnumDef(lower_enum(it)?)),
        cst::Item::Fn(it) => Some(ast::Item::Fn(lower_fn(it)?)),
    }
}

fn lower_enum(node: cst::Enum) -> Option<ast::EnumDef> {
    let name = node.uident().unwrap().to_string();
    let variants = node
        .variant_list()
        .unwrap_or_else(|| panic!("Enum {} has no variants", name))
        .variants()
        .flat_map(lower_variant)
        .collect();
    Some(ast::EnumDef {
        name: ast::Uident::new(&name),
        variants,
    })
}

fn lower_variant(node: cst::Variant) -> Option<(ast::Uident, Vec<ast::Ty>)> {
    let name = node.uident().unwrap().to_string();
    let typs = match node.type_list() {
        None => vec![],
        Some(xs) => xs.types().flat_map(lower_ty).collect(),
    };
    Some((ast::Uident::new(&name), typs))
}

fn lower_ty(node: cst::Type) -> Option<ast::Ty> {
    match node {
        cst::Type::UnitTy(_) => Some(ast::Ty::TUnit),
        cst::Type::BoolTy(_) => Some(ast::Ty::TBool),
        cst::Type::IntTy(_) => Some(ast::Ty::TInt),
        cst::Type::TupleTy(it) => {
            let typs = it.type_list()?.types().flat_map(lower_ty).collect();
            return Some(ast::Ty::TTuple { typs });
        }
        cst::Type::EnumTy(it) => Some(ast::Ty::TEnum {
            name: ast::Uident::new(&it.uident().unwrap().to_string()),
        }),
        cst::Type::FuncTy(..) => todo!(),
    }
}

fn lower_fn(node: cst::Fn) -> Option<ast::Fn> {
    let name = node.lident().unwrap().to_string();
    let params = node
        .param_list()
        .unwrap_or_else(|| panic!("Fn {} has no params", name))
        .params()
        .flat_map(lower_param)
        .collect();
    let ret_ty = node.return_type().and_then(lower_ty);
    let body = node
        .block()
        .and_then(lower_block)
        .unwrap_or_else(|| panic!("Fn {} has no body", name));
    Some(ast::Fn {
        name: ast::Lident(name),
        params,
        ret_ty,
        body,
    })
}

fn lower_block(node: cst::Block) -> Option<ast::Expr> {
    let cst_e = node.expr();

    cst_e.and_then(lower_expr)
}

fn lower_param(node: cst::Param) -> Option<(ast::Lident, ast::Ty)> {
    let name = node.lident().unwrap().to_string();
    let ty = node
        .ty()
        .and_then(lower_ty)
        .unwrap_or_else(|| panic!("Param {} has no type", name));
    Some((ast::Lident(name), ty))
}

fn lower_expr(node: cst::Expr) -> Option<ast::Expr> {
    match node {
        cst::Expr::UnitExpr(_) => Some(ast::Expr::EUnit),
        cst::Expr::BoolExpr(it) => {
            let value = it
                .value()
                .unwrap_or_else(|| panic!("BoolExpr has no value"))
                .to_string();
            let value = match value.as_str() {
                "true" => Some(ast::Expr::EBool { value: true }),
                "false" => Some(ast::Expr::EBool { value: false }),
                _ => unreachable!(),
            };
            value
        }
        cst::Expr::IntExpr(it) => {
            let value = it
                .value()
                .unwrap_or_else(|| panic!("IntExpr has no value"))
                .to_string();
            let value = value.parse::<i32>().ok()?;
            Some(ast::Expr::EInt { value })
        }
        cst::Expr::StrExpr(it) => {
            let value = it
                .value()
                .unwrap_or_else(|| panic!("StrExpr has no value"))
                .to_string();
            // parse string literal
            let value = value
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .unwrap_or_else(|| panic!("StrExpr has no value"))
                .to_string();
            Some(ast::Expr::EString { value })
        }
        cst::Expr::CallExpr(it) => {
            let l = it.l_name();
            let u = it.u_name();

            if l.is_some() {
                let func = l.unwrap().to_string();
                let args = it
                    .arg_list()
                    .unwrap_or_else(|| panic!("PrimExpr has no args"))
                    .args()
                    .flat_map(lower_arg)
                    .collect();
                return Some(ast::Expr::ECall {
                    func: ast::Lident(func),
                    args,
                });
            }

            if u.is_some() {
                let func = u.unwrap().to_string();
                let args = it
                    .arg_list()
                    .unwrap_or_else(|| panic!("PrimExpr has no args"))
                    .args()
                    .flat_map(lower_arg)
                    .collect();
                return Some(ast::Expr::EConstr {
                    vcon: ast::Uident::new(&func),
                    args,
                });
            }

            unreachable!()
        }
        cst::Expr::MatchExpr(it) => {
            let expr = it
                .expr()
                .and_then(lower_expr)
                .unwrap_or_else(|| panic!("MatchExpr has no expr"));
            let arms = it
                .match_arm_list()
                .unwrap_or_else(|| panic!("MatchExpr has no arms"))
                .arms()
                .flat_map(lower_arm)
                .collect();
            Some(ast::Expr::EMatch {
                expr: Box::new(expr),
                arms,
            })
        }
        cst::Expr::UidentExpr(it) => {
            let name = it.uident().unwrap().to_string();
            Some(ast::Expr::EConstr {
                vcon: ast::Uident::new(&name),
                args: vec![],
            })
        }
        cst::Expr::LidentExpr(it) => {
            let name = it.lident_token().unwrap().to_string();
            Some(ast::Expr::EVar {
                name: ast::Lident(name),
                astptr: MySyntaxNodePtr::new(it.syntax()),
            })
        }
        cst::Expr::TupleExpr(it) => {
            let items = it.exprs().flat_map(lower_expr).collect();
            Some(ast::Expr::ETuple { items })
        }
        cst::Expr::LetExpr(it) => {
            let pat = it
                .pattern()
                .and_then(lower_pat)
                .unwrap_or_else(|| panic!("LetExpr has no pattern"));
            let value = it
                .value()
                .unwrap_or_else(|| panic!("LetExpr has no value"))
                .expr()
                .and_then(lower_expr)
                .unwrap_or_else(|| panic!("failed to lower value {:#?}", it.value()));
            let body = it
                .body()
                .unwrap_or_else(|| panic!("LetExpr has no body"))
                .expr()
                .and_then(lower_expr)
                .unwrap_or_else(|| panic!("failed to lower body"));
            Some(ast::Expr::ELet {
                pat,
                value: Box::new(value),
                body: Box::new(body),
            })
        }
        cst::Expr::BinaryExpr(it) => {
            let exprs: Vec<ast::Expr> = it.exprs().flat_map(lower_expr).collect();
            let op = it.op().unwrap().to_string();
            match op.as_str() {
                "." => {
                    // FIXME: no clone
                    let lhs = exprs[0].clone();
                    let rhs = exprs[1].clone();
                    Some(ast::Expr::EProj {
                        tuple: Box::new(lhs),
                        index: Box::new(rhs),
                    })
                }
                _ => panic!("BinaryExpr has no op"),
            }
        }
    }
}

fn lower_arg(node: cst::Arg) -> Option<ast::Expr> {
    lower_expr(node.expr().unwrap_or_else(|| panic!("Arg has no expr")))
}

fn lower_arm(node: cst::MatchArm) -> Option<ast::Arm> {
    let pat = node.pattern().and_then(lower_pat)?;
    let expr = node.expr().and_then(lower_expr)?;
    Some(ast::Arm { pat, body: expr })
}

fn lower_pat(node: cst::Pattern) -> Option<ast::Pat> {
    match node {
        cst::Pattern::VarPat(it) => {
            let name = it.lident().unwrap().to_string();
            Some(ast::Pat::PVar {
                name: ast::Lident(name),
                astptr: MySyntaxNodePtr::new(it.syntax()),
            })
        }
        cst::Pattern::UnitPat(_) => Some(ast::Pat::PUnit),
        cst::Pattern::BoolPat(it) => {
            let value = it.value()?.to_string();
            let value = match value.as_str() {
                "true" => Some(ast::Pat::PBool { value: true }),
                "false" => Some(ast::Pat::PBool { value: false }),
                _ => unreachable!(),
            };
            value
        }
        cst::Pattern::ConstrPat(it) => {
            let name = it.uident().unwrap().to_string();
            let pats = it.patterns().flat_map(lower_pat).collect();
            Some(ast::Pat::PConstr {
                vcon: ast::Uident::new(&name),
                args: pats,
            })
        }
        cst::Pattern::TuplePat(it) => {
            let items = it.patterns().flat_map(lower_pat).collect();
            Some(ast::Pat::PTuple { pats: items })
        }
        cst::Pattern::WildPat(_) => Some(ast::Pat::PWild),
    }
}
