use ast::ast::{Lident, Uident};

use crate::tast;
use std::{cell::Cell, collections::HashMap};

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
    pub funcs: HashMap<Lident, tast::Ty>,
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
            funcs: HashMap::new(),
        }
    }

    pub fn get_variant_name(&self, tenum_name: &str, index: i32) -> String {
        let enum_def = self.enums.get(&Uident::new(tenum_name)).unwrap();
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
                    return Some(tast::Ty::TEnum {
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

    pub fn get_type_of_function(&self, func: &str) -> Option<tast::Ty> {
        self.funcs.get(&Lident(func.to_string())).cloned()
    }

    pub fn get_args_ty_of_function(&self, func: &str) -> Vec<tast::Ty> {
        if let Some(ty) = self.get_type_of_function(func) {
            match ty {
                tast::Ty::TFunc { params, .. } => params,
                _ => vec![],
            }
        } else {
            vec![]
        }
    }

    pub fn get_ret_ty_of_function(&self, func: &str) -> tast::Ty {
        if let Some(ty) = self.get_type_of_function(func) {
            match ty {
                tast::Ty::TFunc { ret_ty, .. } => *ret_ty,
                _ => panic!("Expected a function type for {}", func),
            }
        } else {
            panic!("Function {} not found in environment", func);
        }
    }
}
