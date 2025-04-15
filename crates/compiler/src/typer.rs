use ::ast::ast::Lident;
use ast::ast;
use ena::unify::InPlaceUnificationTable;

use crate::{
    env::{self, Env},
    tast::{self, TypeVar},
};

pub fn check_file(ast: ast::File) -> (tast::File, env::Env) {
    let mut env = env::Env::new();
    collect_typedefs(&mut env, &ast);
    let mut typer = TypeInference::new();
    let mut typed_toplevel_tasts = vec![];
    for item in ast.toplevels.iter() {
        if let ast::Item::Fn(f) = item {
            let mut vars = im::HashMap::<Lident, tast::Ty>::new();
            for (name, ty) in f.params.iter() {
                let ty = ast_ty_to_tast_ty(ty);
                vars.insert(name.clone(), ty);
            }
            let new_params = f
                .params
                .iter()
                .map(|(name, ty)| (name.0.clone(), ast_ty_to_tast_ty(ty)))
                .collect::<Vec<_>>();

            let ret_ty = match &f.ret_ty {
                Some(ty) => ast_ty_to_tast_ty(ty),
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
                let variants = enum_def
                    .variants
                    .iter()
                    .map(|(vcon, typs)| {
                        let typs = typs.iter().map(ast_ty_to_tast_ty).collect();
                        (vcon.clone(), typs)
                    })
                    .collect();
                env.enums
                    .insert(name.clone(), env::EnumDef { name, variants });
            }
            ast::Item::Fn(func) => {
                let name = func.name.clone();
                let args = func
                    .params
                    .iter()
                    .map(|(_, ty)| ast_ty_to_tast_ty(ty))
                    .collect();
                let ret = match &func.ret_ty {
                    Some(ty) => ast_ty_to_tast_ty(ty),
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
            ast::Item::Expr(..) => continue,
        }
    }
}

pub fn ast_ty_to_tast_ty(ast_ty: &ast::Ty) -> tast::Ty {
    match ast_ty {
        ast::Ty::TUnit => tast::Ty::TUnit,
        ast::Ty::TBool => tast::Ty::TBool,
        ast::Ty::TInt => tast::Ty::TInt,
        ast::Ty::TTuple { typs } => {
            let typs = typs.iter().map(ast_ty_to_tast_ty).collect();
            tast::Ty::TTuple { typs }
        }
        ast::Ty::TEnum { name } => tast::Ty::TEnum { name: name.clone() },
        ast::Ty::TFunc { params, ret_ty } => tast::Ty::TFunc {
            params: params.iter().map(ast_ty_to_tast_ty).collect(),
            ret_ty: Box::new(ast_ty_to_tast_ty(ret_ty)),
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
        tast::Ty::TTuple { typs } => {
            for ty in typs.iter() {
                occurs(var, ty);
            }
        }
        tast::Ty::TEnum { .. } => {}
        tast::Ty::TFunc { params, ret_ty } => {
            for param in params.iter() {
                occurs(var, param);
            }
            occurs(var, ret_ty);
        }
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
            tast::Ty::TTuple { typs } => {
                let typs = typs.iter().map(|ty| self.norm(ty)).collect();
                tast::Ty::TTuple { typs }
            }
            tast::Ty::TEnum { name } => tast::Ty::TEnum { name: name.clone() },
            tast::Ty::TFunc { params, ret_ty } => {
                let params = params.iter().map(|ty| self.norm(ty)).collect();
                let ret_ty = Box::new(self.norm(ret_ty));
                tast::Ty::TFunc { params, ret_ty }
            }
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
            (tast::Ty::TTuple { typs: typs1 }, tast::Ty::TTuple { typs: typs2 }) => {
                if typs1.len() != typs2.len() {
                    panic!("Tuple types have different lengths: {:?} and {:?}", l, r);
                }
                for (ty1, ty2) in typs1.iter().zip(typs2.iter()) {
                    self.unify(ty1, ty2);
                }
            }
            (tast::Ty::TEnum { name: name1 }, tast::Ty::TEnum { name: name2 }) => {
                if name1 != name2 {
                    panic!("Constructor types are different: {:?} and {:?}", l, r);
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
            tast::Ty::TTuple { typs } => {
                let typs = typs.iter().map(|ty| self.subst_ty(ty)).collect();
                tast::Ty::TTuple { typs }
            }
            tast::Ty::TEnum { name } => tast::Ty::TEnum { name: name.clone() },
            tast::Ty::TFunc { params, ret_ty } => {
                let params = params.iter().map(|ty| self.subst_ty(ty)).collect();
                let ret_ty = Box::new(self.subst_ty(ret_ty));
                tast::Ty::TFunc { params, ret_ty }
            }
        }
    }

    fn subst_pat(&mut self, p: tast::Pat) -> tast::Pat {
        match p {
            tast::Pat::PVar { name, ty } => {
                let ty = self.subst_ty(&ty);
                tast::Pat::PVar {
                    name: name.clone(),
                    ty: ty.clone(),
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
            tast::Expr::EVar { name, ty } => {
                let ty = self.subst_ty(&ty);
                tast::Expr::EVar {
                    name,
                    ty: ty.clone(),
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
            ast::Expr::EVar { name } => {
                let ty = vars
                    .get(name)
                    .unwrap_or_else(|| panic!("Variable {} not found in environment", name.0));
                tast::Expr::EVar {
                    name: name.0.clone(),
                    ty: ty.clone(),
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
            ast::Expr::EConstr { vcon, args } => {
                let ty = env
                    .get_type_of_constructor(&vcon.0)
                    .unwrap_or_else(|| panic!("Constructor {} not found in environment", vcon.0));
                let expected_args_ty = env.get_args_ty_of_constructor(&vcon.0);
                let index = env.get_index_of_constructor(&vcon.0).unwrap();
                if args.len() != expected_args_ty.len() {
                    panic!(
                        "Constructor {} expects {} arguments, but got {}",
                        vcon.0,
                        expected_args_ty.len(),
                        args.len()
                    );
                }
                let mut args_tast = Vec::new();
                for (arg, expected_ty) in args.iter().zip(expected_args_ty.iter()) {
                    let arg_tast = self.check(env, vars, arg, expected_ty);
                    args_tast.push(arg_tast.clone());
                }
                tast::Expr::EConstr {
                    index: index as usize,
                    args: args_tast,
                    ty,
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
                    "print_unit" => {
                        if args.len() != 1 {
                            panic!("print_unit takes exactly one argument");
                        }
                        let arg0_tast = self.check(env, vars, &args[0], &tast::Ty::TUnit);
                        tast::Expr::ECall {
                            func: func.0.clone(),
                            args: vec![arg0_tast],
                            ty: tast::Ty::TUnit,
                        }
                    }
                    "print_bool" => {
                        if args.len() != 1 {
                            panic!("print_unit takes exactly one argument");
                        }
                        let arg0_tast = self.check(env, vars, &args[0], &tast::Ty::TBool);
                        tast::Expr::ECall {
                            func: func.0.clone(),
                            args: vec![arg0_tast],
                            ty: tast::Ty::TUnit,
                        }
                    }
                    "print_int" => {
                        if args.len() != 1 {
                            panic!("print_unit takes exactly one argument");
                        }
                        let arg0_tast = self.check(env, vars, &args[0], &tast::Ty::TInt);
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
                        let func_ty = env
                            .get_type_of_function(f)
                            .unwrap_or_else(|| panic!("Function {} not found in environment", f));
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
                            ty: func_ty,
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
            ast::Expr::EVar { name } => {
                let tast = self.infer(env, vars, e);
                self.unify(&tast.get_ty(), ty);
                tast::Expr::EVar {
                    name: name.0.clone(),
                    ty: tast.get_ty(),
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
            ast::Pat::PVar { name } => {
                vars.insert(name.clone(), ty.clone());
                tast::Pat::PVar {
                    name: name.0.clone(),
                    ty: ty.clone(),
                }
            }
            ast::Pat::PUnit => tast::Pat::PUnit {
                ty: tast::Ty::TUnit,
            },
            ast::Pat::PBool { value } => tast::Pat::PBool {
                value: *value,
                ty: tast::Ty::TBool,
            },
            ast::Pat::PConstr { vcon, args } => {
                let pat_ty = self.fresh_ty_var();
                let constr_ty = env
                    .get_type_of_constructor(&vcon.0)
                    .unwrap_or_else(|| panic!("Constructor {} not found in environment", vcon.0));
                let expected_args_ty = env.get_args_ty_of_constructor(&vcon.0);
                if args.len() != expected_args_ty.len() {
                    panic!(
                        "Constructor {} expects {} arguments, but got {}",
                        vcon.0,
                        expected_args_ty.len(),
                        args.len()
                    );
                }
                let mut args_tast = Vec::new();
                for (arg, expected_ty) in args.iter().zip(expected_args_ty.iter()) {
                    let arg_tast = self.check_pat(env, vars, arg, expected_ty);
                    args_tast.push(arg_tast.clone());
                }
                let index = env.get_index_of_constructor(&vcon.0).unwrap();
                self.unify(&pat_ty, &constr_ty);
                tast::Pat::PConstr {
                    index: index as usize,
                    args: args_tast,
                    ty: pat_ty,
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
            ast::Pat::PVar { name } => {
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
                }
            }
            ast::Pat::PConstr { vcon, args } => {
                let variant_name = &vcon.0;
                let constr_ty = env
                    .get_type_of_constructor(variant_name)
                    .unwrap_or_else(|| {
                        panic!("Constructor {} not found in environment", variant_name)
                    });
                let expected_args_ty = env.get_args_ty_of_constructor(variant_name);
                if args.len() != expected_args_ty.len() {
                    panic!(
                        "Constructor {} expects {} arguments, but got {}",
                        variant_name,
                        expected_args_ty.len(),
                        args.len()
                    );
                }
                let mut args_tast = Vec::new();
                for (arg, expected_ty) in args.iter().zip(expected_args_ty.iter()) {
                    let arg_tast = self.check_pat(env, vars, arg, expected_ty);
                    args_tast.push(arg_tast.clone());
                }
                tast::Pat::PConstr {
                    index: env.get_index_of_constructor(variant_name).unwrap() as usize,
                    args: args_tast,
                    ty: constr_ty,
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
                unreachable!()
            }
        }
    }
}
