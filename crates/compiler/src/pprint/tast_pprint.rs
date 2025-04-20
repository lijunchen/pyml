use pretty::RcDoc;

use crate::env::Env;
use crate::tast::Expr;
use crate::tast::File;
use crate::tast::Fn;
use crate::tast::ImplBlock;
use crate::tast::Item;
use crate::tast::Pat;
use crate::tast::Ty;

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

impl Item {
    pub fn to_doc(&self, env: &Env) -> RcDoc<()> {
        match self {
            Self::ImplBlock(impl_block) => impl_block.to_doc(env),
            Self::Fn(func) => func.to_doc(env),
        }
    }

    pub fn to_pretty(&self, env: &Env, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc(env).render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl ImplBlock {
    pub fn to_doc(&self, env: &Env) -> RcDoc<()> {
        let trait_name = RcDoc::text(self.trait_name.0.clone());
        let for_type = self.for_type.to_doc(env);
        let methods = RcDoc::intersperse(
            self.methods.iter().map(|method| method.to_doc(env)),
            RcDoc::hardline(),
        );

        RcDoc::text("impl")
            .append(RcDoc::space())
            .append(trait_name)
            .append(RcDoc::space())
            .append(RcDoc::text("for"))
            .append(RcDoc::space())
            .append(for_type)
            .append(RcDoc::text("{"))
            .append(RcDoc::hardline().append(methods).nest(2))
            .append(RcDoc::hardline())
            .append(RcDoc::text("}"))
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

impl Ty {
    pub fn to_doc(&self, _env: &Env) -> RcDoc<()> {
        match self {
            Self::TVar(x) => RcDoc::text(format!("{:?}", x)),
            Self::TUnit => RcDoc::text("Unit"),
            Self::TBool => RcDoc::text("Bool"),
            Self::TInt => RcDoc::text("Int"),
            Self::TString => RcDoc::text("String"),
            Self::TTuple { typs } => {
                let mut doc = RcDoc::text("(");

                if !typs.is_empty() {
                    let mut iter = typs.iter();
                    if let Some(first) = iter.next() {
                        doc = doc.append(first.to_doc(_env));
                    }
                    for item in iter {
                        doc = doc.append(RcDoc::text(", ")).append(item.to_doc(_env));
                    }
                }

                doc.append(RcDoc::text(")"))
            }
            Self::TApp { name, args } => {
                let mut doc = RcDoc::text(name.0.clone());

                if !args.is_empty() {
                    let mut iter = args.iter();
                    if let Some(first) = iter.next() {
                        doc = doc.append(RcDoc::text("[")).append(first.to_doc(_env));
                    }
                    for item in iter {
                        doc = doc.append(RcDoc::text(", ")).append(item.to_doc(_env));
                    }
                    doc = doc.append(RcDoc::text("]"));
                }

                doc
            }
            Self::TFunc { params, ret_ty } => {
                let mut doc = RcDoc::text("(");

                if !params.is_empty() {
                    let mut iter = params.iter();
                    if let Some(first) = iter.next() {
                        doc = doc.append(first.to_doc(_env));
                    }
                    for item in iter {
                        doc = doc.append(RcDoc::text(", ")).append(item.to_doc(_env));
                    }
                }

                doc.append(RcDoc::text(") -> ")).append(ret_ty.to_doc(_env))
            }
            Self::TParam { name } => RcDoc::text(name.clone()),
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
            Self::EVar {
                name,
                ty,
                astptr: _,
            } => RcDoc::text("(")
                .append(RcDoc::text(name.clone()))
                .append(RcDoc::text(" : "))
                .append(ty.to_doc(env))
                .append(RcDoc::text(")")),

            Self::EUnit { ty: _ } => RcDoc::text("()"),

            Self::EBool { value, ty: _ } => {
                if *value {
                    RcDoc::text("true")
                } else {
                    RcDoc::text("false")
                }
            }
            Self::EInt { value, ty: _ } => RcDoc::text(value.to_string()),
            Self::EString { value, ty: _ } => RcDoc::text(format!("{:?}", value)),
            Expr::EConstr { index, args, ty } => {
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

            Self::ETuple { items, ty: _ } => {
                if items.is_empty() {
                    RcDoc::text("()")
                } else {
                    let items_doc = RcDoc::intersperse(
                        items.iter().map(|item| item.to_doc(env)),
                        RcDoc::text(", "),
                    );

                    RcDoc::text("(")
                        .append(items_doc)
                        .nest(2)
                        .append(RcDoc::text(")"))
                        .group()
                }
            }

            Self::ELet {
                pat,
                value,
                body,
                ty: _,
            } => RcDoc::text("let")
                .append(RcDoc::space())
                .append(pat.to_doc(env))
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(value.to_doc(env))
                .append(RcDoc::space())
                .append(RcDoc::text("in"))
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

            Self::ECall { func, args, ty: _ } => {
                let args_doc =
                    RcDoc::intersperse(args.iter().map(|arg| arg.to_doc(env)), RcDoc::text(", "));

                RcDoc::text(func)
                    .append(RcDoc::text("("))
                    .append(args_doc)
                    .append(RcDoc::text(")"))
            }
            Self::EProj {
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

impl Pat {
    pub fn to_doc(&self, env: &Env) -> RcDoc<()> {
        match self {
            Pat::PVar {
                name,
                ty,
                astptr: _,
            } => RcDoc::text(name.clone())
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(ty.to_doc(env)),
            Pat::PUnit { .. } => RcDoc::text("()"),
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
                        RcDoc::text(", "),
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
                        RcDoc::text(", "),
                    );
                    RcDoc::text("(").append(items_doc).append(RcDoc::text(")"))
                }
            }
            Pat::PWild { ty } => RcDoc::text("_")
                .append(RcDoc::text(" : "))
                .append(ty.to_doc(env)),
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
