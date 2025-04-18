use std::collections::HashMap;

use ::ast::ast::Lident;
use ast::ast;
use ena::unify::InPlaceUnificationTable;

use crate::{
    env::{self, Env},
    rename,
    tast::{self, TypeVar},
};

pub fn check_file(ast: ast::File) -> (tast::File, env::Env) {
    let mut env = env::Env::new();
    let ast = rename::Rename::default().rename_file(ast);
    collect_typedefs(&mut env, &ast);
    let mut typer = TypeInference::new();
    let mut typed_toplevel_tasts = vec![];
    for item in ast.toplevels.iter() {
        if let ast::Item::Fn(f) = item {
            let mut vars = im::HashMap::<Lident, tast::Ty>::new();
            for (name, ty) in f.params.iter() {
                let ty = ast_ty_to_tast_ty_with_tparams_env(ty, &f.generics);
                vars.insert(name.clone(), ty);
            }
            let new_params = f
                .params
                .iter()
                .map(|(name, ty)| {
                    (
                        name.0.clone(),
                        ast_ty_to_tast_ty_with_tparams_env(ty, &f.generics),
                    )
                })
                .collect::<Vec<_>>();

            let ret_ty = match &f.ret_ty {
                Some(ty) => ast_ty_to_tast_ty_with_tparams_env(ty, &f.generics),
                None => tast::Ty::TUnit,
            };

            let typed_body = typer.infer(&env, &vars, &f.body);
            let typed_body = typer.subst(typed_body);
            typed_toplevel_tasts.push(tast::Fn {
                name: f.name.0.clone(),
                params: new_params,
                ret_ty,
                body: typed_body,
            });
        }
    }

    (
        tast::File {
            toplevels: typed_toplevel_tasts,
        },
        env,
    )
}

fn collect_typedefs(env: &mut Env, ast: &ast::File) {
    for item in ast.toplevels.iter() {
        match item {
            ast::Item::EnumDef(enum_def) => {
                let name = enum_def.name.clone();
                let params_env = &enum_def.generics;

                let variants = enum_def
                    .variants
                    .iter()
                    .map(|(vcon, typs)| {
                        let typs = typs
                            .iter()
                            .map(|ast_ty| ast_ty_to_tast_ty_with_tparams_env(ast_ty, params_env))
                            .collect();
                        (vcon.clone(), typs)
                    })
                    .collect();
                env.enums.insert(
                    name.clone(),
                    env::EnumDef {
                        name,
                        generics: enum_def.generics.clone(),
                        variants,
                    },
                );
            }
            ast::Item::Fn(func) => {
                let name = func.name.clone();
                let args = func
                    .params
                    .iter()
                    .map(|(_, ty)| ast_ty_to_tast_ty_with_tparams_env(ty, &func.generics))
                    .collect();
                let ret = match &func.ret_ty {
                    Some(ty) => ast_ty_to_tast_ty_with_tparams_env(ty, &func.generics),
                    None => tast::Ty::TUnit,
                };
                env.funcs.insert(
                    name.clone(),
                    tast::Ty::TFunc {
                        params: args,
                        ret_ty: Box::new(ret),
                    },
                );
            }
        }
    }
}

pub fn ast_ty_to_tast_ty_with_tparams_env(
    ast_ty: &ast::Ty,
    tparams_env: &[ast::Uident],
) -> tast::Ty {
    match ast_ty {
        ast::Ty::TUnit => tast::Ty::TUnit,
        ast::Ty::TBool => tast::Ty::TBool,
        ast::Ty::TInt => tast::Ty::TInt,
        ast::Ty::TTuple { typs } => {
            let typs = typs
                .iter()
                .map(|ty| ast_ty_to_tast_ty_with_tparams_env(ty, tparams_env))
                .collect();
            tast::Ty::TTuple { typs }
        }
        ast::Ty::TApp { name, args } => {
            if tparams_env.contains(name) {
                return tast::Ty::TParam {
                    name: name.0.clone(),
                };
            }
            tast::Ty::TApp {
                name: name.clone(),
                args: args
                    .iter()
                    .map(|ty| ast_ty_to_tast_ty_with_tparams_env(ty, tparams_env))
                    .collect(),
            }
        }
        ast::Ty::TFunc { params, ret_ty } => tast::Ty::TFunc {
            params: params
                .iter()
                .map(|ty| ast_ty_to_tast_ty_with_tparams_env(ty, tparams_env))
                .collect(),
            ret_ty: Box::new(ast_ty_to_tast_ty_with_tparams_env(ret_ty, tparams_env)),
        },
    }
}

