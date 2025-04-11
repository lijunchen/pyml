use crate::ident::{Lident, Uident};

#[derive(Debug, Clone)]
pub enum Ty {
    TUnit,
    TBool,
    TTuple { typs: Vec<Ty> },
    TEnum { name: Uident },
}

#[derive(Debug, Clone)]
pub struct File {
    pub toplevels: Vec<Item>,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Item {
    EnumDef(EnumDef),
    Fn(Fn),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Fn {
    pub name: Lident,
    pub params: Vec<(Lident, Ty)>,
    pub ret_ty: Option<Ty>,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: Uident,
    pub variants: Vec<(Uident, Vec<Ty>)>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    EVar {
        name: Lident,
    },
    EUnit,
    EBool {
        value: bool,
    },
    EInt {
        value: i32,
    },
    EConstr {
        vcon: Uident,
        args: Vec<Expr>,
    },
    ETuple {
        items: Vec<Expr>,
    },
    ELet {
        pat: Pat,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    EMatch {
        expr: Box<Expr>,
        arms: Vec<Arm>,
    },
    EPrim {
        func: Lident,
        args: Vec<Expr>,
    },
    EProj {
        tuple: Box<Expr>,
        index: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub struct Arm {
    pub pat: Pat,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub enum Pat {
    PVar { name: Lident },
    PUnit,
    PBool { value: bool },
    PConstr { vcon: Uident, args: Vec<Pat> },
    PTuple { pats: Vec<Pat> },
    PWild,
}
