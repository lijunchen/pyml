use parser::syntax::{MySyntaxKind, MySyntaxNode, MySyntaxToken};

use crate::{
    cst::{CstChildren, CstNode},
    support,
};
use MySyntaxKind::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct File {
    pub(crate) syntax: MySyntaxNode,
}

impl CstNode for File {
    fn can_cast(kind: MySyntaxKind) -> bool {
        kind == MySyntaxKind::File
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

impl File {
    pub fn items(&self) -> CstChildren<Item> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Enum(Enum),
    Fn(Fn),
}

impl CstNode for Item {
    fn can_cast(kind: MySyntaxKind) -> bool {
        matches!(kind, MySyntaxKind::Enum | MySyntaxKind::Fn)
    }
    fn cast(syntax: MySyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            Enum => Item::Enum(Enum { syntax }),
            Fn => Item::Fn(Fn { syntax }),
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

///////////////////////////////////////////////////////////////////

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

impl std::fmt::Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}

impl CstNode for Enum {
    fn can_cast(kind: MySyntaxKind) -> bool {
        kind == Enum
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

///////////////////////////////////////////////////////////////////

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariantList {
    pub(crate) syntax: MySyntaxNode,
}

impl VariantList {
    pub fn variants(&self) -> CstChildren<Variant> {
        support::children(&self.syntax)
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}

impl std::fmt::Display for VariantList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}

impl CstNode for Variant {
    fn can_cast(kind: MySyntaxKind) -> bool {
        kind == Variant
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

impl CstNode for VariantList {
    fn can_cast(kind: MySyntaxKind) -> bool {
        kind == VariantList
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

///////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fn {
    pub(crate) syntax: MySyntaxNode,
}

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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitExpr {
    pub(crate) syntax: MySyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoolExpr {
    pub(crate) syntax: MySyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntExpr {
    pub(crate) syntax: MySyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchExpr {
    pub(crate) syntax: MySyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrimExpr {
    pub(crate) syntax: MySyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UidentExpr {
    pub(crate) syntax: MySyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LidentExpr {
    pub(crate) syntax: MySyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleExpr {
    pub(crate) syntax: MySyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    VarPat(VarPat),
    UnitPat(UnitPat),
    BoolPat(BoolPat),
    ConstrPat(ConstrPat),
    TuplePat(TuplePat),
    WildPat(WildPat),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarPat {
    pub(crate) syntax: MySyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitPat {
    pub(crate) syntax: MySyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoolPat {
    pub(crate) syntax: MySyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstrPat {
    pub(crate) syntax: MySyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TuplePat {
    pub(crate) syntax: MySyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WildPat {
    pub(crate) syntax: MySyntaxNode,
}

///////////////////////////////////////////////////////////////////

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitTy {
    pub(crate) syntax: MySyntaxNode,
}

impl UnitTy {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoolTy {
    pub(crate) syntax: MySyntaxNode,
}

impl BoolTy {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntTy {
    pub(crate) syntax: MySyntaxNode,
}

impl IntTy {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleTy {
    pub(crate) syntax: MySyntaxNode,
}

impl TupleTy {
    pub fn types(&self) -> CstChildren<Type> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumTy {
    pub(crate) syntax: MySyntaxNode,
}

impl EnumTy {
    pub fn uident(&self) -> Option<MySyntaxToken> {
        support::token(&self.syntax, MySyntaxKind::Uident)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncTy {
    pub(crate) syntax: MySyntaxNode,
}

impl FuncTy {}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl CstNode for UnitTy {
    fn can_cast(kind: MySyntaxKind) -> bool {
        kind == TYPE_UNIT
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
impl CstNode for BoolTy {
    fn can_cast(kind: MySyntaxKind) -> bool {
        kind == TYPE_BOOL
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

impl CstNode for IntTy {
    fn can_cast(kind: MySyntaxKind) -> bool {
        kind == TYPE_INT
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

impl CstNode for TupleTy {
    fn can_cast(kind: MySyntaxKind) -> bool {
        kind == TYPE_TUPLE
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
impl CstNode for EnumTy {
    fn can_cast(kind: MySyntaxKind) -> bool {
        kind == TYPE_ENUM
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
impl CstNode for FuncTy {
    fn can_cast(kind: MySyntaxKind) -> bool {
        kind == TYPE_FUNC
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
impl std::fmt::Display for UnitTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BoolTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TupleTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for EnumTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FuncTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}

///////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeList {
    pub(crate) syntax: MySyntaxNode,
}

impl TypeList {
    pub fn types(&self) -> CstChildren<Type> {
        support::children(&self.syntax)
    }
}

impl std::fmt::Display for TypeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}

impl CstNode for TypeList {
    fn can_cast(kind: MySyntaxKind) -> bool {
        kind == TYPE_LIST
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