pub struct TypeInference {
    pub uni: InPlaceUnificationTable<TypeVar>,
}

impl Default for TypeInference {
    fn default() -> Self {
        Self::new()
    }
}

fn occurs(var: TypeVar, ty: &tast::Ty) {
    match ty {
        tast::Ty::TVar(v) => {
            if var == *v {
                panic!("occurs check failed: {:?} occurs in {:?}", var, ty);
            }
        }
        tast::Ty::TUnit => {}
        tast::Ty::TBool => {}
        tast::Ty::TInt => {}
        tast::Ty::TString => {}
        tast::Ty::TTuple { typs } => {
            for ty in typs.iter() {
                occurs(var, ty);
            }
        }
        tast::Ty::TApp { .. } => {}
        tast::Ty::TFunc { params, ret_ty } => {
            for param in params.iter() {
                occurs(var, param);
            }
            occurs(var, ret_ty);
        }
        tast::Ty::TParam { .. } => {}
    }
}

impl TypeInference {
    fn norm(&mut self, ty: &tast::Ty) -> tast::Ty {
        match ty {
            tast::Ty::TVar(v) => {
                if let Some(value) = self.uni.probe_value(*v) {
                    self.norm(&value)
                } else {
                    tast::Ty::TVar(self.uni.find(*v))
                }
            }
            tast::Ty::TUnit => tast::Ty::TUnit,
            tast::Ty::TBool => tast::Ty::TBool,
            tast::Ty::TInt => tast::Ty::TInt,
            tast::Ty::TString => tast::Ty::TString,
            tast::Ty::TTuple { typs } => {
                let typs = typs.iter().map(|ty| self.norm(ty)).collect();
                tast::Ty::TTuple { typs }
            }
            tast::Ty::TApp { name, args } => tast::Ty::TApp {
                name: name.clone(),
                args: args.iter().map(|ty| self.norm(ty)).collect(),
            },
            tast::Ty::TFunc { params, ret_ty } => {
                let params = params.iter().map(|ty| self.norm(ty)).collect();
                let ret_ty = Box::new(self.norm(ret_ty));
                tast::Ty::TFunc { params, ret_ty }
            }
            tast::Ty::TParam { name } => tast::Ty::TParam { name: name.clone() },
        }
    }

    fn unify(&mut self, l: &tast::Ty, r: &tast::Ty) {
        let l_norm = self.norm(l);
        let r_norm = self.norm(r);
        match (&l_norm, &r_norm) {
            (tast::Ty::TVar(a), tast::Ty::TVar(b)) => {
                self.uni.unify_var_var(*a, *b).unwrap_or_else(|_| {
                    panic!("Failed to unify type variables {:?} and {:?}", a, b)
                });
            }
            (tast::Ty::TVar(a), t) | (t, tast::Ty::TVar(a)) => {
                occurs(*a, t);
                self.uni
                    .unify_var_value(*a, Some(t.clone()))
                    .unwrap_or_else(|_| {
                        panic!("Failed to unify type variable {:?} with {:?}", a, t)
                    });
            }

            (tast::Ty::TUnit, tast::Ty::TUnit) => {}
            (tast::Ty::TBool, tast::Ty::TBool) => {}
            (tast::Ty::TInt, tast::Ty::TInt) => {}
            (tast::Ty::TString, tast::Ty::TString) => {}
            (tast::Ty::TTuple { typs: typs1 }, tast::Ty::TTuple { typs: typs2 }) => {
                if typs1.len() != typs2.len() {
                    panic!("Tuple types have different lengths: {:?} and {:?}", l, r);
                }
                for (ty1, ty2) in typs1.iter().zip(typs2.iter()) {
                    self.unify(ty1, ty2);
                }
            }
            (
                tast::Ty::TFunc {
                    params: param1,
                    ret_ty: ret_ty1,
                },
                tast::Ty::TFunc {
                    params: param2,
                    ret_ty: ret_ty2,
                },
            ) => {
                if param1.len() != param2.len() {
                    panic!(
                        "Function types have different parameter lengths: {:?} and {:?}",
                        l, r
                    );
                }
                for (p1, p2) in param1.iter().zip(param2.iter()) {
                    self.unify(p1, p2);
                }
                self.unify(ret_ty1, ret_ty2);
            }
            (
                tast::Ty::TApp {
                    name: name1,
                    args: args1,
                },
                tast::Ty::TApp {
                    name: name2,
                    args: args2,
                },
            ) => {
                if name1 != name2 {
                    panic!("Constructor types are different: {:?} and {:?}", l, r);
                }
                for (arg1, arg2) in args1.iter().zip(args2.iter()) {
                    self.unify(arg1, arg2);
                }
            }
            _ => {
                panic!("type not equal {:?} and {:?}", l_norm, r_norm);
            }
        }
    }
}

