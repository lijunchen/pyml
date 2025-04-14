use lexer::TokenKind;

#[allow(bad_style)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum MySyntaxKind {
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
    UnitKeyword,
    BoolKeyword,
    IntKeyword,
    Lident,
    Uident,
    Int32,
    Whitespace,
    Comment,

    Error,

    TombStone,
    ErrorTree,
    FN,
    ENUM,
    TYPE_LIST,
    TYPE_UNIT,
    TYPE_BOOL,
    TYPE_INT,
    TYPE_TUPLE,
    TYPE_ENUM,
    TYPE_FUNC,
    EXPR_LITERAL,
    EXPR_UIDENT,
    EXPR_LIDENT,
    EXPR_PAREN,
    EXPR_BINARY,
    EXPR_CALL,
    EXPR_IF,
    EXPR_LET,
    EXPR_LET_VALUE,
    EXPR_LET_BODY,
    EXPR_MATCH,
    EXPR_UNIT,
    EXPR_BOOL,
    EXPR_INT,
    EXPR_TUPLE,
    EXPR_PRIM,
    ARG_LIST,
    ARG,
    VARIANT_LIST,
    VARIANT,
    PATTERN,
    PATTERN_UNIT,
    PATTERN_BOOL,
    PATTERN_INT,
    PATTERN_TUPLE,
    PATTERN_CONSTR,
    PATTERN_VARIABLE,
    PATTERN_WILDCARD,
    MATCH_ARM_LIST,
    MATCH_ARM,
    PARAM,
    PARAM_LIST,
    BLOCK,
    FILE,
}

impl From<MySyntaxKind> for rowan::SyntaxKind {
    fn from(kind: MySyntaxKind) -> Self {
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
pub type MySyntaxNodeChildren = rowan::SyntaxNodeChildren<MyLang>;
pub type MySyntaxElementChildren = rowan::SyntaxElementChildren<MyLang>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MyLang {}

impl rowan::Language for MyLang {
    type Kind = MySyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= MySyntaxKind::FILE as u16);
        unsafe { std::mem::transmute::<u16, MySyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
