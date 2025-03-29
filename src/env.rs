use std::{cell::Cell, collections::HashMap};

use crate::tast::Ty;

#[derive(Debug)]
#[allow(unused)]
pub struct EnumDef {
    pub name: String,
    pub variants: Vec<(String, Vec<Ty>)>,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Env {
    counter: Cell<i32>,
    pub enums: HashMap<String, EnumDef>,
}

impl Env {
    pub fn toy_env() -> Self {
        let mut enums = HashMap::new();
        enums.insert(
            "Color".to_string(),
            EnumDef {
                name: "Color".to_string(),
                variants: vec![
                    ("Red".to_string(), vec![]),
                    ("Green".to_string(), vec![]),
                    ("Blue".to_string(), vec![]),
                ],
            },
        );
        Self {
            counter: Cell::new(0),
            enums,
        }
    }

    pub fn gensym(&self, prefix: &str) -> String {
        let count = self.counter.get();
        self.counter.set(count + 1);
        format!("{}{}", prefix, count)
    }

    #[allow(unused)]
    pub fn reset(&self) {
        self.counter.set(0);
    }
}