impl TypeInference {
    pub fn new() -> Self {
        Self {
            uni: InPlaceUnificationTable::new(),
        }
    }

    fn fresh_ty_var(&mut self) -> tast::Ty {
        tast::Ty::TVar(self.uni.new_key(None))
    }

    fn inst_ty(&mut self, ty: &tast::Ty) -> tast::Ty {
        let mut subst: HashMap<String, tast::Ty> = HashMap::new();
        self._go_inst_ty(ty, &mut subst)
    }

    fn _go_inst_ty(&mut self, ty: &tast::Ty, subst: &mut HashMap<String, tast::Ty>) -> tast::Ty {
        match ty {
            tast::Ty::TVar(_) => ty.clone(),
            tast::Ty::TUnit => ty.clone(),
            tast::Ty::TBool => ty.clone(),
            tast::Ty::TInt => ty.clone(),
            tast::Ty::TString => ty.clone(),
            tast::Ty::TTuple { typs } => {
                let typs = typs
                    .iter()
                    .map(|ty| self._go_inst_ty(ty, subst))
                    .collect::<Vec<_>>();
                tast::Ty::TTuple { typs }
            }
            tast::Ty::TApp { name, args } => {
                let args = args
                    .iter()
                    .map(|arg| self._go_inst_ty(arg, subst))
                    .collect::<Vec<_>>();
                tast::Ty::TApp {
                    name: name.clone(),
                    args,
                }
            }
            tast::Ty::TParam { name } => {
                if subst.contains_key(name) {
                    let ty = subst.get(name).unwrap();
                    ty.clone()
                } else {
                    let new_ty = self.fresh_ty_var();
                    subst.insert(name.clone(), new_ty.clone());
                    new_ty
                }
            }
            tast::Ty::TFunc { params, ret_ty } => {
                let params = params
                    .iter()
                    .map(|ty| self._go_inst_ty(ty, subst))
                    .collect::<Vec<_>>();
                let ret_ty = Box::new(self._go_inst_ty(ret_ty, subst));
                tast::Ty::TFunc { params, ret_ty }
            }
        }
    }

    fn subst_ty(&mut self, ty: &tast::Ty) -> tast::Ty {
        match ty {
            tast::Ty::TVar(v) => {
                if let Some(value) = self.uni.probe_value(*v) {
                    self.subst_ty(&value)
                } else {
                    panic!("Type variable {:?} not resolved", v);
                }
            }
            tast::Ty::TUnit => tast::Ty::TUnit,
            tast::Ty::TBool => tast::Ty::TBool,
            tast::Ty::TInt => tast::Ty::TInt,
            tast::Ty::TString => tast::Ty::TString,
            tast::Ty::TTuple { typs } => {
                let typs = typs.iter().map(|ty| self.subst_ty(ty)).collect();
                tast::Ty::TTuple { typs }
            }
            tast::Ty::TApp { name, args } => tast::Ty::TApp {
                name: name.clone(),
                args: args.iter().map(|arg| self.subst_ty(arg)).collect(),
            },
            tast::Ty::TFunc { params, ret_ty } => {
                let params = params.iter().map(|ty| self.subst_ty(ty)).collect();
                let ret_ty = Box::new(self.subst_ty(ret_ty));
                tast::Ty::TFunc { params, ret_ty }
            }
            tast::Ty::TParam { name } => tast::Ty::TParam { name: name.clone() },
        }
    }

