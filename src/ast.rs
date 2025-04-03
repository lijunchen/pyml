#[derive(Debug, Clone)]
pub enum Ty {
    TUnit,
    TBool,
    TTuple { typs: Vec<Ty> },
    TConstr { name: String },
}

#[derive(Debug, Clone)]
pub struct Lident(pub String);

#[derive(Debug, Clone)]
pub struct Uident(pub String);

#[derive(Debug, Clone)]
pub enum Expr {
    EVar {
        name: Lident,
    },
    EUnit,
    EBool {
        value: bool,
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
        index: u32,
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
