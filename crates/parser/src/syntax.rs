use lexer::TokenKind;

#[allow(bad_style)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Eq,
    Semi,
    Comma,
    Colon,
    Arrow,
    FatArrow,
    Plus,
    Minus,
    Star,
    Slash,
    Dot,
    FnKeyword,
    EnumKeyword,
    MatchKeyword,
    IfKeyword,
    ElseKeyword,
    LetKeyword,
    InKeyword,
    ReturnKeyword,
    TrueKeyword,
    FalseKeyword,
    WildcardKeyword,
    Lident,
    Uident,
    Int32,
    Whitespace,
    Comment,

    Error,

    TombStone,
    ErrorTree,
    Fn,
    TypeExpr,
    ParamList,
    Param,
    Block,
    StmtLet,
    StmtReturn,
    StmtExpr,
    ExprLiteral,
    ExprName,
    ConstructorName,
    ExprParen,
    ExprBinary,
    ExprCall,
    ExprIf,
    ExprLet,
    ExprMatch,
    ExprUnit,
    ExprTuple,
    ArgList,
    EnumDef,
    VariantList,
    Variant,
    TypeList,
    Pattern,
    PatternTuple,
    PatternConstr,
    PatternVariable,
    MATCH_ARM_LIST,
    MATCH_ARM,
    Arg,
    File,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

pub trait ToSyntaxKind {
    fn to_syntax_kind(self) -> rowan::SyntaxKind;
}

impl ToSyntaxKind for TokenKind {
    fn to_syntax_kind(self) -> rowan::SyntaxKind {
        rowan::SyntaxKind(self as u16)
    }
}

pub type MySyntaxNode = rowan::SyntaxNode<MyLang>;
pub type MySyntaxElement = rowan::SyntaxElement<MyLang>;
pub type MySyntaxToken = rowan::SyntaxToken<MyLang>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MyLang {}

impl rowan::Language for MyLang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::File as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
