use std::marker::PhantomData;

use parser::syntax::{MySyntaxKind, MySyntaxNode, MySyntaxNodeChildren, MySyntaxToken};

pub use crate::nodes::*;

pub trait CstNode {
    fn can_cast(kind: MySyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: MySyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &MySyntaxNode;
}

pub trait CstToken {
    fn can_cast(token: MySyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: MySyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &MySyntaxToken;

    fn text(&self) -> &str {
        self.syntax().text()
    }
}

#[derive(Debug, Clone)]
pub struct CstChildren<N> {
    inner: MySyntaxNodeChildren,
    ph: PhantomData<N>,
}

impl<N> CstChildren<N> {
    pub fn new(parent: &MySyntaxNode) -> Self {
        CstChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: CstNode> Iterator for CstChildren<N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}
