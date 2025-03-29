use pretty::RcDoc;

impl crate::tast::Ty {
    pub fn to_doc(&self) -> RcDoc<()> {
        match self {
            Self::TUnit => RcDoc::text("()"),
            Self::TColor => RcDoc::text("Color"),
            Self::TTuple(items) => {
                let mut doc = RcDoc::text("(");

                if !items.is_empty() {
                    let mut iter = items.iter();
                    if let Some(first) = iter.next() {
                        doc = doc.append(first.to_doc());
                    }
                    for item in iter {
                        doc = doc.append(RcDoc::text(", ")).append(item.to_doc());
                    }
                }

                doc.append(RcDoc::text(")"))
            }
            Self::TConstr { name } => RcDoc::text(name),
        }
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}