    fn subst_pat(&mut self, p: tast::Pat) -> tast::Pat {
        match p {
            tast::Pat::PVar { name, ty, astptr } => {
                let ty = self.subst_ty(&ty);
                tast::Pat::PVar {
                    name: name.clone(),
                    ty: ty.clone(),
                    astptr,
                }
            }
            tast::Pat::PUnit { ty } => {
                let ty = self.subst_ty(&ty);
                tast::Pat::PUnit { ty: ty.clone() }
            }
            tast::Pat::PBool { value, ty } => {
                let ty = self.subst_ty(&ty);
                tast::Pat::PBool {
                    value,
                    ty: ty.clone(),
                }
            }
            tast::Pat::PConstr { index, args, ty } => {
                let ty = self.subst_ty(&ty);
                let args = args
                    .into_iter()
                    .map(|arg| self.subst_pat(arg))
                    .collect::<Vec<_>>();
                tast::Pat::PConstr {
                    index,
                    args,
                    ty: ty.clone(),
                }
            }
            tast::Pat::PTuple { items, ty } => {
                let ty = self.subst_ty(&ty);
                let items = items
                    .into_iter()
                    .map(|item| self.subst_pat(item))
                    .collect::<Vec<_>>();
                tast::Pat::PTuple {
                    items,
                    ty: ty.clone(),
                }
            }
            tast::Pat::PWild { ty } => {
                let ty = self.subst_ty(&ty);
                tast::Pat::PWild { ty: ty.clone() }
            }
        }
    }

    pub fn subst(&mut self, e: tast::Expr) -> tast::Expr {
        match e {
            tast::Expr::EVar { name, ty, astptr } => {
                let ty = self.subst_ty(&ty);
                tast::Expr::EVar {
                    name,
                    ty: ty.clone(),
                    astptr,
                }
            }
            tast::Expr::EUnit { ty } => {
                let ty = self.subst_ty(&ty);
                tast::Expr::EUnit { ty: ty.clone() }
            }
            tast::Expr::EBool { value, ty } => {
                let ty = self.subst_ty(&ty);
                tast::Expr::EBool {
                    value,
                    ty: ty.clone(),
                }
            }
            tast::Expr::EInt { value, ty } => {
                let ty = self.subst_ty(&ty);
                tast::Expr::EInt {
                    value,
                    ty: ty.clone(),
                }
            }
            tast::Expr::EString { value, ty } => {
                let ty = self.subst_ty(&ty);
                tast::Expr::EString {
                    value: value.clone(),
                    ty: ty.clone(),
                }
            }
            tast::Expr::EConstr { index, args, ty } => {
                let ty = self.subst_ty(&ty);
                let args = args
                    .into_iter()
                    .map(|arg| self.subst(arg))
                    .collect::<Vec<_>>();
                tast::Expr::EConstr { index, args, ty }
            }
            tast::Expr::ETuple { items, ty } => {
                let ty = self.subst_ty(&ty);
                let items = items
                    .into_iter()
                    .map(|item| self.subst(item))
                    .collect::<Vec<_>>();
                tast::Expr::ETuple {
                    items,
                    ty: ty.clone(),
                }
            }
            tast::Expr::ELet {
                pat,
                value,
                body,
                ty,
            } => {
                let ty = self.subst_ty(&ty);
                let pat = self.subst_pat(pat);
                let value = Box::new(self.subst(*value));
                let body = Box::new(self.subst(*body));
                tast::Expr::ELet {
                    pat,
                    value,
                    body,
                    ty: ty.clone(),
                }
            }
            tast::Expr::EMatch { expr, arms, ty } => {
                let ty = self.subst_ty(&ty);
                let expr = Box::new(self.subst(*expr));
                let arms = arms
                    .into_iter()
                    .map(|arm| tast::Arm {
                        pat: self.subst_pat(arm.pat),
                        body: self.subst(arm.body),
                    })
                    .collect::<Vec<_>>();
                tast::Expr::EMatch {
                    expr,
                    arms,
                    ty: ty.clone(),
                }
            }
            tast::Expr::ECall { func, args, ty } => {
                let ty = self.subst_ty(&ty);
                let args = args
                    .into_iter()
                    .map(|arg| self.subst(arg))
                    .collect::<Vec<_>>();
                tast::Expr::ECall {
                    func,
                    args,
                    ty: ty.clone(),
                }
            }
            tast::Expr::EProj { tuple, index, ty } => {
                let ty = self.subst_ty(&ty);
                let tuple = Box::new(self.subst(*tuple));
                tast::Expr::EProj {
                    tuple,
                    index,
                    ty: ty.clone(),
                }
            }
        }
    }

