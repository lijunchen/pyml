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
    EBool {
        value: bool,
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

#[derive(Debug)]
pub struct Arm {
    pub lhs: Expr,
    pub body: Expr,
}
