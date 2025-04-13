use crate::syntax::SyntaxKind;

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Open {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    Close,
    Advance,
    Error(String),
}

impl Event {
    pub fn tombstone() -> Self {
        Event::Open {
            kind: SyntaxKind::TombStone,
            forward_parent: None,
        }
    }
}