    pub fn infer(
        &mut self,
        env: &Env,
        vars: &im::HashMap<Lident, tast::Ty>,
        e: &ast::Expr,
    ) -> tast::Expr {
        match e {
            ast::Expr::EVar { name, astptr } => {
                let ty = vars
                    .get(name)
                    .unwrap_or_else(|| panic!("Variable {} not found in environment", name.0));
                tast::Expr::EVar {
                    name: name.0.clone(),
                    ty: ty.clone(),
                    astptr: Some(*astptr),
                }
            }
            ast::Expr::EUnit => tast::Expr::EUnit {
                ty: tast::Ty::TUnit,
            },
            ast::Expr::EBool { value } => tast::Expr::EBool {
                value: *value,
                ty: tast::Ty::TBool,
            },
            ast::Expr::EInt { value } => tast::Expr::EInt {
                value: *value,
                ty: tast::Ty::TInt,
            },
            ast::Expr::EString { value } => tast::Expr::EString {
                value: value.clone(),
                ty: tast::Ty::TString,
            },
            ast::Expr::EConstr { vcon, args } => {
                let constr_ty = env
                    .get_type_of_constructor(&vcon.0)
                    .unwrap_or_else(|| panic!("Constructor {} not found in environment", vcon.0));
                let inst_constr_ty = self.inst_ty(&constr_ty);

                let index = env.get_index_of_constructor(&vcon.0).unwrap();

                let ret_ty = self.fresh_ty_var();
                let mut args_tast = Vec::new();
                for arg in args.iter() {
                    let arg_tast = self.infer(env, vars, arg);
                    args_tast.push(arg_tast.clone());
                }

                if !args.is_empty() {
                    let actual_ty = tast::Ty::TFunc {
                        params: args_tast.iter().map(|arg| arg.get_ty()).collect(),
                        ret_ty: Box::new(ret_ty.clone()),
                    };
                    self.unify(&inst_constr_ty, &actual_ty);
                } else {
                    self.unify(&inst_constr_ty, &ret_ty);
                }

                tast::Expr::EConstr {
                    index: index as usize,
                    args: args_tast,
                    ty: ret_ty,
                }
            }
            ast::Expr::ETuple { items } => {
                let mut typs = Vec::new();
                let mut items_tast = Vec::new();
                for item in items.iter() {
                    let item_tast = self.infer(env, vars, item);
                    items_tast.push(item_tast.clone());
                    typs.push(item_tast.get_ty());
                }
                tast::Expr::ETuple {
                    items: items_tast,
                    ty: tast::Ty::TTuple { typs },
                }
            }
            ast::Expr::ELet { pat, value, body } => {
                let value_tast = self.infer(env, vars, value);
                let value_ty = value_tast.get_ty();

                let mut new_vars = vars.clone();
                let pat_tast = self.check_pat(env, &mut new_vars, pat, &value_ty);

                let body_tast = self.infer(env, &new_vars, body);
                let body_ty = body_tast.get_ty();
                tast::Expr::ELet {
                    pat: pat_tast,
                    value: Box::new(value_tast),
                    body: Box::new(body_tast),
                    ty: body_ty.clone(),
                }
            }
            ast::Expr::EMatch { expr, arms } => {
                let expr_tast = self.infer(env, vars, expr);
                let expr_ty = expr_tast.get_ty();

                let mut arms_tast = Vec::new();
                let arm_ty = self.fresh_ty_var();
                for arm in arms.iter() {
                    let mut new_vars = vars.clone();
                    let arm_tast = self.check_pat(env, &mut new_vars, &arm.pat, &expr_ty);
                    let arm_body_tast = self.infer(env, &new_vars, &arm.body);
                    self.unify(&arm_body_tast.get_ty(), &arm_ty);
                    arms_tast.push(tast::Arm {
                        pat: arm_tast,
                        body: arm_body_tast,
                    });
                }
                tast::Expr::EMatch {
                    expr: Box::new(expr_tast),
                    arms: arms_tast,
                    ty: arm_ty,
                }
            }
            ast::Expr::ECall { func, args } => {
                let f = &func.0;
                match f.as_str() {
                    "unit_to_string" => {
                        if args.len() != 1 {
                            panic!("unit_to_string takes exactly one argument");
                        }
                        let arg0_tast = self.check(env, vars, &args[0], &tast::Ty::TUnit);
                        tast::Expr::ECall {
                            func: func.0.clone(),
                            args: vec![arg0_tast],
                            ty: tast::Ty::TString,
                        }
                    }
                    "bool_to_string" => {
                        if args.len() != 1 {
                            panic!("bool_to_string takes exactly one argument");
                        }
                        let arg0_tast = self.check(env, vars, &args[0], &tast::Ty::TBool);
                        tast::Expr::ECall {
                            func: func.0.clone(),
                            args: vec![arg0_tast],
                            ty: tast::Ty::TString,
                        }
                    }
                    "int_to_string" => {
                        if args.len() != 1 {
                            panic!("int_to_string takes exactly one argument");
                        }
                        let arg0_tast = self.check(env, vars, &args[0], &tast::Ty::TInt);
                        tast::Expr::ECall {
                            func: func.0.clone(),
                            args: vec![arg0_tast],
                            ty: tast::Ty::TString,
                        }
                    }
                    "int_add" => {
                        if args.len() != 2 {
                            panic!("int_add takes exactly two arguments");
                        }
                        let arg0_tast = self.check(env, vars, &args[0], &tast::Ty::TInt);
                        let arg1_tast = self.check(env, vars, &args[1], &tast::Ty::TInt);
                        tast::Expr::ECall {
                            func: func.0.clone(),
                            args: vec![arg0_tast, arg1_tast],
                            ty: tast::Ty::TInt,
                        }
                    }
                    "int_sub" => {
                        if args.len() != 2 {
                            panic!("int_sub takes exactly two arguments");
                        }
                        let arg0_tast = self.check(env, vars, &args[0], &tast::Ty::TInt);
                        let arg1_tast = self.check(env, vars, &args[1], &tast::Ty::TInt);
                        tast::Expr::ECall {
                            func: func.0.clone(),
                            args: vec![arg0_tast, arg1_tast],
                            ty: tast::Ty::TInt,
                        }
                    }
                    "int_less" => {
                        if args.len() != 2 {
                            panic!("int_less takes exactly two arguments");
                        }
                        let arg0_tast = self.check(env, vars, &args[0], &tast::Ty::TInt);
                        let arg1_tast = self.check(env, vars, &args[1], &tast::Ty::TInt);
                        tast::Expr::ECall {
                            func: func.0.clone(),
                            args: vec![arg0_tast, arg1_tast],
                            ty: tast::Ty::TBool,
                        }
                    }
                    "print" | "println" => {
                        if args.len() != 1 {
                            panic!("print/println takes exactly one argument");
                        }
                        let arg0_tast = self.check(env, vars, &args[0], &tast::Ty::TString);
                        tast::Expr::ECall {
                            func: func.0.clone(),
                            args: vec![arg0_tast],
                            ty: tast::Ty::TUnit,
                        }
                    }
                    _ => {
                        let mut args_tast = Vec::new();
                        for arg in args.iter() {
                            let arg_tast = self.infer(env, vars, arg);
                            args_tast.push(arg_tast.clone());
                        }
                        let expected_args_ty = env.get_args_ty_of_function(f);
                        if args.len() != expected_args_ty.len() {
                            panic!(
                                "Function {} expects {} arguments, but got {}",
                                f,
                                expected_args_ty.len(),
                                args.len()
                            );
                        }
                        for (arg, expected_ty) in args_tast.iter().zip(expected_args_ty.iter()) {
                            self.unify(&arg.get_ty(), expected_ty);
                        }
                        tast::Expr::ECall {
                            func: func.0.clone(),
                            args: args_tast,
                            ty: env.get_ret_ty_of_function(&func.0),
                        }
                    }
                }
            }
            ast::Expr::EProj { tuple, index } => {
                let tuple_tast = self.infer(env, vars, tuple);
                let tuple_ty = tuple_tast.get_ty();
                let index = match index.as_ref() {
                    ast::Expr::EInt { value } => *value,
                    _ => panic!("Expected an integer for tuple index"),
                };
                let tuple_ty = self.norm(&tuple_ty);
                match tuple_ty {
                    tast::Ty::TTuple { typs } => {
                        let index = index as usize;
                        let ty = typs[index].clone();
                        tast::Expr::EProj {
                            tuple: Box::new(tuple_tast),
                            index,
                            ty,
                        }
                    }
                    _ => {
                        panic!("Expected a tuple type for projection");
                    }
                }
            }
        }
    }

