use ena::unify::InPlaceUnificationTable;

use crate::{
    ast,
    env::{self, Env},
    ident::Lident,
    tast::{self, TypeVar},
};

pub fn check_file(ast: ast::File) -> (tast::Expr, env::Env) {
    let mut env = env::Env::new();
    collect_typedefs(&mut env, &ast);
    let mut typer = TypeInference::new();
    let vars = im::HashMap::<Lident, TypeVar>::new();
    let tast = typer.infer(&env, &vars, &ast.expr);
    (tast, env)
}

fn collect_typedefs(env: &mut Env, ast: &ast::File) {
    for enum_def in ast.enum_defs.iter() {
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
}

pub fn ast_ty_to_tast_ty(ast_ty: &ast::Ty) -> tast::Ty {
    match ast_ty {
        ast::Ty::TUnit => tast::Ty::TUnit,
        ast::Ty::TBool => tast::Ty::TBool,
        ast::Ty::TTuple { typs } => {
            let typs = typs.iter().map(ast_ty_to_tast_ty).collect();
            tast::Ty::TTuple { typs }
        }
        ast::Ty::TConstr { name } => tast::Ty::TConstr { name: name.clone() },
    }
}

pub struct TypeInference {
    uni: InPlaceUnificationTable<TypeVar>,
}

impl Default for TypeInference {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeInference {
    pub fn new() -> Self {
        Self {
            uni: InPlaceUnificationTable::new(),
        }
    }

    fn fresh_ty_var(&mut self) -> TypeVar {
        self.uni.new_key(None)
    }

    pub fn infer(
        &mut self,
        env: &Env,
        vars: &im::HashMap<Lident, TypeVar>,
        e: &ast::Expr,
    ) -> tast::Expr {
        match e {
            ast::Expr::EVar { name } => {
                let a = self.uni.find(vars[name]);
                let ty = self.uni.probe_value(a).unwrap();

                tast::Expr::EVar {
                    name: name.0.clone(),
                    ty,
                }
            }
            ast::Expr::EUnit => tast::Expr::EUnit {
                ty: tast::Ty::TUnit,
            },
            ast::Expr::EBool { value } => tast::Expr::EBool {
                value: *value,
                ty: tast::Ty::TBool,
            },
            ast::Expr::EInt { value: _ } => todo!(),
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

                let body_tast = self.infer(env, &mut new_vars, body);
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
                    let arm_body_tast = self.infer(env, &mut new_vars, &arm.body);
                    self.uni
                        .unify_var_value(arm_ty, Some(arm_body_tast.get_ty()))
                        .unwrap_or_else(|_| {
                            panic!(
                                "Failed to unify arm type {:?} with body type {:?}",
                                arm_ty,
                                arm_body_tast.get_ty()
                            )
                        });
                    arms_tast.push(tast::Arm {
                        pat: arm_tast,
                        body: arm_body_tast,
                    });
                }
                tast::Expr::EMatch {
                    expr: Box::new(expr_tast),
                    arms: arms_tast,
                    ty: tast::Ty::TVar(arm_ty),
                }
            }
            ast::Expr::EPrim { func, args } => {
                let f = &func.0;
                match f.as_str() {
                    "print_unit" => {
                        if args.len() != 1 {
                            panic!("print_unit takes exactly one argument");
                        }
                        let arg_tast = self.check(env, vars, e, &tast::Ty::TUnit);
                        tast::Expr::EPrim {
                            func: func.0.clone(),
                            args: vec![arg_tast],
                            ty: tast::Ty::TUnit,
                        }
                    }
                    "print_bool" => {
                        if args.len() != 1 {
                            panic!("print_unit takes exactly one argument");
                        }
                        let arg_tast = self.check(env, vars, e, &tast::Ty::TBool);
                        tast::Expr::EPrim {
                            func: func.0.clone(),
                            args: vec![arg_tast],
                            ty: tast::Ty::TUnit,
                        }
                    }
                    _ => {
                        panic!("Unknown prim function: {}", f);
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
        vars: &im::HashMap<Lident, TypeVar>,
        e: &ast::Expr,
        ty: &tast::Ty,
    ) -> tast::Expr {
        match (e, ty) {
            (_, tast::Ty::TVar(ty_var)) => {
                let e_tast = self.infer(env, vars, e);
                let e_ty = e_tast.get_ty();
                self.uni.unify_var_value(*ty_var, Some(e_ty)).unwrap();
                e_tast
            }
            (ast::Expr::EVar { name }, _) => {
                let a = self.uni.find(vars[name]);
                self.uni.unify_var_value(a, Some(ty.clone())).unwrap();
                tast::Expr::EVar {
                    name: name.0.clone(),
                    ty: ty.clone(),
                }
            }
            (ast::Expr::EUnit, tast::Ty::TUnit) => tast::Expr::EUnit {
                ty: tast::Ty::TUnit,
            },
            (ast::Expr::EBool { value }, tast::Ty::TBool) => tast::Expr::EBool {
                value: *value,
                ty: tast::Ty::TBool,
            },
            (ast::Expr::EInt { value: _ }, _) => todo!(),
            (ast::Expr::EConstr { vcon: _, args: _ }, _) => todo!(),
            (ast::Expr::ETuple { items: _ }, _) => todo!(),
            (
                ast::Expr::ELet {
                    pat: _,
                    value: _,
                    body: _,
                },
                _,
            ) => todo!(),
            (ast::Expr::EMatch { expr: _, arms: _ }, _) => todo!(),
            (ast::Expr::EPrim { func: _, args: _ }, _) => todo!(),
            (ast::Expr::EProj { tuple: _, index: _ }, _) => todo!(),
            _ => {
                panic!("type mismatch: expected {:?}, found {:?}", ty, e)
            }
        }
    }

    fn check_pat(
        &mut self,
        env: &Env,
        vars: &mut im::HashMap<Lident, TypeVar>,
        pat: &ast::Pat,
        ty: &tast::Ty,
    ) -> tast::Pat {
        match (pat, ty) {
            (_, tast::Ty::TVar(ty_var)) => {
                let pat_tast = self.infer_pat(env, vars, pat);
                let pat_ty = pat_tast.get_ty();
                self.uni.unify_var_value(*ty_var, Some(pat_ty)).unwrap();
                pat_tast
            }
            (ast::Pat::PVar { name }, _) => {
                let ty_var = self.fresh_ty_var();
                self.uni.unify_var_value(ty_var, Some(ty.clone())).unwrap();
                vars.insert(name.clone(), ty_var);
                tast::Pat::PVar {
                    name: name.0.clone(),
                    ty: ty.clone(),
                }
            }
            (ast::Pat::PTuple { pats }, tast::Ty::TTuple { typs }) => {
                let mut pats_tast = Vec::new();
                for (pat, ty) in pats.iter().zip(typs.iter()) {
                    let pat_tast = self.check_pat(env, vars, pat, ty);
                    pats_tast.push(pat_tast);
                }
                tast::Pat::PTuple {
                    items: pats_tast,
                    ty: ty.clone(),
                }
            }

            (ast::Pat::PConstr { vcon, args }, _) => {
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
                tast::Pat::PConstr {
                    index: env.get_index_of_constructor(&vcon.0).unwrap() as usize,
                    args: args_tast,
                    ty: ty.clone(),
                }
            }
            (ast::Pat::PUnit, tast::Ty::TUnit) => tast::Pat::PUnit {
                ty: tast::Ty::TUnit,
            },
            (ast::Pat::PBool { value }, tast::Ty::TBool) => tast::Pat::PBool {
                value: *value,
                ty: tast::Ty::TBool,
            },
            (ast::Pat::PWild, _) => {
                let ty_var = self.fresh_ty_var();
                self.uni.unify_var_value(ty_var, Some(ty.clone())).unwrap();
                tast::Pat::PWild { ty: ty.clone() }
            }
            _ => {
                panic!("type mismatch: expected {:?}, found {:?}", ty, pat)
            }
        }
    }

    fn infer_pat(
        &mut self,
        env: &Env,
        vars: &mut im::HashMap<Lident, TypeVar>,
        pat: &ast::Pat,
    ) -> tast::Pat {
        match pat {
            ast::Pat::PVar { name } => {
                let ty_var = vars[name];
                let ty = self.uni.probe_value(ty_var).unwrap();
                tast::Pat::PVar {
                    name: name.0.clone(),
                    ty,
                }
            }
            ast::Pat::PConstr { vcon, args } => {
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
                tast::Pat::PConstr {
                    index: env.get_index_of_constructor(&vcon.0).unwrap() as usize,
                    args: args_tast,
                    ty: tast::Ty::TConstr { name: vcon.clone() },
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
