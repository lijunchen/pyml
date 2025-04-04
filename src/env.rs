use crate::tast;
use std::{cell::Cell, collections::HashMap};

use crate::ident::Uident;

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: Uident,
    pub variants: Vec<(Uident, Vec<tast::Ty>)>,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Env {
    counter: Cell<i32>,
    pub enums: HashMap<Uident, EnumDef>,
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

impl Env {
    pub fn new() -> Self {
        Self {
            counter: Cell::new(0),
            enums: HashMap::new(),
        }
    }
    pub fn toy_env() -> Self {
        let mut enums = HashMap::new();
        enums.insert(
            Uident::new("Color"),
            EnumDef {
                name: Uident::new("Color"),
                variants: vec![
                    (Uident::new("Red"), vec![]),
                    (Uident::new("Green"), vec![]),
                    (Uident::new("Blue"), vec![]),
                ],
            },
        );
        let expr_ty = tast::Ty::TConstr {
            name: Uident::new("Expr"),
        };
        enums.insert(
            Uident::new("Expr"),
            EnumDef {
                name: Uident::new("Expr"),
                variants: vec![
                    (Uident::new("Zero"), vec![]),
                    (Uident::new("Succ"), vec![expr_ty.clone()]),
                    (Uident::new("Add"), vec![expr_ty.clone(), expr_ty.clone()]),
                    (Uident::new("Mul"), vec![expr_ty.clone(), expr_ty.clone()]),
                ],
            },
        );
        Self {
            counter: Cell::new(0),
            enums,
        }
    }

    pub fn get_variant_name(&self, tconstr_name: &str, index: i32) -> String {
        let enum_def = self.enums.get(&Uident::new(tconstr_name)).unwrap();
        let variant = &enum_def.variants[index as usize];
        format!("{}::{}", enum_def.name.0, variant.0.0)
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

    pub fn get_type_of_constructor(&self, constr: &str) -> Option<tast::Ty> {
        for (enum_name, enum_def) in self.enums.iter() {
            for variant in enum_def.variants.iter() {
                if variant.0.0 == constr {
                    return Some(tast::Ty::TConstr {
                        name: enum_name.clone(),
                    });
                }
            }
        }
        None
    }

    pub fn get_index_of_constructor(&self, constr: &str) -> Option<i32> {
        for (_, enum_def) in self.enums.iter() {
            for (index, variant) in enum_def.variants.iter().enumerate() {
                if variant.0.0 == constr {
                    return Some(index as i32);
                }
            }
        }
        None
    }

    pub fn get_args_ty_of_constructor(&self, constr: &str) -> Vec<tast::Ty> {
        for (_, enum_def) in self.enums.iter() {
            for variant in enum_def.variants.iter() {
                if variant.0.0 == constr {
                    return variant.1.clone();
                }
            }
        }
        vec![]
    }
}
