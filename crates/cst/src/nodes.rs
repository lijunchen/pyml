use parser::syntax::{MySyntaxKind, MySyntaxNode, MySyntaxToken};

use crate::{
    cst::{CstChildren, CstNode},
    support,
};
use MySyntaxKind::*;

macro_rules! impl_cst_node_simple {
    ($node:ident, $syntax_kind:expr) => {
        impl CstNode for $node {
            fn can_cast(kind: MySyntaxKind) -> bool {
                kind == $syntax_kind
            }
            fn cast(syntax: MySyntaxNode) -> Option<Self> {
                if Self::can_cast(syntax.kind()) {
                    Some(Self { syntax })
                } else {
                    None
                }
            }
            fn syntax(&self) -> &MySyntaxNode {
                &self.syntax
            }
        }
    };
}

macro_rules! impl_display_via_syntax {
    ($node:ident) => {
        impl std::fmt::Display for $node {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(self.syntax(), f)
            }
        }
    };
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct File {
    pub(crate) syntax: MySyntaxNode,
}

impl File {
    pub fn items(&self) -> CstChildren<Item> {
        support::children(&self.syntax)
    }

    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(File, MySyntaxKind::FILE);
impl_display_via_syntax!(File);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Enum(Enum),
    Trait(Trait),
    Impl(Impl),
    Fn(Fn),
}

impl CstNode for Item {
    fn can_cast(kind: MySyntaxKind) -> bool {
        matches!(
            kind,
            MySyntaxKind::ENUM | MySyntaxKind::TRAIT | MySyntaxKind::IMPL | MySyntaxKind::FN
        )
    }
    fn cast(syntax: MySyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            ENUM => Item::Enum(Enum { syntax }),
            TRAIT => Item::Trait(Trait { syntax }),
            IMPL => Item::Impl(Impl { syntax }),
            FN => Item::Fn(Fn { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &MySyntaxNode {
        match self {
            Item::Enum(it) => &it.syntax,
            Item::Trait(it) => &it.syntax,
            Item::Impl(it) => &it.syntax,
            Item::Fn(it) => &it.syntax,
        }
    }
}

impl_display_via_syntax!(Item);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Enum {
    pub(crate) syntax: MySyntaxNode,
}

impl Enum {
    pub fn uident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Uident)
    }

    pub fn generic_list(&self) -> Option<GenericList> {
        support::child(&self.syntax)
    }

    pub fn variant_list(&self) -> Option<VariantList> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(Enum, MySyntaxKind::ENUM);
impl_display_via_syntax!(Enum);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Trait {
    pub(crate) syntax: MySyntaxNode,
}

impl Trait {
    pub fn uident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Uident)
    }

    pub fn trait_method_list(&self) -> Option<TraitMethodList> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(Trait, MySyntaxKind::TRAIT);
impl_display_via_syntax!(Trait);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitMethodList {
    pub(crate) syntax: MySyntaxNode,
}

impl TraitMethodList {
    pub fn methods(&self) -> CstChildren<TraitMethod> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(TraitMethodList, MySyntaxKind::TRAIT_METHOD_SIG_LIST);
impl_display_via_syntax!(TraitMethodList);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitMethod {
    pub(crate) syntax: MySyntaxNode,
}

impl TraitMethod {
    pub fn lident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Lident)
    }

    pub fn type_list(&self) -> Option<TypeList> {
        support::child(&self.syntax)
    }

    pub fn return_type(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(TraitMethod, MySyntaxKind::TRAIT_METHOD_SIG);
impl_display_via_syntax!(TraitMethod);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Impl {
    pub(crate) syntax: MySyntaxNode,
}

impl Impl {
    pub fn uident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Uident)
    }

    pub fn for_type(&self) -> Option<Type> {
        support::child(&self.syntax)
    }

    pub fn functions(&self) -> CstChildren<Fn> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(Impl, MySyntaxKind::IMPL);
impl_display_via_syntax!(Impl);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Generic {
    pub(crate) syntax: MySyntaxNode,
}

impl Generic {
    pub fn uident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Uident)
    }
}
impl_cst_node_simple!(Generic, MySyntaxKind::GENERIC);
impl_display_via_syntax!(Generic);

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericList {
    pub(crate) syntax: MySyntaxNode,
}

