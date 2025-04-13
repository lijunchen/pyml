use crate::syntax::MySyntaxKind;

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Open {
        kind: MySyntaxKind,
        forward_parent: Option<usize>,
    },
    Close,
    Advance,
    Error(String),
}

impl Event {
    pub fn tombstone() -> Self {
        Event::Open {
            kind: MySyntaxKind::TombStone,
            forward_parent: None,
        }
    }
}
