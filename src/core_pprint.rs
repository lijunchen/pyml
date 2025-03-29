use pretty::RcDoc;

use crate::core::{Arm, Expr};

impl Expr {
    pub fn to_doc(&self) -> RcDoc<()> {
        // print as rust source code style
        match self {
            Expr::EVar { name, ty: _ } => RcDoc::text(name.clone()),

            Expr::EUnit { ty: _ } => RcDoc::text("()"),

            Expr::EConstr { index, args, ty } => {
                let prefix = ty
                    .to_doc()
                    .append(RcDoc::text("["))
                    .append(RcDoc::text(index.to_string()))
                    .append(RcDoc::text("]"));

                if args.is_empty() {
                    prefix
                } else {
                    let args_doc =
                        RcDoc::intersperse(args.iter().map(|arg| arg.to_doc()), RcDoc::text(","));

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
                        items.iter().map(|item| item.to_doc()),
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
                .append(value.to_doc())
                .append(RcDoc::text(";"))
                .append(RcDoc::line())
                .append(body.to_doc())
                .group(),

            Expr::EMatch {
                expr,
                arms,
                default,
                ty: _,
            } => {
                let match_expr = RcDoc::text("match")
                    .append(RcDoc::space())
                    .append(expr.to_doc())
                    .append(RcDoc::space())
                    .append(RcDoc::text("{"));

                // Format arms with proper nesting
                let arms_doc = RcDoc::concat(
                    arms.iter()
                        .map(|arm| RcDoc::hardline().append(arm.to_doc())),
                );

                // Format default arm if it exists
                let default_doc = if let Some(default_expr) = default {
                    RcDoc::hardline()
                        .append(RcDoc::text("_"))
                        .append(RcDoc::space())
                        .append(RcDoc::text("=>"))
                        .append(RcDoc::space())
                        .append(default_expr.to_doc())
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
                    RcDoc::intersperse(args.iter().map(|arg| arg.to_doc()), RcDoc::text(","));

                RcDoc::text(func)
                    .append(RcDoc::text("("))
                    .append(args_doc)
                    .append(RcDoc::text(")"))
            }
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
        self.lhs
            .to_doc()
            .append(RcDoc::space())
            .append(RcDoc::text("=>"))
            .append(RcDoc::space())
            .append(self.body.to_doc())
            .append(RcDoc::text(","))
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}
