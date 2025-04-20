use ast::ast::Uident;
use ena::unify::{EqUnifyValue, UnifyKey};
use parser::syntax::MySyntaxNodePtr;

#[derive(Debug)]
pub struct File {
    pub toplevels: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    ImplBlock(ImplBlock),
    Fn(Fn),
}

#[derive(Debug)]
pub struct ImplBlock {
    pub trait_name: Uident,
    pub for_type: Ty,
    pub methods: Vec<Fn>,
}

#[derive(Debug)]
pub struct Fn {
    pub name: String,
    pub params: Vec<(String, Ty)>,
    pub ret_ty: Ty,
    pub body: Expr,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    TVar(TypeVar),
    TUnit,
    TBool,
    TInt,
    TString,
    TTuple { typs: Vec<Ty> },
    TApp { name: Uident, args: Vec<Ty> },
    TParam { name: String },
    TFunc { params: Vec<Ty>, ret_ty: Box<Ty> },
}

impl std::fmt::Debug for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TVar(var) => write!(f, "TVar({})", var.0),
            Self::TUnit => write!(f, "TUnit"),
            Self::TBool => write!(f, "TBool"),
            Self::TInt => write!(f, "TInt"),
            Self::TString => write!(f, "TString"),
            Self::TTuple { typs } => write!(f, "TTuple({:?})", typs),
            Self::TApp { name, args } => write!(f, "TApp({:?}, {:?})", name, args),
            Self::TParam { name } => write!(f, "TParam({})", name),
            Self::TFunc { params, ret_ty } => write!(f, "TFunc({:?}, {:?})", params, ret_ty),
        }
    }
}

impl Ty {
    pub fn get_constr_name_unsafe(&self) -> String {
        match self {
            Self::TApp { name, .. } => name.0.clone(),
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
    EString {
        value: String,
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
            Self::EString { ty, .. } => ty.clone(),
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
