use pretty::RcDoc;

use crate::{
    anf::{AExpr, Arm, CExpr, File, Fn, ImmExpr},
    env::Env,
};

impl File {
    pub fn to_doc(&self, env: &Env) -> RcDoc<()> {
        let items = RcDoc::intersperse(
            self.toplevels.iter().map(|item| item.to_doc(env)),
            RcDoc::hardline().append(RcDoc::hardline()),
        );
        items
    }

    pub fn to_pretty(&self, env: &Env, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc(env).render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl Fn {
    pub fn to_doc(&self, env: &Env) -> RcDoc<()> {
        let name = RcDoc::text(self.name.clone());
        let params = RcDoc::intersperse(
            self.params.iter().map(|(name, ty)| {
                RcDoc::text(name.clone())
                    .append(RcDoc::text(":"))
                    .append(RcDoc::space())
                    .append(ty.to_doc(env))
            }),
            RcDoc::text(", "),
        );

        let ret_ty = self.ret_ty.to_doc(env);

        let body = self.body.to_doc(env);

        RcDoc::text("fn")
            .append(RcDoc::space())
            .append(name)
            .append(RcDoc::text("("))
            .append(params)
            .append(RcDoc::text(")"))
            .append(RcDoc::space())
            .append(RcDoc::text("->"))
            .append(RcDoc::space())
            .append(ret_ty)
            .append(RcDoc::space())
            .append(RcDoc::text("{"))
            .append(RcDoc::hardline().append(body).nest(2))
            .append(RcDoc::hardline())
            .append(RcDoc::text("}"))
    }

    pub fn to_pretty(&self, env: &Env, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc(env).render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl AExpr {
    pub fn to_doc(&self, env: &Env) -> RcDoc<()> {
        match self {
            AExpr::ACExpr { expr } => expr.to_doc(env),
            AExpr::ALet {
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
        }
    }

    pub fn to_pretty(&self, env: &Env, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc(env).render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl CExpr {
    pub fn to_doc(&self, env: &Env) -> RcDoc<()> {
        match self {
            CExpr::CImm { imm } => imm.to_doc(env),
            CExpr::EConstr { index, args, ty } => {
                let prefix =
                    RcDoc::text(env.get_variant_name(&ty.get_constr_name_unsafe(), *index as i32));

                if args.is_empty() {
                    prefix
                } else {
                    let args_doc = RcDoc::intersperse(
                        args.iter().map(|arg| arg.to_doc(env)),
                        RcDoc::text(", "),
                    );

                    prefix
                        .append(RcDoc::text("("))
                        .append(args_doc)
                        .append(RcDoc::text(")"))
                }
            }
            CExpr::ETuple { items, ty: _ } => {
                if items.is_empty() {
                    RcDoc::text("()")
                } else {
                    let items_doc = RcDoc::intersperse(
                        items.iter().map(|item| item.to_doc(env)),
                        RcDoc::text(", "),
                    );

                    RcDoc::text("(").append(items_doc).append(RcDoc::text(")"))
                }
            }
            CExpr::EMatch {
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
                        .append(default_expr.to_doc(env)) // Default case body is AExpr
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
            CExpr::EIf {
                cond,
                then,
                else_,
                ty: _,
            } => RcDoc::text("if")
                .append(RcDoc::space())
                .append(cond.to_doc(env))
                .append(RcDoc::space())
                .append(RcDoc::text("{"))
                .append(RcDoc::hardline().append(then.to_doc(env)).nest(2))
                .append(RcDoc::hardline())
                .append(RcDoc::text("}"))
                .append(RcDoc::space())
                .append(RcDoc::text("else"))
                .append(RcDoc::space())
                .append(RcDoc::text("{"))
                .append(RcDoc::hardline().append(else_.to_doc(env)).nest(2))
                .append(RcDoc::hardline())
                .append(RcDoc::text("}"))
                .group(),
            CExpr::EConstrGet {
                expr,
                variant_index,
                field_index,
                ty: _,
            } => {
                let base = expr.to_doc(env);
                base.append(RcDoc::text(".")).append(RcDoc::text(format!(
                    "constr.{}.{}",
                    variant_index, field_index
                )))
            }
            CExpr::ECall { func, args, ty: _ } => {
                let args_doc =
                    RcDoc::intersperse(args.iter().map(|arg| arg.to_doc(env)), RcDoc::text(", "));

                RcDoc::text(func)
                    .append(RcDoc::text("("))
                    .append(args_doc)
                    .append(RcDoc::text(")"))
            }
            CExpr::EProj {
                tuple,
                index,
                ty: _,
            } => tuple
                .to_doc(env)
                .append(RcDoc::text("."))
                .append(RcDoc::text(index.to_string())),
        }
    }

    pub fn to_pretty(&self, env: &Env, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc(env).render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl ImmExpr {
    pub fn to_doc(&self, _env: &Env) -> RcDoc<()> {
        match self {
            ImmExpr::ImmVar { name, ty: _ } => RcDoc::text(name.clone()),
            ImmExpr::ImmUnit { ty: _ } => RcDoc::text("()"),
            ImmExpr::ImmBool { value, ty: _ } => {
                if *value {
                    RcDoc::text("true")
                } else {
                    RcDoc::text("false")
                }
            }
            ImmExpr::ImmInt { value, ty: _ } => RcDoc::text(value.to_string()),
            ImmExpr::ImmString { value, ty: _ } => RcDoc::text(format!("{:?}", value)),
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
