use ast::ast::Uident;
use ena::unify::{EqUnifyValue, UnifyKey};
use parser::syntax::MySyntaxNodePtr;

#[derive(Debug)]
pub struct File {
    pub toplevels: Vec<Fn>,
}

#[derive(Debug)]
pub struct Fn {
    pub name: String,
    pub params: Vec<(String, Ty)>,
    pub ret_ty: Ty,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    TVar(TypeVar),
    TUnit,
    TBool,
    TInt,
    TTuple { typs: Vec<Ty> },
    TEnum { name: Uident },
    TFunc { params: Vec<Ty>, ret_ty: Box<Ty> },
}

impl Ty {
    pub fn get_constr_name_unsafe(&self) -> String {
        match self {
            Self::TEnum { name } => name.0.clone(),
            _ => {
                panic!("Expected a constructor type, got: {:?}", self)
            }
        }
    }
}

impl EqUnifyValue for Ty {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVar(u32);

impl UnifyKey for TypeVar {
    type Value = Option<Ty>;

    fn index(&self) -> u32 {
        self.0
    }

    fn from_index(u: u32) -> TypeVar {
        TypeVar(u)
    }

    fn tag() -> &'static str {
        "TypeVar"
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    EVar {
        name: String,
        ty: Ty,
        astptr: Option<MySyntaxNodePtr>,
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
        pat: Pat,
        value: Box<Expr>,
        body: Box<Expr>,
        ty: Ty,
    },
    EMatch {
        expr: Box<Expr>,
        arms: Vec<Arm>,
        ty: Ty,
    },
    ECall {
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
            Self::EVar { ty, .. } => ty.clone(),
            Self::EUnit { ty, .. } => ty.clone(),
            Self::EBool { ty, .. } => ty.clone(),
            Self::EInt { ty, .. } => ty.clone(),
            Self::EConstr { ty, .. } => ty.clone(),
            Self::ETuple { ty, .. } => ty.clone(),
            Self::ELet { ty, .. } => ty.clone(),
            Self::EMatch { ty, .. } => ty.clone(),
            Self::ECall { ty, .. } => ty.clone(),
            Self::EProj { ty, .. } => ty.clone(),
        }
    }
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
        astptr: Option<MySyntaxNodePtr>,
    },
    PUnit {
        ty: Ty,
    },
    PBool {
        value: bool,
        ty: Ty,
    },
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
            Self::PVar { ty, .. } => ty.clone(),
            Self::PUnit { ty, .. } => ty.clone(),
            Self::PBool { ty, .. } => ty.clone(),
            Self::PConstr { ty, .. } => ty.clone(),
            Self::PTuple { ty, .. } => ty.clone(),
            Self::PWild { ty, .. } => ty.clone(),
        }
    }
}
