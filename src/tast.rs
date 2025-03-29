#[derive(Debug, Clone)]
pub enum Ty {
    TUnit,
    TColor,
    TTuple(Vec<Ty>),
    TConstr { name: String },
}

#[derive(Debug, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
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

#[derive(Debug, Clone)]
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
