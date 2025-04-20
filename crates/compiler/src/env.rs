use ast::ast::{Lident, Uident};

use crate::tast;
use std::{cell::Cell, collections::HashMap};

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: Uident,
    pub generics: Vec<Uident>,
    pub variants: Vec<(Uident, Vec<tast::Ty>)>,
}

pub fn encode_trait_impl(trait_name: &Uident, type_name: &tast::Ty) -> String {
    let trait_name = trait_name.0.clone();
    let type_name = type_name.clone();
    // impl ToString for Int
    // __ToString%Int
    format!(
        "__{}%{}",
        trait_name,
        match type_name {
            tast::Ty::TUnit => "Unit".to_string(),
            tast::Ty::TInt => "Int".to_string(),
            tast::Ty::TBool => "Bool".to_string(),
            tast::Ty::TString => "String".to_string(),
            _ => {
                todo!()
            }
        }
    )
}

pub fn decode_trait_impl(impl_name: &str) -> (Uident, tast::Ty) {
    let parts: Vec<&str> = impl_name.split('%').collect();
    if parts.len() != 2 {
        panic!("Invalid trait impl name format");
    }
    let trait_name = Uident::new(parts[0].trim_start_matches("__"));
    let type_name = match parts[1] {
        "Unit" => tast::Ty::TUnit,
        "Bool" => tast::Ty::TBool,
        "Int" => tast::Ty::TInt,
        "String" => tast::Ty::TString,
        _ => {
            todo!()
        }
    };
    (trait_name, type_name)
}

#[derive(Debug, Clone)]
pub enum Constraint {
    TypeEqual(tast::Ty, tast::Ty),
    Overloaded {
        op: Lident,
        trait_name: Uident,
        call_site_type: tast::Ty,
    },
}

#[derive(Debug)]
#[allow(unused)]
pub struct Env {
    counter: Cell<i32>,
    pub enums: HashMap<Uident, EnumDef>,
    pub trait_defs: HashMap<String, tast::Ty>,
    pub overloaded_funcs_to_trait_name: HashMap<String, Uident>,
    pub trait_impls: HashMap<(String, tast::Ty, Lident), tast::Ty>,
    pub funcs: HashMap<String, tast::Ty>,
    pub constraints: Vec<Constraint>,
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
            trait_defs: HashMap::new(),
            overloaded_funcs_to_trait_name: HashMap::new(),
            trait_impls: HashMap::new(),
            constraints: Vec::new(),
        }
    }

    pub fn get_trait_impl(
        &self,
        trait_name: &Uident,
        type_name: &tast::Ty,
        func_name: &Lident,
    ) -> Option<tast::Ty> {
        self.trait_impls
            .get(&(trait_name.0.clone(), type_name.clone(), func_name.clone()))
            .cloned()
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
                    let return_ty = tast::Ty::TApp {
                        name: enum_name.clone(),
                        args: enum_def
                            .generics
                            .iter()
                            .map(|g| tast::Ty::TParam { name: g.0.clone() })
                            .collect(),
                    };

                    let params_ty = variant.1.clone();

                    if params_ty.is_empty() {
                        return Some(return_ty);
                    }

                    return Some(tast::Ty::TFunc {
                        params: params_ty,
                        ret_ty: Box::new(return_ty),
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

    pub fn get_type_of_function(&self, func: &str) -> Option<tast::Ty> {
        self.funcs.get(func).cloned()
    }
}
