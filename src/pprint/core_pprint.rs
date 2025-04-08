use pretty::RcDoc;

use crate::{
    core::{Arm, Expr},
    env::Env,
};

impl Expr {
    pub fn to_doc(&self, env: &Env) -> RcDoc<()> {
        match self {
            Expr::EVar { name, ty: _ } => RcDoc::text(name.clone()),

            Expr::EUnit { ty: _ } => RcDoc::text("()"),

            Expr::EBool { value, ty: _ } => {
                if *value {
                    RcDoc::text("true")
                } else {
                    RcDoc::text("false")
                }
            }
            Expr::EInt { value, ty: _ } => RcDoc::text(value.to_string()),
            Expr::EConstr { index, args, ty } => {
                let prefix =
                    RcDoc::text(env.get_variant_name(&ty.get_constr_name_unsafe(), *index as i32));

                if args.is_empty() {
                    prefix
                } else {
                    let args_doc = RcDoc::intersperse(
                        args.iter().map(|arg| arg.to_doc(env)),
                        RcDoc::text(","),
                    );

                    prefix
                        .append(RcDoc::text("("))
                        .append(args_doc)
                        .append(RcDoc::text(")"))
                }
            }

            Expr::ETuple { items, ty: _ } => {
                if items.is_empty() {
                    RcDoc::text("()")
                } else {
                    let items_doc = RcDoc::intersperse(
                        items.iter().map(|item| item.to_doc(env)),
                        RcDoc::text(","),
                    );

                    RcDoc::text("(").append(items_doc).append(RcDoc::text(")"))
                }
            }

            Expr::ELet {
                name,
                value,
                body,
                ty: _,
            } => RcDoc::text("let")
                .append(RcDoc::space())
                .append(RcDoc::text(name))
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(value.to_doc(env))
                .append(RcDoc::space())
                .append(RcDoc::text("in"))
                .append(RcDoc::hardline())
                .append(body.to_doc(env))
                .group(),

            Expr::EMatch {
                expr,
                arms,
                default,
                ty: _,
            } => {
                let match_expr = RcDoc::text("match")
                    .append(RcDoc::space())
                    .append(expr.to_doc(env))
                    .append(RcDoc::space())
                    .append(RcDoc::text("{"));

                let arms_doc = RcDoc::concat(
                    arms.iter()
                        .map(|arm| RcDoc::hardline().append(arm.to_doc(env))),
                );

                let default_doc = if let Some(default_expr) = default {
                    RcDoc::hardline()
                        .append(RcDoc::text("_"))
                        .append(RcDoc::space())
                        .append(RcDoc::text("=>"))
                        .append(RcDoc::space())
                        .append(default_expr.to_doc(env))
                        .append(RcDoc::text(","))
                } else {
                    RcDoc::nil()
                };

                match_expr
                    .append(arms_doc.nest(2))
                    .append(default_doc.nest(2))
                    .append(RcDoc::line())
                    .append(RcDoc::text("}"))
                    .group()
            }

            Expr::EPrim { func, args, ty: _ } => {
                let args_doc =
                    RcDoc::intersperse(args.iter().map(|arg| arg.to_doc(env)), RcDoc::text(","));

                RcDoc::text(func)
                    .append(RcDoc::text("("))
                    .append(args_doc)
                    .append(RcDoc::text(")"))
            }
            Expr::EProj {
                tuple,
                index,
                ty: _,
            } => tuple
                .to_doc(env)
                .append(RcDoc::text("."))
                .append(RcDoc::text(index.to_string())),
            Expr::EConstrGet {
                expr,
                variant_index,
                field_index,
                ty: _,
            } => RcDoc::text("variant_field")
                .append(RcDoc::text("("))
                .append(expr.to_doc(env))
                .append(RcDoc::text(","))
                .append(RcDoc::text(variant_index.to_string()))
                .append(RcDoc::text(","))
                .append(RcDoc::text(field_index.to_string()))
                .append(RcDoc::text(")")),
        }
    }

    pub fn to_pretty(&self, env: &Env, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc(env).render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl Arm {
    pub fn to_doc(&self, env: &Env) -> RcDoc<()> {
        self.lhs
            .to_doc(env)
            .append(RcDoc::space())
            .append(RcDoc::text("=>"))
            .append(RcDoc::space())
            .append(RcDoc::text("{"))
            .append(
                RcDoc::hardline()
                    .append(self.body.to_doc(env))
                    .append(RcDoc::hardline())
                    .nest(2),
            )
            .append(RcDoc::text("}"))
            .append(RcDoc::text(","))
    }

    pub fn to_pretty(&self, env: &Env, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc(env).render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}