impl GenericList {
    pub fn generics(&self) -> CstChildren<Generic> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(GenericList, MySyntaxKind::GENERIC_LIST);
impl_display_via_syntax!(GenericList);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParamList {
    pub(crate) syntax: MySyntaxNode,
}

impl TypeParamList {
    pub fn types(&self) -> CstChildren<Type> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(TypeParamList, MySyntaxKind::TYPE_PARAM_LIST);
impl_display_via_syntax!(TypeParamList);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    pub(crate) syntax: MySyntaxNode,
}

impl Variant {
    pub fn uident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Uident)
    }

    pub fn type_list(&self) -> Option<TypeList> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(Variant, MySyntaxKind::VARIANT);
impl_display_via_syntax!(Variant);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariantList {
    pub(crate) syntax: MySyntaxNode,
}

impl VariantList {
    pub fn variants(&self) -> CstChildren<Variant> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(VariantList, MySyntaxKind::VARIANT_LIST);
impl_display_via_syntax!(VariantList);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fn {
    pub(crate) syntax: MySyntaxNode,
}

impl Fn {
    pub fn lident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Lident)
    }

    pub fn generic_list(&self) -> Option<GenericList> {
        support::child(&self.syntax)
    }

    pub fn param_list(&self) -> Option<ParamList> {
        support::child(&self.syntax)
    }

    pub fn return_type(&self) -> Option<Type> {
        support::child(&self.syntax)
    }

    pub fn block(&self) -> Option<Block> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(Fn, MySyntaxKind::FN);
impl_display_via_syntax!(Fn);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamList {
    pub(crate) syntax: MySyntaxNode,
}

impl ParamList {
    pub fn params(&self) -> CstChildren<Param> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(ParamList, MySyntaxKind::PARAM_LIST);
impl_display_via_syntax!(ParamList);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub(crate) syntax: MySyntaxNode,
}

impl Param {
    pub fn lident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Lident)
    }

    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(Param, MySyntaxKind::PARAM);
impl_display_via_syntax!(Param);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub(crate) syntax: MySyntaxNode,
}

impl Block {
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(Block, MySyntaxKind::BLOCK);
impl_display_via_syntax!(Block);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    UnitExpr(UnitExpr),
    BoolExpr(BoolExpr),
    IntExpr(IntExpr),
    StrExpr(StrExpr),
    CallExpr(CallExpr),
    MatchExpr(MatchExpr),
    UidentExpr(UidentExpr),
    LidentExpr(LidentExpr),
    TupleExpr(TupleExpr),
    LetExpr(LetExpr),
    BinaryExpr(BinaryExpr),
}

