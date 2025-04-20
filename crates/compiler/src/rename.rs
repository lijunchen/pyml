use std::cell::Cell;

use ::ast::ast::Lident;
use ast::ast;

#[derive(Default)]
pub struct Rename {
    counter: Cell<u32>,
}

#[derive(Debug)]
struct Env(im::Vector<(Lident, Lident)>);

impl Env {
    pub fn new() -> Self {
        Self(im::Vector::new())
    }

    #[allow(unused)]
    pub fn enter_scope(&self) -> Self {
        Self(self.0.clone())
    }

    pub fn add(&mut self, name: &Lident, new_name: &Lident) {
        self.0.push_back((name.clone(), new_name.clone()));
    }

    pub fn rfind(&self, key: &Lident) -> Option<&Lident> {
        self.0
            .iter()
            .rfind(|(name, _)| name == key)
            .map(|(_, new_name)| new_name)
    }
}

impl Rename {
    fn fresh_name(&self, name: &str) -> Lident {
        let new_name = format!("{}/{}", name, self.counter.get());
        self.counter.set(self.counter.get() + 1);
        Lident(new_name)
    }

    pub fn rename_file(&self, ast: ast::File) -> ast::File {
        ast::File {
            toplevels: ast
                .toplevels
                .iter()
                .map(|it| self.rename_item(it))
                .collect(),
        }
    }

    pub fn rename_item(&self, item: &ast::Item) -> ast::Item {
        match item {
            ast::Item::EnumDef(_) => item.clone(),
            ast::Item::TraitDef(_) => item.clone(),
            ast::Item::ImplBlock(i) => ast::Item::ImplBlock(ast::ImplBlock {
                trait_name: i.trait_name.clone(),
                for_type: i.for_type.clone(),
                methods: i.methods.iter().map(|m| self.rename_fn(m)).collect(),
            }),
            ast::Item::Fn(func) => ast::Item::Fn(self.rename_fn(func)),
        }
    }

    pub fn rename_fn(&self, func: &ast::Fn) -> ast::Fn {
        let ast::Fn {
            name,
            generics,
            params,
            ret_ty,
            body,
        } = func;
        let mut env = Env::new();
        for param in params {
            env.add(&param.0, &self.fresh_name(&param.0.0));
        }
        let new_params = params
            .iter()
            .map(|param| (env.rfind(&param.0).unwrap().clone(), param.1.clone()))
            .collect();
        ast::Fn {
            name: name.clone(),
            generics: generics.clone(),
            params: new_params,
            ret_ty: ret_ty.clone(),
            body: self.rename_expr(body, &mut env),
        }
    }

    fn rename_expr(&self, expr: &ast::Expr, env: &mut Env) -> ast::Expr {
        match expr {
            ast::Expr::EVar { name, astptr } => {
                if let Some(new_name) = env.rfind(name) {
                    ast::Expr::EVar {
                        name: new_name.clone(),
                        astptr: *astptr,
                    }
                } else {
                    panic!("Variable {} not found in environment", name.0);
                }
            }
            ast::Expr::EUnit => expr.clone(),
            ast::Expr::EBool { .. } => expr.clone(),
            ast::Expr::EInt { .. } => expr.clone(),
            ast::Expr::EString { .. } => expr.clone(),
            ast::Expr::EConstr { vcon, args } => ast::Expr::EConstr {
                vcon: vcon.clone(),
                args: args.iter().map(|arg| self.rename_expr(arg, env)).collect(),
            },
            ast::Expr::ETuple { items } => ast::Expr::ETuple {
                items: items
                    .iter()
                    .map(|item| self.rename_expr(item, env))
                    .collect(),
            },
            ast::Expr::ELet { pat, value, body } => {
                let new_value = self.rename_expr(value, env);
                let new_pat = self.rename_pat(pat, env);
                let new_body = self.rename_expr(body, env);
                ast::Expr::ELet {
                    pat: new_pat,
                    value: Box::new(new_value),
                    body: Box::new(new_body),
                }
            }
            ast::Expr::EMatch { expr, arms } => {
                let new_expr = self.rename_expr(expr, env);
                let new_arms = arms
                    .iter()
                    .map(|arm| {
                        let new_pat = self.rename_pat(&arm.pat, env);
                        let new_body = self.rename_expr(&arm.body, env);
                        ast::Arm {
                            pat: new_pat,
                            body: new_body,
                        }
                    })
                    .collect();
                ast::Expr::EMatch {
                    expr: Box::new(new_expr),
                    arms: new_arms,
                }
            }
            ast::Expr::ECall { func, args } => {
                let new_func = if let Some(new_name) = env.rfind(func) {
                    new_name.clone()
                } else {
                    func.clone()
                };
                let new_args = args.iter().map(|arg| self.rename_expr(arg, env)).collect();
                ast::Expr::ECall {
                    func: new_func,
                    args: new_args,
                }
            }
            ast::Expr::EProj { tuple, index } => {
                let new_tuple = self.rename_expr(tuple, env);
                let new_index = self.rename_expr(index, env);
                ast::Expr::EProj {
                    tuple: Box::new(new_tuple),
                    index: Box::new(new_index),
                }
            }
        }
    }

    fn rename_pat(&self, pat: &ast::Pat, env: &mut Env) -> ast::Pat {
        match pat {
            ast::Pat::PVar { name, astptr } => {
                let newname = self.fresh_name(&name.0);
                env.add(name, &newname);
                ast::Pat::PVar {
                    name: newname,
                    astptr: *astptr,
                }
            }
            ast::Pat::PUnit => pat.clone(),
            ast::Pat::PBool { .. } => pat.clone(),
            ast::Pat::PConstr { vcon, args } => {
                let new_args = args.iter().map(|arg| self.rename_pat(arg, env)).collect();
                ast::Pat::PConstr {
                    vcon: vcon.clone(),
                    args: new_args,
                }
            }
            ast::Pat::PTuple { pats } => {
                let new_pats = pats.iter().map(|pat| self.rename_pat(pat, env)).collect();
                ast::Pat::PTuple { pats: new_pats }
            }
            ast::Pat::PWild => pat.clone(),
        }
    }
}
