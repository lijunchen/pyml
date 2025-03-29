use pretty::RcDoc;

use crate::env::Env;
use crate::tast::Expr;
use crate::tast::Pat;
use crate::tast::Ty;

impl Ty {
    pub fn to_doc(&self, env: &Env) -> RcDoc<()> {
        match self {
            Self::TUnit => RcDoc::text("()"),
            Self::TBool => RcDoc::text("bool"),
            Self::TTuple { typs } => {
                let mut doc = RcDoc::text("(");

                if !typs.is_empty() {
                    let mut iter = typs.iter();
                    if let Some(first) = iter.next() {
                        doc = doc.append(first.to_doc(env));
                    }
                    for item in iter {
                        doc = doc.append(RcDoc::text(", ")).append(item.to_doc(env));
                    }
                }

                doc.append(RcDoc::text(")"))
            }
            Self::TConstr { name } => RcDoc::text(name),
        }
    }

    pub fn to_pretty(&self, env: &Env, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc(env).render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl Expr {
    pub fn to_doc(&self, env: &Env) -> RcDoc<()> {
        match self {
            Self::EVar { name, ty: _ } => RcDoc::text(name.clone()),

            Self::EUnit { ty: _ } => RcDoc::text("()"),

            Self::EBool { value, ty: _ } => {
                if *value {
                    RcDoc::text("true")
                } else {
                    RcDoc::text("false")
                }
            }

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

            Self::ETuple { items, ty: _ } => {
                if items.is_empty() {
                    RcDoc::text("()")
                } else {
                    let items_doc = RcDoc::intersperse(
                        items.iter().map(|item| item.to_doc(env)),
                        RcDoc::text(",")
                            .append(RcDoc::line())
                            .append(RcDoc::space()),
                    );

                    RcDoc::text("(")
                        .append(RcDoc::softline())
                        .append(items_doc)
                        .nest(2)
                        .append(RcDoc::softline())
                        .append(RcDoc::text(")"))
                        .group()
                }
            }

            Self::ELet {
                name,
                value,
                body,
                ty: _,
            } => RcDoc::text("let")
                .append(RcDoc::space())
                .append(name.to_doc(env))
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(value.to_doc(env))
                .append(RcDoc::text(";"))
                .append(RcDoc::hardline())
                .append(body.to_doc(env))
                .group(),

            Self::EMatch { expr, arms, ty: _ } => {
                let match_expr = RcDoc::text("match")
                    .append(RcDoc::space())
                    .append(expr.to_doc(env))
                    .append(RcDoc::space())
                    .append(RcDoc::text("{"));

                let arms_doc = RcDoc::concat(
                    arms.iter()
                        .map(|arm| RcDoc::hardline().append(arm.to_doc(env))),
                );

                match_expr
                    .append(arms_doc.nest(4))
                    .append(RcDoc::hardline())
                    .append(RcDoc::text("}"))
                    .group()
            }

            Self::EPrim { func, args, ty: _ } => {
                if args.is_empty() {
                    RcDoc::text(func).append(RcDoc::text("()"))
                } else {
                    let args_doc = RcDoc::intersperse(
                        args.iter().map(|arg| arg.to_doc(env)),
                        RcDoc::text(",")
                            .append(RcDoc::line())
                            .append(RcDoc::space()),
                    );

                    RcDoc::text(func)
                        .append(RcDoc::text("("))
                        .append(RcDoc::softline())
                        .append(args_doc)
                        .nest(2)
                        .append(RcDoc::softline())
                        .append(RcDoc::text(")"))
                        .group()
                }
            }
        }
    }

    pub fn to_pretty(&self, env: &Env, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc(env).render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl Pat {
    pub fn to_doc(&self, env: &Env) -> RcDoc<()> {
        match self {
            Pat::PVar { name, ty: _ } => RcDoc::text(name.clone()),
            Pat::PUnit => RcDoc::text("()"),
            Pat::PBool { value, ty: _ } => {
                if *value {
                    RcDoc::text("true")
                } else {
                    RcDoc::text("false")
                }
            }
            Pat::PConstr { index, args, ty } => {
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
            Pat::PTuple { items, ty: _ } => {
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
            Pat::PWild { ty: _ } => RcDoc::text("_"),
        }
    }
    pub fn to_pretty(&self, env: &Env, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc(env).render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl crate::tast::Arm {
    pub fn to_doc(&self, env: &Env) -> RcDoc<()> {
        self.pat
            .to_doc(env)
            .append(RcDoc::space())
            .append(RcDoc::text("=>"))
            .append(RcDoc::space())
            .append(
                self.body.to_doc(env).nest(2), // Properly indent the body of the arm
            )
            .append(RcDoc::text(","))
    }

    pub fn to_pretty(&self, env: &Env, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc(env).render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}