impl CstNode for Expr {
    fn can_cast(kind: MySyntaxKind) -> bool {
        matches!(
            kind,
            EXPR_UNIT
                | EXPR_BOOL
                | EXPR_INT
                | EXPR_STR
                | EXPR_CALL
                | EXPR_MATCH
                | EXPR_UIDENT
                | EXPR_LIDENT
                | EXPR_TUPLE
                | EXPR_LET
                | EXPR_BINARY
        )
    }
    fn cast(syntax: MySyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            EXPR_UNIT => Expr::UnitExpr(UnitExpr { syntax }),
            EXPR_BOOL => Expr::BoolExpr(BoolExpr { syntax }),
            EXPR_INT => Expr::IntExpr(IntExpr { syntax }),
            EXPR_STR => Expr::StrExpr(StrExpr { syntax }),
            EXPR_CALL => Expr::CallExpr(CallExpr { syntax }),
            EXPR_MATCH => Expr::MatchExpr(MatchExpr { syntax }),
            EXPR_UIDENT => Expr::UidentExpr(UidentExpr { syntax }),
            EXPR_LIDENT => Expr::LidentExpr(LidentExpr { syntax }),
            EXPR_TUPLE => Expr::TupleExpr(TupleExpr { syntax }),
            EXPR_LET => Expr::LetExpr(LetExpr { syntax }),
            EXPR_BINARY => Expr::BinaryExpr(BinaryExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &MySyntaxNode {
        match self {
            Self::UnitExpr(it) => &it.syntax,
            Self::BoolExpr(it) => &it.syntax,
            Self::IntExpr(it) => &it.syntax,
            Self::StrExpr(it) => &it.syntax,
            Self::CallExpr(it) => &it.syntax,
            Self::MatchExpr(it) => &it.syntax,
            Self::UidentExpr(it) => &it.syntax,
            Self::LidentExpr(it) => &it.syntax,
            Self::TupleExpr(it) => &it.syntax,
            Self::LetExpr(it) => &it.syntax,
            Self::BinaryExpr(it) => &it.syntax,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitExpr {
    pub(crate) syntax: MySyntaxNode,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoolExpr {
    pub(crate) syntax: MySyntaxNode,
}

impl BoolExpr {
    pub fn value(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::TrueKeyword)
            .or_else(|| support::token(&self.syntax, MySyntaxKind::FalseKeyword))
    }
}

impl_cst_node_simple!(BoolExpr, MySyntaxKind::EXPR_BOOL);
impl_display_via_syntax!(BoolExpr);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntExpr {
    pub(crate) syntax: MySyntaxNode,
}

impl IntExpr {
    pub fn value(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Int)
    }
}

impl_cst_node_simple!(IntExpr, MySyntaxKind::EXPR_INT);
impl_display_via_syntax!(IntExpr);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StrExpr {
    pub(crate) syntax: MySyntaxNode,
}

impl StrExpr {
    pub fn value(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Str)
    }
}

impl_cst_node_simple!(StrExpr, MySyntaxKind::EXPR_STR);
impl_display_via_syntax!(StrExpr);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchExpr {
    pub(crate) syntax: MySyntaxNode,
}

impl MatchExpr {
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }

    pub fn match_arm_list(&self) -> Option<MatchArmList> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(MatchExpr, MySyntaxKind::EXPR_MATCH);
impl_display_via_syntax!(MatchExpr);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchArmList {
    pub(crate) syntax: MySyntaxNode,
}

impl MatchArmList {
    pub fn arms(&self) -> CstChildren<MatchArm> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(MatchArmList, MySyntaxKind::MATCH_ARM_LIST);
impl_display_via_syntax!(MatchArmList);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchArm {
    pub(crate) syntax: MySyntaxNode,
}

impl MatchArm {
    pub fn pattern(&self) -> Option<Pattern> {
        support::child(&self.syntax)
    }

    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(MatchArm, MySyntaxKind::MATCH_ARM);
impl_display_via_syntax!(MatchArm);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpr {
    pub(crate) syntax: MySyntaxNode,
}

impl CallExpr {
    pub fn l_name(&self) -> Option<String> {
        let node: Option<Expr> = support::child(&self.syntax);
        match node {
            Some(Expr::LidentExpr(it)) => it.lident_token().map(|t| t.text().to_string()),
            _ => None,
        }
    }

    pub fn u_name(&self) -> Option<String> {
        let node: Option<Expr> = support::child(&self.syntax);
        match node {
            Some(Expr::UidentExpr(it)) => it.uident().map(|t| t.text().to_string()),
            _ => None,
        }
    }

    pub fn arg_list(&self) -> Option<ArgList> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(CallExpr, MySyntaxKind::EXPR_CALL);
impl_display_via_syntax!(CallExpr);

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgList {
    pub(crate) syntax: MySyntaxNode,
}

impl ArgList {
    pub fn args(&self) -> CstChildren<Arg> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(ArgList, MySyntaxKind::ARG_LIST);
impl_display_via_syntax!(ArgList);

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arg {
    pub(crate) syntax: MySyntaxNode,
}

impl Arg {
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }

    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(Arg, MySyntaxKind::ARG);
impl_display_via_syntax!(Arg);

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UidentExpr {
    pub(crate) syntax: MySyntaxNode,
}

impl UidentExpr {
    pub fn uident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Uident)
    }
}

impl_cst_node_simple!(UidentExpr, MySyntaxKind::EXPR_UIDENT);
impl_display_via_syntax!(UidentExpr);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LidentExpr {
    pub(crate) syntax: MySyntaxNode,
}

impl LidentExpr {
    pub fn lident_token(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Lident)
    }
}

impl_cst_node_simple!(LidentExpr, MySyntaxKind::EXPR_LIDENT);
impl_display_via_syntax!(LidentExpr);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleExpr {
    pub(crate) syntax: MySyntaxNode,
}

impl TupleExpr {
    pub fn exprs(&self) -> CstChildren<Expr> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(TupleExpr, MySyntaxKind::EXPR_TUPLE);
impl_display_via_syntax!(TupleExpr);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetExpr {
    pub(crate) syntax: MySyntaxNode,
}

impl LetExpr {
    pub fn pattern(&self) -> Option<Pattern> {
        support::child(&self.syntax)
    }

    pub fn value(&self) -> Option<LetExprValue> {
        support::child(&self.syntax)
    }

    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }

    pub fn body(&self) -> Option<LetExprBody> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(LetExpr, MySyntaxKind::EXPR_LET);
impl_display_via_syntax!(LetExpr);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryExpr {
    pub(crate) syntax: MySyntaxNode,
}

impl BinaryExpr {
    pub fn exprs(&self) -> CstChildren<Expr> {
        support::children(&self.syntax)
    }

    pub fn op(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Dot)
    }
}

impl_cst_node_simple!(BinaryExpr, MySyntaxKind::EXPR_BINARY);
impl_display_via_syntax!(BinaryExpr);

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetExprValue {
    pub(crate) syntax: MySyntaxNode,
}

impl LetExprValue {
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
impl_cst_node_simple!(LetExprValue, MySyntaxKind::EXPR_LET_VALUE);
impl_display_via_syntax!(LetExprValue);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetExprBody {
    pub(crate) syntax: MySyntaxNode,
}

impl LetExprBody {
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
impl_cst_node_simple!(LetExprBody, MySyntaxKind::EXPR_LET_BODY);
impl_display_via_syntax!(LetExprBody);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    VarPat(VarPat),
    UnitPat(UnitPat),
    BoolPat(BoolPat),
    ConstrPat(ConstrPat),
    TuplePat(TuplePat),
    WildPat(WildPat),
}

impl CstNode for Pattern {
    fn can_cast(kind: MySyntaxKind) -> bool {
        matches!(
            kind,
            PATTERN_VARIABLE
                | PATTERN_UNIT
                | PATTERN_BOOL
                | PATTERN_CONSTR
                | PATTERN_TUPLE
                | PATTERN_WILDCARD
        )
    }
    fn cast(syntax: MySyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            PATTERN_VARIABLE => Pattern::VarPat(VarPat { syntax }),
            PATTERN_UNIT => Pattern::UnitPat(UnitPat { syntax }),
            PATTERN_BOOL => Pattern::BoolPat(BoolPat { syntax }),
            PATTERN_CONSTR => Pattern::ConstrPat(ConstrPat { syntax }),
            PATTERN_TUPLE => Pattern::TuplePat(TuplePat { syntax }),
            PATTERN_WILDCARD => Pattern::WildPat(WildPat { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &MySyntaxNode {
        match self {
            Self::VarPat(it) => &it.syntax,
            Self::UnitPat(it) => &it.syntax,
            Self::BoolPat(it) => &it.syntax,
            Self::ConstrPat(it) => &it.syntax,
            Self::TuplePat(it) => &it.syntax,
            Self::WildPat(it) => &it.syntax,
        }
    }
}

impl_display_via_syntax!(Pattern);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarPat {
    pub(crate) syntax: MySyntaxNode,
}

impl VarPat {
    pub fn lident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Lident)
    }
}

impl_cst_node_simple!(VarPat, MySyntaxKind::PATTERN_VARIABLE);
impl_display_via_syntax!(VarPat);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitPat {
    pub(crate) syntax: MySyntaxNode,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoolPat {
    pub(crate) syntax: MySyntaxNode,
}

impl BoolPat {
    pub fn value(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::TrueKeyword)
            .or_else(|| support::token(&self.syntax, MySyntaxKind::FalseKeyword))
    }
}

impl_cst_node_simple!(BoolPat, MySyntaxKind::PATTERN_BOOL);
impl_display_via_syntax!(BoolPat);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstrPat {
    pub(crate) syntax: MySyntaxNode,
}

impl ConstrPat {
    pub fn uident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Uident)
    }

    pub fn patterns(&self) -> CstChildren<Pattern> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(ConstrPat, MySyntaxKind::PATTERN_CONSTR);
impl_display_via_syntax!(ConstrPat);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TuplePat {
    pub(crate) syntax: MySyntaxNode,
}

impl TuplePat {
    pub fn patterns(&self) -> CstChildren<Pattern> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(TuplePat, MySyntaxKind::PATTERN_TUPLE);
impl_display_via_syntax!(TuplePat);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WildPat {
    pub(crate) syntax: MySyntaxNode,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    UnitTy(UnitTy),
    BoolTy(BoolTy),
    IntTy(IntTy),
    StringTy(StringTy),
    TupleTy(TupleTy),
    TAppTy(TAppTy),
    FuncTy(FuncTy),
}

impl CstNode for Type {
    fn can_cast(kind: MySyntaxKind) -> bool {
        matches!(
            kind,
            TYPE_UNIT | TYPE_BOOL | TYPE_INT | TYPE_STRING | TYPE_TUPLE | TYPE_TAPP | TYPE_FUNC
        )
    }
    fn cast(syntax: MySyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TYPE_UNIT => Type::UnitTy(UnitTy { syntax }),
            TYPE_BOOL => Type::BoolTy(BoolTy { syntax }),
            TYPE_INT => Type::IntTy(IntTy { syntax }),
            TYPE_STRING => Type::StringTy(StringTy { syntax }),
            TYPE_TUPLE => Type::TupleTy(TupleTy { syntax }),
            TYPE_TAPP => Type::TAppTy(TAppTy { syntax }),
            TYPE_FUNC => Type::FuncTy(FuncTy { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &MySyntaxNode {
        match self {
            Type::UnitTy(it) => &it.syntax,
            Type::BoolTy(it) => &it.syntax,
            Type::IntTy(it) => &it.syntax,
            Type::StringTy(it) => &it.syntax,
            Type::TupleTy(it) => &it.syntax,
            Type::TAppTy(it) => &it.syntax,
            Type::FuncTy(it) => &it.syntax,
        }
    }
}

impl_display_via_syntax!(Type);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitTy {
    pub(crate) syntax: MySyntaxNode,
}

impl UnitTy {}

impl_cst_node_simple!(UnitTy, MySyntaxKind::TYPE_UNIT);
impl_display_via_syntax!(UnitTy);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoolTy {
    pub(crate) syntax: MySyntaxNode,
}

impl BoolTy {}

impl_cst_node_simple!(BoolTy, MySyntaxKind::TYPE_BOOL);
impl_display_via_syntax!(BoolTy);

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntTy {
    pub(crate) syntax: MySyntaxNode,
}

impl IntTy {}

impl_cst_node_simple!(IntTy, MySyntaxKind::TYPE_INT);
impl_display_via_syntax!(IntTy);
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringTy {
    pub(crate) syntax: MySyntaxNode,
}

impl StringTy {}

impl_cst_node_simple!(StringTy, MySyntaxKind::TYPE_STRING);
impl_display_via_syntax!(StringTy);
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleTy {
    pub(crate) syntax: MySyntaxNode,
}

////////////////////////////////////////////////////////////////////////////////

impl TupleTy {
    pub fn type_list(&self) -> Option<TypeList> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(TupleTy, MySyntaxKind::TYPE_TUPLE);
impl_display_via_syntax!(TupleTy);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TAppTy {
    pub(crate) syntax: MySyntaxNode,
}

impl TAppTy {
    pub fn uident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Uident)
    }

    pub fn type_param_list(&self) -> Option<TypeParamList> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(TAppTy, MySyntaxKind::TYPE_TAPP);
impl_display_via_syntax!(TAppTy);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncTy {
    pub(crate) syntax: MySyntaxNode,
}

impl FuncTy {}

impl_cst_node_simple!(FuncTy, MySyntaxKind::TYPE_FUNC);
impl_display_via_syntax!(FuncTy);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeList {
    pub(crate) syntax: MySyntaxNode,
}

impl TypeList {
    pub fn types(&self) -> CstChildren<Type> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(TypeList, MySyntaxKind::TYPE_LIST);
impl_display_via_syntax!(TypeList);
