use std::path;

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
            ast::Expr::EInt { value } => todo!(),
            ast::Expr::EConstr { vcon, args } => todo!(),
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
            ast::Expr::EMatch { expr, arms } => todo!(),
            ast::Expr::EPrim { func, args } => todo!(),
            ast::Expr::EProj { tuple, index } => todo!(),
        }
    }

    fn check_pat(
        &mut self,
        env: &Env,
        vars: &mut im::HashMap<Lident, TypeVar>,
        pat: &ast::Pat,
        ty: &tast::Ty,
    ) -> tast::Pat {
        match pat {
            ast::Pat::PVar { name } => {
                let ty_var = self.uni.new_key(None);
                self.uni.unify_var_value(ty_var, Some(ty.clone())).unwrap();
                vars.insert(name.clone(), ty_var);
                tast::Pat::PVar {
                    name: name.0.clone(),
                    ty: ty.clone(),
                }
            }
            ast::Pat::PConstr { vcon, args } => todo!(),
            ast::Pat::PTuple { pats } => todo!(),
            ast::Pat::PUnit => todo!(),
            ast::Pat::PBool { value } => todo!(),
            ast::Pat::PWild => todo!(),
        }
    }

    fn infer_pat(
        &mut self,
        env: &Env,
        vars: &mut im::HashMap<Lident, TypeVar>,
        pat: &ast::Pat,
    ) -> tast::Pat {
        match pat {
            ast::Pat::PVar { name } => todo!(),
            ast::Pat::PConstr { vcon, args } => todo!(),
            ast::Pat::PTuple { pats } => todo!(),
            ast::Pat::PUnit => todo!(),
            ast::Pat::PBool { value } => todo!(),
            ast::Pat::PWild => todo!(),
        }
    }
}
