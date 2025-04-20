use parser::syntax::MySyntaxNodePtr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lident(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uident(pub String);

impl Uident {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[derive(Debug, Clone)]
pub enum Ty {
    TUnit,
    TBool,
    TInt,
    TTuple { typs: Vec<Ty> },
    TApp { name: Uident, args: Vec<Ty> },
    TFunc { params: Vec<Ty>, ret_ty: Box<Ty> },
}

#[derive(Debug, Clone)]
pub struct File {
    pub toplevels: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    EnumDef(EnumDef),
    TraitDef(TraitDef),
    ImplBlock(ImplBlock),
    Fn(Fn),
}

#[derive(Debug, Clone)]
pub struct Fn {
    pub name: Lident,
    pub generics: Vec<Uident>,
    pub params: Vec<(Lident, Ty)>,
    pub ret_ty: Option<Ty>,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: Uident,
    pub generics: Vec<Uident>,
    pub variants: Vec<(Uident, Vec<Ty>)>,
}

#[derive(Debug, Clone)]
pub struct TraitDef {
    pub name: Uident,
    pub method_sigs: Vec<TraitMethodSignature>,
}

#[derive(Debug, Clone)]
pub struct TraitMethodSignature {
    pub name: Lident,
    pub params: Vec<Ty>,
    pub ret_ty: Ty,
}

#[derive(Debug, Clone)]
pub struct ImplBlock {
    pub trait_name: Uident,
    pub for_type: Ty,
    pub methods: Vec<Fn>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    EVar {
        name: Lident,
        astptr: MySyntaxNodePtr,
    },
    EUnit,
    EBool {
        value: bool,
    },
    EInt {
        value: i32,
    },
    EString {
        value: String,
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
    ECall {
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
    PVar {
        name: Lident,
        astptr: MySyntaxNodePtr,
    },
    PUnit,
    PBool {
        value: bool,
    },
    PConstr {
        vcon: Uident,
        args: Vec<Pat>,
    },
    PTuple {
        pats: Vec<Pat>,
    },
    PWild,
}