    fn check(
        &mut self,
        env: &Env,
        vars: &im::HashMap<Lident, tast::Ty>,
        e: &ast::Expr,
        ty: &tast::Ty,
    ) -> tast::Expr {
        match e {
            ast::Expr::EVar { name, astptr } => {
                let tast = self.infer(env, vars, e);
                self.unify(&tast.get_ty(), ty);
                tast::Expr::EVar {
                    name: name.0.clone(),
                    ty: tast.get_ty(),
                    astptr: Some(*astptr),
                }
            }
            ast::Expr::EUnit => {
                let tast = self.infer(env, vars, e);
                self.unify(&tast.get_ty(), ty);
                tast
            }
            ast::Expr::EBool { .. } => {
                let tast = self.infer(env, vars, e);
                self.unify(&tast.get_ty(), ty);
                tast
            }
            ast::Expr::EInt { value: _ } => {
                let tast = self.infer(env, vars, e);
                self.unify(&tast.get_ty(), ty);
                tast
            }
            ast::Expr::EString { value: _ } => {
                let tast = self.infer(env, vars, e);
                self.unify(&tast.get_ty(), ty);
                tast
            }
            ast::Expr::EConstr { .. } => {
                let tast = self.infer(env, vars, e);
                self.unify(&tast.get_ty(), ty);
                tast
            }
            ast::Expr::ETuple { .. } => {
                let tast = self.infer(env, vars, e);
                self.unify(&tast.get_ty(), ty);
                tast
            }
            ast::Expr::ELet { .. } => {
                let tast = self.infer(env, vars, e);
                self.unify(&tast.get_ty(), ty);
                tast
            }
            ast::Expr::EMatch { .. } => {
                let tast = self.infer(env, vars, e);
                self.unify(&tast.get_ty(), ty);
                tast
            }
            ast::Expr::ECall { .. } => {
                let tast = self.infer(env, vars, e);
                self.unify(&tast.get_ty(), ty);
                tast
            }
            ast::Expr::EProj { .. } => {
                let tast = self.infer(env, vars, e);
                self.unify(&tast.get_ty(), ty);
                tast
            }
        }
    }

