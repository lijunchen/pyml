use pretty::RcDoc;

pub type Ty = crate::tast::Ty;

#[derive(Debug)]
pub enum Expr {
    EVar {
        name: String,
        ty: Ty,
    },
    EUnit {
        ty: Ty,
    },
    EConstr {
        index: usize,
        args: Vec<Expr>,
        ty: Ty,
    },
    ETuple {
        items: Vec<Expr>,
        ty: Ty,
    },
    ELet {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
        ty: Ty,
    },
    EMatch {
        expr: Box<Expr>,
        arms: Vec<Arm>,
        default: Option<Box<Expr>>,
        ty: Ty,
    },
    EPrim {
        func: String,
        args: Vec<Expr>,
        ty: Ty,
    },
}

impl Expr {
    pub fn to_doc(&self) -> RcDoc<()> {
        // print as rust source code style
        match self {
            Expr::EVar { name, ty: _ } => RcDoc::text(name.clone()),

            Expr::EUnit { ty: _ } => RcDoc::text("()"),

            Expr::EConstr { index, args, ty: _ } => {
                let prefix = RcDoc::text(format!("C{}", index));

                if args.is_empty() {
                    prefix
                } else {
                    let args_doc = RcDoc::intersperse(
                        args.iter().map(|arg| arg.to_doc()),
                        RcDoc::text(",").append(RcDoc::space()),
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
                        items.iter().map(|item| item.to_doc()),
                        RcDoc::text(",").append(RcDoc::space()),
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
                .append(body.to_doc()),

            Expr::EMatch {
                expr,
                arms,
                default,
                ty: _,
            } => {
                let mut result = RcDoc::text("match")
                    .append(RcDoc::space())
                    .append(expr.to_doc())
                    .append(RcDoc::space())
                    .append(RcDoc::text("{"))
                    .append(RcDoc::line());

                for arm in arms {
                    result = result
                        .append(RcDoc::text("    "))
                        .append(arm.to_doc())
                        .append(RcDoc::line());
                }

                // Format default arm if it exists
                if let Some(default_expr) = default {
                    result = result
                        .append(RcDoc::text("    "))
                        .append(RcDoc::text("_"))
                        .append(RcDoc::space())
                        .append(RcDoc::text("=>"))
                        .append(RcDoc::space())
                        .append(default_expr.to_doc())
                        .append(RcDoc::text(","))
                        .append(RcDoc::line());
                }

                result.append(RcDoc::text("}"))
            }

            Expr::EPrim { func, args, ty: _ } => {
                let args_doc = RcDoc::intersperse(
                    args.iter().map(|arg| arg.to_doc()),
                    RcDoc::text(",").append(RcDoc::space()),
                );

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

#[derive(Debug)]
pub struct Arm {
    pub lhs: Expr,
    pub body: Expr,
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
