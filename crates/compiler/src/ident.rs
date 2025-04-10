#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lident(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uident(pub String);

impl Uident {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}
