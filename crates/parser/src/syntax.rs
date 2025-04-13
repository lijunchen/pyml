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
    Fn,
    Enum,
    TypeExpr,
    TYPE_LIST,
    TYPE_UNIT,
    TYPE_BOOL,
    TYPE_INT,
    TYPE_TUPLE,
    TYPE_ENUM,
    TYPE_FUNC,
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
    VariantList,
    Variant,
    Pattern,
    PatternTuple,
    PatternConstr,
    PatternVariable,
    MATCH_ARM_LIST,
    MATCH_ARM,
    Arg,
    File,
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
        assert!(raw.0 <= MySyntaxKind::File as u16);
        unsafe { std::mem::transmute::<u16, MySyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
