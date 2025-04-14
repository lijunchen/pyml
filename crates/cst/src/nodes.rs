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
}

impl_cst_node_simple!(File, MySyntaxKind::FILE);
impl_display_via_syntax!(File);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Enum(Enum),
    Fn(Fn),
}

impl CstNode for Item {
    fn can_cast(kind: MySyntaxKind) -> bool {
        matches!(kind, MySyntaxKind::ENUM | MySyntaxKind::FN)
    }
    fn cast(syntax: MySyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            ENUM => Item::Enum(Enum { syntax }),
            FN => Item::Fn(Fn { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &MySyntaxNode {
        match self {
            Item::Enum(it) => &it.syntax,
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

    pub fn variant_list(&self) -> Option<VariantList> {
        support::child(&self.syntax)
    }
}

impl_cst_node_simple!(Enum, MySyntaxKind::ENUM);
impl_display_via_syntax!(Enum);

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
    PrimExpr(PrimExpr),
    MatchExpr(MatchExpr),
    UidentExpr(UidentExpr),
    LidentExpr(LidentExpr),
    TupleExpr(TupleExpr),
    LetExpr(LetExpr),
}

impl CstNode for Expr {
    fn can_cast(kind: MySyntaxKind) -> bool {
        matches!(
            kind,
            EXPR_UNIT
                | EXPR_BOOL
                | EXPR_INT
                | EXPR_PRIM
                | EXPR_MATCH
                | EXPR_UIDENT
                | EXPR_LIDENT
                | EXPR_TUPLE
                | EXPR_LET
        )
    }
    fn cast(syntax: MySyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            EXPR_UNIT => Expr::UnitExpr(UnitExpr { syntax }),
            EXPR_BOOL => Expr::BoolExpr(BoolExpr { syntax }),
            EXPR_INT => Expr::IntExpr(IntExpr { syntax }),
            EXPR_PRIM => Expr::PrimExpr(PrimExpr { syntax }),
            EXPR_MATCH => Expr::MatchExpr(MatchExpr { syntax }),
            EXPR_UIDENT => Expr::UidentExpr(UidentExpr { syntax }),
            EXPR_LIDENT => Expr::LidentExpr(LidentExpr { syntax }),
            EXPR_TUPLE => Expr::TupleExpr(TupleExpr { syntax }),
            EXPR_LET => Expr::LetExpr(LetExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &MySyntaxNode {
        match self {
            Self::UnitExpr(it) => &it.syntax,
            Self::BoolExpr(it) => &it.syntax,
            Self::IntExpr(it) => &it.syntax,
            Self::PrimExpr(it) => &it.syntax,
            Self::MatchExpr(it) => &it.syntax,
            Self::UidentExpr(it) => &it.syntax,
            Self::LidentExpr(it) => &it.syntax,
            Self::TupleExpr(it) => &it.syntax,
            Self::LetExpr(it) => &it.syntax,
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
        support::token(&self.syntax, MySyntaxKind::Int32)
    }
}

impl_cst_node_simple!(IntExpr, MySyntaxKind::EXPR_INT);
impl_display_via_syntax!(IntExpr);

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
pub struct PrimExpr {
    pub(crate) syntax: MySyntaxNode,
}

impl PrimExpr {
    pub fn lident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Lident)
    }

    pub fn exprs(&self) -> CstChildren<Expr> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(PrimExpr, MySyntaxKind::EXPR_PRIM);
impl_display_via_syntax!(PrimExpr);

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
    pub fn lident(&self) -> Option<MySyntaxToken> {
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
    TupleTy(TupleTy),
    EnumTy(EnumTy),
    FuncTy(FuncTy),
}

impl CstNode for Type {
    fn can_cast(kind: MySyntaxKind) -> bool {
        matches!(
            kind,
            TYPE_UNIT | TYPE_BOOL | TYPE_INT | TYPE_TUPLE | TYPE_ENUM | TYPE_FUNC
        )
    }
    fn cast(syntax: MySyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TYPE_UNIT => Type::UnitTy(UnitTy { syntax }),
            TYPE_BOOL => Type::BoolTy(BoolTy { syntax }),
            TYPE_INT => Type::IntTy(IntTy { syntax }),
            TYPE_TUPLE => Type::TupleTy(TupleTy { syntax }),
            TYPE_ENUM => Type::EnumTy(EnumTy { syntax }),
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
            Type::TupleTy(it) => &it.syntax,
            Type::EnumTy(it) => &it.syntax,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntTy {
    pub(crate) syntax: MySyntaxNode,
}

////////////////////////////////////////////////////////////////////////////////

impl IntTy {}

impl_cst_node_simple!(IntTy, MySyntaxKind::TYPE_INT);
impl_display_via_syntax!(IntTy);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleTy {
    pub(crate) syntax: MySyntaxNode,
}

////////////////////////////////////////////////////////////////////////////////

impl TupleTy {
    pub fn types(&self) -> CstChildren<Type> {
        support::children(&self.syntax)
    }
}

impl_cst_node_simple!(TupleTy, MySyntaxKind::TYPE_TUPLE);
impl_display_via_syntax!(TupleTy);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumTy {
    pub(crate) syntax: MySyntaxNode,
}

impl EnumTy {
    pub fn uident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Uident)
    }
}

impl_cst_node_simple!(EnumTy, MySyntaxKind::TYPE_ENUM);
impl_display_via_syntax!(EnumTy);

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
