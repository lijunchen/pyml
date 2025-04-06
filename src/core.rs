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
    EInt {
        value: i32,
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
    EProj {
        tuple: Box<Expr>,
        index: usize,
        ty: Ty,
    },
}

impl Expr {
    pub fn get_ty(&self) -> Ty {
        match self {
            Expr::EVar { ty, .. } => ty.clone(),
            Expr::EUnit { ty } => ty.clone(),
            Expr::EBool { ty, .. } => ty.clone(),
            Expr::EInt { ty, .. } => ty.clone(),
            Expr::EConstr { ty, .. } => ty.clone(),
            Expr::ETuple { ty, .. } => ty.clone(),
            Expr::ELet { ty, .. } => ty.clone(),
            Expr::EMatch { ty, .. } => ty.clone(),
            Expr::EPrim { ty, .. } => ty.clone(),
            Expr::EProj { ty, .. } => ty.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Arm {
    pub lhs: Expr,
    pub body: Expr,
}
