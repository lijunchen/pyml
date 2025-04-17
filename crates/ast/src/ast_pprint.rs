use crate::ast::{Arm, EnumDef, Expr, File, Fn, Item, Pat, Ty};
use pretty::RcDoc;

impl Ty {
    pub fn to_doc(&self) -> RcDoc<()> {
        match self {
            Self::TUnit => RcDoc::text("Unit"),
            Self::TBool => RcDoc::text("Bool"),
            Self::TInt => RcDoc::text("Int"),
            Self::TTuple { typs } => {
                let mut doc = RcDoc::text("(");

                if !typs.is_empty() {
                    let mut iter = typs.iter();
                    if let Some(first) = iter.next() {
                        doc = doc.append(first.to_doc());
                    }
                    for item in iter {
                        doc = doc.append(RcDoc::text(", ")).append(item.to_doc());
                    }
                }

                doc.append(RcDoc::text(")"))
            }
            Self::TEnum { name } => RcDoc::text(name.0.clone()),
            Self::TFunc { params, ret_ty } => {
                let mut doc = RcDoc::text("(");

                if !params.is_empty() {
                    let mut iter = params.iter();
                    if let Some(first) = iter.next() {
                        doc = doc.append(first.to_doc());
                    }
                    for item in iter {
                        doc = doc.append(RcDoc::text(", ")).append(item.to_doc());
                    }
                }

                doc.append(RcDoc::text(")"))
                    .append(RcDoc::text(" -> "))
                    .append(ret_ty.to_doc())
            }
        }
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl Expr {
    pub fn to_doc(&self) -> RcDoc<()> {
        match self {
            Self::EVar { name, astptr: _ } => RcDoc::text(name.0.clone()),

            Self::EUnit => RcDoc::text("()"),

            Self::EBool { value } => {
                if *value {
                    RcDoc::text("true")
                } else {
                    RcDoc::text("false")
                }
            }

            Self::EInt { value } => RcDoc::text(value.to_string()),
            Self::EString { value } => RcDoc::text(format!("{:?}", value)),

            Self::EConstr { vcon, args } => {
                let prefix = RcDoc::text(vcon.0.clone());

                if args.is_empty() {
                    prefix
                } else {
                    let args_doc =
                        RcDoc::intersperse(args.iter().map(|arg| arg.to_doc()), RcDoc::text(", "));

                    prefix
                        .append(RcDoc::text("("))
                        .append(args_doc)
                        .append(RcDoc::text(")"))
                }
            }

            Self::ETuple { items } => {
                if items.is_empty() {
                    RcDoc::text("()")
                } else {
                    let items_doc = RcDoc::intersperse(
                        items.iter().map(|item| item.to_doc()),
                        RcDoc::text(", "),
                    );

                    RcDoc::text("(").append(items_doc).append(RcDoc::text(")"))
                }
            }

            Self::ELet { pat, value, body } => RcDoc::text("let")
                .append(RcDoc::space())
                .append(pat.to_doc())
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(value.to_doc())
                .append(RcDoc::space())
                .append(RcDoc::text("in"))
                .append(RcDoc::hardline())
                .append(body.to_doc())
                .group(),

            Self::EMatch { expr, arms } => {
                let match_expr = RcDoc::text("match")
                    .append(RcDoc::space())
                    .append(expr.to_doc())
                    .append(RcDoc::space())
                    .append(RcDoc::text("{"));

                let arms_doc = RcDoc::concat(
                    arms.iter()
                        .map(|arm| RcDoc::hardline().append(arm.to_doc())),
                );

                match_expr
                    .append(arms_doc.nest(4))
                    .append(RcDoc::hardline())
                    .append(RcDoc::text("}"))
                    .group()
            }

            Self::ECall { func, args } => {
                if args.is_empty() {
                    RcDoc::text(func.0.clone()).append(RcDoc::text("()"))
                } else {
                    let args_doc =
                        RcDoc::intersperse(args.iter().map(|arg| arg.to_doc()), RcDoc::text(", "));

                    RcDoc::text(func.0.clone())
                        .append(RcDoc::text("("))
                        .append(args_doc)
                        .nest(2)
                        .append(RcDoc::text(")"))
                        .group()
                }
            }
            Self::EProj { tuple, index } => tuple
                .to_doc()
                .append(RcDoc::text("."))
                .append(index.to_doc()),
        }
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl Pat {
    pub fn to_doc(&self) -> RcDoc<()> {
        match self {
            Pat::PVar { name, astptr: _ } => RcDoc::text(name.0.clone()),
            Pat::PUnit => RcDoc::text("()"),
            Pat::PBool { value } => {
                if *value {
                    RcDoc::text("true")
                } else {
                    RcDoc::text("false")
                }
            }
            Pat::PConstr { vcon, args } => {
                let prefix = RcDoc::text(vcon.0.clone());

                if args.is_empty() {
                    prefix
                } else {
                    let args_doc =
                        RcDoc::intersperse(args.iter().map(|arg| arg.to_doc()), RcDoc::text(", "));
                    prefix
                        .append(RcDoc::text("("))
                        .append(args_doc)
                        .append(RcDoc::text(")"))
                }
            }
            Pat::PTuple { pats } => {
                if pats.is_empty() {
                    RcDoc::text("()")
                } else {
                    let items_doc = RcDoc::intersperse(
                        pats.iter().map(|item| item.to_doc()),
                        RcDoc::text(", "),
                    );
                    RcDoc::text("(").append(items_doc).append(RcDoc::text(")"))
                }
            }
            Pat::PWild => RcDoc::text("_"),
        }
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl Arm {
    pub fn to_doc(&self) -> RcDoc<()> {
        self.pat
            .to_doc()
            .append(RcDoc::space())
            .append(RcDoc::text("=>"))
            .append(RcDoc::space())
            .append(
                self.body.to_doc().nest(2), // Properly indent the body of the arm
            )
            .append(RcDoc::text(","))
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl EnumDef {
    pub fn to_doc(&self) -> RcDoc<()> {
        let header = RcDoc::text("enum")
            .append(RcDoc::space())
            .append(RcDoc::text(&self.name.0));

        let variants_doc =
            RcDoc::concat(self.variants.iter().enumerate().map(|(i, (name, types))| {
                let variant = if i == 0 {
                    RcDoc::hardline()
                } else {
                    RcDoc::text(",").append(RcDoc::hardline())
                }
                .append(RcDoc::text(&name.0));

                if types.is_empty() {
                    variant
                } else {
                    let types_doc =
                        RcDoc::intersperse(types.iter().map(|ty| ty.to_doc()), RcDoc::text(", "));

                    variant
                        .append(RcDoc::text("("))
                        .append(types_doc)
                        .append(RcDoc::text(")"))
                }
            }));

        header
            .append(RcDoc::space())
            .append(RcDoc::text("{"))
            .append(variants_doc.nest(4))
            .append(RcDoc::hardline())
            .append(RcDoc::text("}"))
            .group()
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl Fn {
    pub fn to_doc(&self) -> RcDoc<()> {
        let header = RcDoc::text("fn")
            .append(RcDoc::space())
            .append(RcDoc::text(&self.name.0))
            .append(RcDoc::text("("));

        let params_doc = RcDoc::intersperse(
            self.params.iter().map(|(name, ty)| {
                RcDoc::text(name.0.clone())
                    .append(RcDoc::text(":"))
                    .append(ty.to_doc())
            }),
            RcDoc::text(", "),
        );

        let ret_ty_doc = if let Some(ret_ty) = &self.ret_ty {
            RcDoc::text(" -> ").append(ret_ty.to_doc())
        } else {
            RcDoc::nil()
        };

        header
            .append(params_doc)
            .append(RcDoc::text(")"))
            .append(ret_ty_doc)
            .append(RcDoc::space())
            .append(RcDoc::text("{"))
            .append(RcDoc::hardline().append(self.body.to_doc()).nest(4))
            .append(RcDoc::hardline())
            .append(RcDoc::text("}"))
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl File {
    pub fn to_doc(&self) -> RcDoc<()> {
        RcDoc::concat(self.toplevels.iter().map(|item| {
            item.to_doc()
                .append(RcDoc::hardline())
                .append(RcDoc::hardline())
        }))
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl Item {
    pub fn to_doc(&self) -> RcDoc<()> {
        match self {
            Item::EnumDef(def) => def.to_doc(),
            Item::Fn(func) => func.to_doc(),
        }
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}
