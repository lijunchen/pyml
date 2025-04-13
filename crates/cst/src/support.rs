use parser::syntax::{MySyntaxKind, MySyntaxNode, MySyntaxToken};

use crate::cst::{CstChildren, CstNode};

pub fn child<N: CstNode>(parent: &MySyntaxNode) -> Option<N> {
    parent.children().find_map(N::cast)
}

pub fn children<N: CstNode>(parent: &MySyntaxNode) -> CstChildren<N> {
    CstChildren::new(parent)
}

pub fn token(parent: &MySyntaxNode, kind: MySyntaxKind) -> Option<MySyntaxToken> {
    parent
        .children_with_tokens()
        .filter_map(|it| it.into_token())
        .find(|it| it.kind() == kind)
}