    fn check_pat(
        &mut self,
        env: &Env,
        vars: &mut im::HashMap<Lident, tast::Ty>,
        pat: &ast::Pat,
        ty: &tast::Ty,
    ) -> tast::Pat {
        match pat {
            ast::Pat::PVar { name, astptr } => {
                vars.insert(name.clone(), ty.clone());
                tast::Pat::PVar {
                    name: name.0.clone(),
                    ty: ty.clone(),
                    astptr: Some(*astptr),
                }
            }
            ast::Pat::PUnit => tast::Pat::PUnit {
                ty: tast::Ty::TUnit,
            },
            ast::Pat::PBool { value } => tast::Pat::PBool {
                value: *value,
                ty: tast::Ty::TBool,
            },
            ast::Pat::PConstr { .. } => {
                let tast = self.infer_pat(env, vars, pat);
                self.unify(&tast.get_ty(), ty);
                tast
            }
            ast::Pat::PTuple { pats } => {
                let mut pats_tast = Vec::new();
                let mut pat_typs = Vec::new();
                for pat in pats.iter() {
                    let pat_tast = self.infer_pat(env, vars, pat);
                    pat_typs.push(pat_tast.get_ty());
                    pats_tast.push(pat_tast);
                }
                let pat_ty = tast::Ty::TTuple { typs: pat_typs };
                self.unify(&pat_ty, ty);
                tast::Pat::PTuple {
                    items: pats_tast,
                    ty: pat_ty,
                }
            }
            ast::Pat::PWild => {
                let pat_ty = self.fresh_ty_var();
                self.unify(&pat_ty, ty);
                tast::Pat::PWild { ty: pat_ty }
            }
        }
    }

