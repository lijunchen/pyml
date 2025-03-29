use pretty::RcDoc;

#[derive(Clone)]
pub enum Ty {
    TUnit,
    TColor,
    TTuple(Vec<Ty>),
    TConstr { name: String },
}

impl Ty {
    pub fn to_doc(&self) -> RcDoc<()> {
        match self {
            Self::TUnit => RcDoc::text("()"),
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

impl std::fmt::Debug for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TUnit => write!(f, "TUnit"),
            Self::TColor => write!(f, "TColor"),
            Self::TTuple(items) => write!(f, "TTuple({:?})", items),
            Self::TConstr { name } => write!(f, "TConstr({})", name),
        }
    }
}

#[derive(Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Red => write!(f, "Red"),
            Self::Green => write!(f, "Green"),
            Self::Blue => write!(f, "Blue"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    EVar {
        name: String,
        ty: Ty,
    },
    EUnit {
        ty: Ty,
    },
    EColor {
        value: Color,
        ty: Ty,
    },
    EConstr {
        index: usize,
        arg: Box<Expr>,
        ty: Ty,
    },
    ETuple {
        items: Vec<Expr>,
        ty: Ty,
    },
    ELet {
        name: Pat,
        value: Box<Expr>,
        body: Box<Expr>,
        ty: Ty,
    },
    EMatch {
        expr: Box<Expr>,
        arms: Vec<Arm>,
        ty: Ty,
    },
    EPrim {
        func: String,
        args: Vec<Expr>,
        ty: Ty,
    },
}

#[derive(Debug, Clone)]
pub struct Arm {
    pub pat: Pat,
    pub body: Expr,
}

#[derive(Clone)]
pub enum Pat {
    PVar {
        name: String,
        ty: Ty,
    },
    PUnit,
    PConstr {
        index: usize,
        args: Vec<Pat>,
        ty: Ty,
    },
    PTuple {
        items: Vec<Pat>,
        ty: Ty,
    },
    PWild {
        ty: Ty,
    },
}

impl std::fmt::Debug for Pat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PVar { name, ty } => write!(f, "PVar({})", name),
            Self::PUnit => write!(f, "()"),
            Self::PConstr { index, args, ty } => {
                write!(f, "PConstr({}, {:?}, {:?})", index, args, ty)
            }
            Self::PTuple { items, ty } => {
                // (a, b, c)
                let items_str = items
                    .iter()
                    .map(|item| format!("{:?}", item))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "({})", items_str)
            }
            Self::PWild { ty } => write!(f, "_"),
        }
    }
}

impl Pat {
    pub fn get_ty(&self) -> Ty {
        match self {
            Self::PVar { name: _, ty } => ty.clone(),
            Self::PUnit => Ty::TUnit,
            Self::PConstr {
                index: _,
                args: _,
                ty,
            } => ty.clone(),
            Self::PTuple { items: _, ty } => ty.clone(),
            Self::PWild { ty } => ty.clone(),
        }
    }
}
