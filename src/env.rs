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
        let expr_ty = Ty::TConstr {
            name: "Expr".to_string(),
        };
        enums.insert(
            "Expr".to_string(),
            EnumDef {
                name: "Expr".to_string(),
                variants: vec![
                    ("Zero".to_string(), vec![]),
                    ("Succ".to_string(), vec![expr_ty.clone()]),
                    ("Add".to_string(), vec![expr_ty.clone(), expr_ty.clone()]),
                    ("Mul".to_string(), vec![expr_ty.clone(), expr_ty.clone()]),
                ],
            },
        );
        Self {
            counter: Cell::new(0),
            enums,
        }
    }

    pub fn get_variant_name(&self, tconstr_name: &str, index: i32) -> String {
        let enum_def = self.enums.get(tconstr_name).unwrap();
        let variant = &enum_def.variants[index as usize];
        format!("{}::{}", enum_def.name, variant.0)
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
