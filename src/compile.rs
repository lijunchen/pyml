use crate::core;
use crate::tast::Arm;
use crate::tast::Expr::{self, *};
use crate::tast::Pat::{self, *};
use crate::tast::Ty;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn gensym(name: &str) -> String {
    let old = COUNTER.load(Ordering::SeqCst);
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}{}", name, count)
}

fn reset() {
    COUNTER.store(0, Ordering::SeqCst);
}

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
    // 第一步，make row
    // match a {
    //     (Red, Green, Blue) => ()
    //     (Green, Red, Blue) => ()
    //     (Blue, Red, Green) => ()
    //     (Red, _, _) => ()
    //   }
    // => 变成
    // Rows:
    //   a is (Red, Green, Blue) => ()
    //   a is (Green, Red, Blue) => ()
    //   a is (Blue, Red, Green) => ()
    //   a is (Red, _, _) => ()

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

fn move_variable_patterns(row: &mut Row) {
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
    rows: Vec<Row>,
    branch_var: String,
    branch_var_ty: &Ty,
    mut cases: Vec<(usize, Vec<String>, Vec<Row>)>,
    ty: &Ty,
) -> Vec<core::Arm> {
    println!("Before remove");
    dbg!(&rows);
    println!("{:#?}", cases);
    for mut row in rows {
        if let Some(col) = row.remove_column(&branch_var) {
            println!("!!!find branch var {}, pat: {:#?}", col.var, col.pat);
            if let Pat::PConstr { index, args, ty } = col.pat {
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
    println!("After remove");
    println!("{:#?}", cases);

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
            body: compile_rows(rows, ty),
        })
        .collect()
}

fn compile_rows(mut rows: Vec<Row>, ty: &Ty) -> core::Expr {
    //   a is (Red, Green, Blue) => ()
    //   a is (Green, Red, Blue) => ()
    //   a is (Blue, Red, Green) => ()
    //   a is (Red, _, _) => ()
    // 第一步，确定在哪列进行branch
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
        // 如果 rows 中的第一个row没有任何列，也就是说它是无条件成立的
        // 那么直接把它的body转成core就行了
        let row = rows.remove(0);
        return compile_expr(&row.body);
    }

    let (branch_var, branch_var_ty) = branch_variable(&rows);
    dbg!(&rows);
    dbg!(&branch_var, &branch_var_ty);
    match &branch_var_ty {
        Ty::TUnit => {
            todo!()
        }
        Ty::TColor => {
            let cases = vec![
                (0, vec![], vec![]),
                (1, vec![], vec![]),
                (2, vec![], vec![]),
            ];
            core::Expr::EMatch {
                expr: Box::new(core::Expr::EVar {
                    name: branch_var.clone(),
                    ty: core::Ty::TColor,
                }),
                arms: compile_constructor_cases(rows, branch_var, &branch_var_ty, cases, ty),
                default: None,
                ty: ty.clone(),
            }
        }
        Ty::TConstr { name } => {
            todo!()
        }
        Ty::TTuple(tys) => {
            let names = tys.iter().map(|_| gensym("x")).collect::<Vec<_>>();
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
            compile_rows(new_rows, ty)
        }
    }
}

pub fn compile_expr(e: &Expr) -> core::Expr {
    match e {
        EVar { name, ty } => todo!(),
        EUnit { ty } => core::Expr::EUnit {
            ty: core::Ty::TUnit,
        },
        EColor { value, ty } => todo!(),
        ETuple { items, ty } => {
            todo!()
        }
        EConstr { index, arg, ty } => {
            todo!()
        }
        ELet {
            name,
            value,
            body,
            ty,
        } => todo!(),
        EMatch { expr, arms, ty } => match expr.as_ref() {
            EVar { name, ty } => {
                let rows = make_rows(name, &arms);
                compile_rows(rows, ty)
            }
            _ => {
                unreachable!()
            }
        },
        EPrim { func, args, ty } => {
            let args = args.iter().map(|arg| compile_expr(arg)).collect::<Vec<_>>();
            core::Expr::EPrim {
                func: func.clone(),
                args,
                ty: ty.clone(),
            }
        }
    }
}
