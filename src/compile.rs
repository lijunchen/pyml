use crate::core;
use crate::env::Env;
use crate::tast::Arm;
use crate::tast::Expr::{self, *};
use crate::tast::Pat::{self, *};
use crate::tast::Ty;

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Is {
    var: String,
    pat: Pat,
}

#[derive(Debug, Clone)]
struct Row {
    columns: Vec<Is>,
    body: Expr,
}

impl Row {
    fn remove_column(&mut self, var: &str) -> Option<Is> {
        self.columns
            .iter()
            .position(|c| &c.var == var)
            .map(|idx| self.columns.remove(idx))
    }
}

fn make_rows(name: &str, arms: &[Arm]) -> Vec<Row> {
    let mut result = Vec::new();
    for Arm { pat, body } in arms.iter() {
        result.push(Row {
            columns: vec![Is {
                var: name.to_string(),
                pat: pat.clone(),
            }],
            body: body.clone(),
        })
    }
    result
}

fn move_variable_patterns(_row: &mut Row) {
    ()
}

fn branch_variable(rows: &[Row]) -> (String, Ty) {
    let mut counts = HashMap::new();
    let mut var_ty: HashMap<String, Ty> = HashMap::new();
    for row in rows {
        for col in &row.columns {
            *counts.entry(&col.var).or_insert(0_usize) += 1;
            var_ty.insert(col.var.clone(), col.pat.get_ty());
        }
    }
    let var = rows[0]
        .columns
        .iter()
        .map(|col| col.var.clone())
        .max_by_key(|var| counts[var])
        .unwrap();
    (var.clone(), var_ty[&var.clone()].clone())
}

fn compile_constructor_cases(
    env: &Env,
    rows: Vec<Row>,
    branch_var: String,
    branch_var_ty: &Ty,
    mut cases: Vec<(usize, Vec<String>, Vec<Row>)>,
    ty: &Ty,
) -> Vec<core::Arm> {
    for mut row in rows {
        if let Some(col) = row.remove_column(&branch_var) {
            if let Pat::PConstr { index, args, ty: _ } = col.pat {
                let mut cols = row.columns;
                for (var, pat) in cases[index].1.iter().zip(args.into_iter()) {
                    cols.push(Is {
                        var: var.clone(),
                        pat,
                    })
                }
                cases[index].2.push(Row {
                    columns: cols,
                    body: row.body,
                })
            }
        } else {
            for (_, _, rows) in &mut cases {
                rows.push(row.clone())
            }
        }
    }

    cases
        .into_iter()
        .map(|(cons, vars, rows)| core::Arm {
            lhs: core::Expr::EConstr {
                index: cons,
                args: vars
                    .into_iter()
                    .map(|var| core::Expr::EVar {
                        name: var,
                        ty: branch_var_ty.clone(),
                    })
                    .collect(),
                ty: branch_var_ty.clone(),
            },
            body: compile_rows(env, rows, ty),
        })
        .collect()
}

