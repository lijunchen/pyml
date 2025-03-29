use pretty::RcDoc;

use crate::tast::Expr;
use crate::tast::Pat;
use crate::tast::Ty;

impl Ty {
    pub fn to_doc(&self) -> RcDoc<()> {
        match self {
            Self::TUnit => RcDoc::text("()"),
            Self::TBool => RcDoc::text("bool"),
            Self::TColor => RcDoc::text("Color"),
            Self::TTuple(items) => {
                let mut doc = RcDoc::text("(");

                if !items.is_empty() {
                    let mut iter = items.iter();
                    if let Some(first) = iter.next() {
                        doc = doc.append(first.to_doc());
                    }
                    for item in iter {
                        doc = doc.append(RcDoc::text(", ")).append(item.to_doc());
                    }
                }

                doc.append(RcDoc::text(")"))
            }
            Self::TConstr { name } => RcDoc::text(name),
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
            Self::EVar { name, ty: _ } => RcDoc::text(name.clone()),

            Self::EUnit { ty: _ } => RcDoc::text("()"),

            Self::EBool { value, ty: _ } => {
                if *value {
                    RcDoc::text("true")
                } else {
                    RcDoc::text("false")
                }
            }

            Self::EColor { value, ty: _ } => match value {
                crate::tast::Color::Red => RcDoc::text("Red"),
                crate::tast::Color::Green => RcDoc::text("Green"),
                crate::tast::Color::Blue => RcDoc::text("Blue"),
            },

            Self::EConstr { index, arg, ty } => ty
                .to_doc()
                .append(RcDoc::text("["))
                .append(RcDoc::text(index.to_string()))
                .append(RcDoc::text("]"))
                .append(RcDoc::text("("))
                .append(arg.to_doc())
                .append(RcDoc::text(")")),

            Self::ETuple { items, ty: _ } => {
                if items.is_empty() {
                    RcDoc::text("()")
                } else {
                    let items_doc = RcDoc::intersperse(
                        items.iter().map(|item| item.to_doc()),
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
                .append(name.to_doc())
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(value.to_doc())
                .append(RcDoc::text(";"))
                .append(RcDoc::hardline())
                .append(body.to_doc())
                .group(),

            Self::EMatch { expr, arms, ty: _ } => {
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

            Self::EPrim { func, args, ty: _ } => {
                if args.is_empty() {
                    RcDoc::text(func).append(RcDoc::text("()"))
                } else {
                    let args_doc = RcDoc::intersperse(
                        args.iter().map(|arg| arg.to_doc()),
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

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl Pat {
    pub fn to_doc(&self) -> RcDoc<()> {
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
                if args.is_empty() {
                    ty.to_doc()
                        .append(RcDoc::text("["))
                        .append(RcDoc::text(index.to_string()))
                        .append(RcDoc::text("]"))
                } else {
                    let args_doc =
                        RcDoc::intersperse(args.iter().map(|arg| arg.to_doc()), RcDoc::text(","));
                    ty.to_doc()
                        .append(RcDoc::text("["))
                        .append(RcDoc::text(index.to_string()))
                        .append(RcDoc::text("]"))
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
                        items.iter().map(|item| item.to_doc()),
                        RcDoc::text(","),
                    );
                    RcDoc::text("(").append(items_doc).append(RcDoc::text(")"))
                }
            }
            Pat::PWild { ty: _ } => RcDoc::text("_"),
        }
    }
    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl crate::tast::Arm {
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