    fn infer_pat(
        &mut self,
        env: &Env,
        vars: &mut im::HashMap<Lident, tast::Ty>,
        pat: &ast::Pat,
    ) -> tast::Pat {
        match pat {
            ast::Pat::PVar { name, astptr } => {
                let pat_ty = match vars.get(name) {
                    Some(ty) => ty.clone(),
                    None => {
                        let newvar = self.fresh_ty_var();
                        vars.insert(name.clone(), newvar.clone());
                        newvar
                    }
                };
                tast::Pat::PVar {
                    name: name.0.clone(),
                    ty: pat_ty.clone(),
                    astptr: Some(*astptr),
                }
            }
            ast::Pat::PConstr { vcon, args } => {
                let constr_ty = env
                    .get_type_of_constructor(&vcon.0)
                    .unwrap_or_else(|| panic!("Constructor {} not found in environment", vcon.0));
                let inst_constr_ty = self.inst_ty(&constr_ty);

                let index = env.get_index_of_constructor(&vcon.0).unwrap();

                let ret_ty = self.fresh_ty_var();
                let mut args_tast = Vec::new();
                for arg in args.iter() {
                    let arg_tast = self.infer_pat(env, vars, arg);
                    args_tast.push(arg_tast.clone());
                }

                if !args.is_empty() {
                    let actual_ty = tast::Ty::TFunc {
                        params: args_tast.iter().map(|arg| arg.get_ty()).collect(),
                        ret_ty: Box::new(ret_ty.clone()),
                    };
                    self.unify(&inst_constr_ty, &actual_ty);
                } else {
                    self.unify(&inst_constr_ty, &ret_ty);
                }

                tast::Pat::PConstr {
                    index: index as usize,
                    args: args_tast,
                    ty: ret_ty,
                }
            }
            ast::Pat::PTuple { pats } => {
                let mut pats_tast = Vec::new();
                let mut pat_typs = Vec::new();
                for pat in pats.iter() {
                    let pat_tast = self.infer_pat(env, vars, pat);
                    pat_typs.push(pat_tast.get_ty());
                    pats_tast.push(pat_tast);
                }
                tast::Pat::PTuple {
                    items: pats_tast,
                    ty: tast::Ty::TTuple { typs: pat_typs },
                }
            }
            ast::Pat::PUnit => tast::Pat::PUnit {
                ty: tast::Ty::TUnit,
            },
            ast::Pat::PBool { value } => tast::Pat::PBool {
                value: *value,
                ty: tast::Ty::TBool,
            },
            ast::Pat::PWild => {
                let pat_ty = self.fresh_ty_var();
                tast::Pat::PWild { ty: pat_ty }
            }
        }
    }
}