fn compile_rows(env: &Env, mut rows: Vec<Row>, ty: &Ty) -> core::Expr {
    if rows.is_empty() {
        return core::Expr::EPrim {
            func: "missing".to_string(),
            args: vec![],
            ty: core::Ty::TUnit,
        };
    }
    for row in &mut rows {
        move_variable_patterns(row);
    }

    if rows.first().map_or(false, |c| c.columns.is_empty()) {
        let row = rows.remove(0);
        return compile_expr(&row.body, env);
    }

    let (branch_var, branch_var_ty) = branch_variable(&rows);
    match &branch_var_ty {
        Ty::TUnit => {
            let mut new_rows = vec![];
            for mut r in rows {
                r.remove_column(&branch_var);
                new_rows.push(r);
            }
            core::Expr::EMatch {
                expr: Box::new(core::Expr::EVar {
                    name: branch_var.clone(),
                    ty: core::Ty::TUnit,
                }),
                arms: vec![core::Arm {
                    lhs: core::Expr::EUnit {
                        ty: core::Ty::TUnit,
                    },
                    body: compile_rows(env, new_rows, ty),
                }],
                default: None,
                ty: ty.clone(),
            }
        }
        Ty::TBool => {
            let mut true_rows = vec![];
            let mut false_rows = vec![];
            for mut r in rows {
                if let Some(col) = r.remove_column(&branch_var) {
                    if let Pat::PBool { value, ty: _ } = col.pat {
                        if value {
                            true_rows.push(r);
                        } else {
                            false_rows.push(r);
                        }
                    }
                }
            }
            core::Expr::EMatch {
                expr: Box::new(core::Expr::EVar {
                    name: branch_var.clone(),
                    ty: core::Ty::TBool,
                }),
                arms: vec![
                    core::Arm {
                        lhs: core::Expr::EBool {
                            value: true,
                            ty: core::Ty::TBool,
                        },
                        body: compile_rows(env, true_rows.clone(), ty),
                    },
                    core::Arm {
                        lhs: core::Expr::EBool {
                            value: false,
                            ty: core::Ty::TBool,
                        },
                        body: compile_rows(env, false_rows, ty),
                    },
                ],
                default: None,
                ty: ty.clone(),
            }
        }
        Ty::TConstr { name } => {
            let tydef = &env.enums[name];
            let cases = tydef
                .variants
                .iter()
                .enumerate()
                .map(|(idx, (_, args))| {
                    (
                        idx,
                        args.iter().map(|_| env.gensym("x")).collect::<Vec<_>>(),
                        vec![],
                    )
                })
                .collect();
            core::Expr::EMatch {
                expr: Box::new(core::Expr::EVar {
                    name: branch_var.clone(),
                    ty: ty.clone(),
                }),
                arms: compile_constructor_cases(env, rows, branch_var, &branch_var_ty, cases, ty),
                default: None,
                ty: ty.clone(),
            }
        }
        Ty::TTuple(tys) => {
            let names = tys.iter().map(|_| env.gensym("x")).collect::<Vec<_>>();
            let mut new_rows = vec![];
            for row in rows {
                let mut cols = vec![];
                for Is { var: _, pat } in row.columns.iter() {
                    match pat {
                        PTuple { items, ty: _ } => {
                            for (i, item) in items.iter().enumerate() {
                                cols.push(Is {
                                    var: names[i].clone(),
                                    pat: item.clone(),
                                });
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                new_rows.push(Row {
                    columns: cols,
                    body: row.body,
                });
            }
            compile_rows(env, new_rows, ty)
        }
    }
}

pub fn compile(env: &Env, e: &Expr) -> core::Expr {
    compile_expr(e, env)
}

fn compile_expr(e: &Expr, env: &Env) -> core::Expr {
    match e {
        EVar { name, ty } => core::Expr::EVar {
            name: name.to_string(),
            ty: ty.clone(),
        },
        EUnit { ty: _ } => core::Expr::EUnit {
            ty: core::Ty::TUnit,
        },
        EBool { value, ty: _ } => core::Expr::EBool {
            value: *value,
            ty: Ty::TBool,
        },
        ETuple { items, ty } => {
            let items = items.iter().map(|item| compile_expr(item, env)).collect();
            core::Expr::ETuple {
                items,
                ty: ty.clone(),
            }
        }
        EConstr { index, args, ty } => {
            let args = args.iter().map(|arg| compile_expr(arg, env)).collect();
            core::Expr::EConstr {
                index: *index,
                args,
                ty: ty.clone(),
            }
        }
        ELet {
            name: _,
            value: _,
            body: _,
            ty: _,
        } => todo!(),
        EMatch { expr, arms, ty } => match expr.as_ref() {
            EVar { name, ty: _ty } => {
                let rows = make_rows(name, &arms);
                compile_rows(env, rows, ty)
            }
            _ => {
                unreachable!()
            }
        },
        EPrim { func, args, ty } => {
            let args = args.iter().map(|arg| compile_expr(arg, env)).collect();
            core::Expr::EPrim {
                func: func.clone(),
                args,
                ty: ty.clone(),
            }
        }
    }
}
