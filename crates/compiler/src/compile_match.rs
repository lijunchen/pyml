use ast::ast::Uident;

use crate::core;
use crate::env::Env;
use crate::tast::Arm;
use crate::tast::Expr::{self, *};
use crate::tast::Pat::{self, *};
use crate::tast::Ty;
use crate::tast::{self, File};

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Column {
    var: String,
    pat: tast::Pat,
}

#[derive(Debug, Clone)]
struct Row {
    columns: Vec<Column>,
    body: tast::Expr,
}

impl Row {
    fn remove_column(&mut self, var: &str) -> Option<Column> {
        let mut index = None;
        for (i, col) in self.columns.iter().enumerate() {
            if col.var == var {
                index = Some(i);
                break;
            }
        }
        if let Some(i) = index {
            let col = self.columns.remove(i);
            return Some(col);
        }
        None
    }
}

fn make_rows(name: &str, arms: &[Arm]) -> Vec<Row> {
    let mut result = Vec::new();
    for Arm { pat, body } in arms.iter() {
        result.push(Row {
            columns: vec![Column {
                var: name.to_string(),
                pat: pat.clone(),
            }],
            body: body.clone(),
        })
    }
    result
}

fn move_variable_patterns(row: &mut Row) {
    row.columns.retain(|col| match &col.pat {
        Pat::PVar {
            name,
            ty,
            astptr: _,
        } => {
            row.body = ELet {
                pat: Pat::PVar {
                    name: name.clone(),
                    ty: ty.clone(),
                    astptr: None,
                },
                value: Box::new(EVar {
                    name: col.var.clone(),
                    ty: col.pat.get_ty(),
                    astptr: None,
                }),
                ty: row.body.get_ty(),
                body: Box::new(row.body.clone()),
            };
            false
        }
        Pat::PWild { ty: _ } => false,
        _ => true,
    });
}

#[derive(Debug, Clone)]
struct Variable {
    pub name: String,
    pub ty: Ty,
}

impl Variable {
    pub fn to_core(&self) -> core::Expr {
        core::Expr::EVar {
            name: self.name.clone(),
            ty: self.ty.clone(),
        }
    }
}

fn emissing(ty: &Ty) -> core::Expr {
    core::Expr::ECall {
        func: "missing".to_string(),
        args: vec![],
        ty: ty.clone(),
    }
}

fn branch_variable(rows: &[Row]) -> Variable {
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
    Variable {
        name: var.clone(),
        ty: var_ty[&var].clone(),
    }
}

struct ConstructorCase {
    index: usize,
    vars: Vec<Variable>,
    rows: Vec<Row>,
}

fn compile_constructor_cases(
    env: &Env,
    rows: Vec<Row>,
    bvar: &Variable,
    mut cases: Vec<ConstructorCase>,
    ty: &Ty,
) -> Vec<core::Arm> {
    for mut row in rows {
        if let Some(col) = row.remove_column(&bvar.name) {
            if let Pat::PConstr { index, args, ty: _ } = col.pat {
                let mut cols = row.columns;
                for (var, pat) in cases[index].vars.iter().zip(args.into_iter()) {
                    cols.push(Column {
                        var: var.name.clone(),
                        pat,
                    })
                }
                cases[index].rows.push(Row {
                    columns: cols,
                    body: row.body,
                })
            } else {
                unreachable!()
            }
        } else {
            for ConstructorCase { rows, .. } in &mut cases {
                rows.push(row.clone())
            }
        }
    }

    let mut arms = vec![];
    for case in cases.into_iter() {
        let args = case.vars.into_iter().map(|var| var.to_core()).collect();
        let arm = core::Arm {
            lhs: core::Expr::EConstr {
                index: case.index,
                args,
                ty: bvar.ty.clone(),
            },
            body: compile_rows(env, case.rows, ty),
        };
        arms.push(arm);
    }
    arms
}

fn compile_enum_case(
    env: &Env,
    rows: Vec<Row>,
    bvar: &Variable,
    ty: &Ty,
    name: &Uident,
) -> core::Expr {
    let tydef = &env.enums[name];

    let cases: Vec<ConstructorCase> = tydef
        .variants
        .iter()
        .enumerate()
        .map(|(index, (_variant, args))| ConstructorCase {
            index,
            vars: args
                .iter()
                .map(|arg_ty| Variable {
                    name: env.gensym("x"),
                    ty: arg_ty.clone(),
                })
                .collect::<Vec<_>>(),
            rows: vec![],
        })
        .collect();

    let mut results = Vec::new();

    for (tag, case) in cases.iter().enumerate().take(tydef.variants.len()) {
        let hole = core::eunit();
        let mut result = hole;
        for (field, var) in case.vars.iter().enumerate().rev() {
            result = core::Expr::ELet {
                name: var.name.clone(),
                value: Box::new(core::Expr::EConstrGet {
                    expr: Box::new(bvar.to_core()),
                    variant_index: tag,
                    field_index: field,
                    ty: bvar.ty.clone(),
                }),
                body: Box::new(result),
                ty: ty.clone(),
            }
        }
        results.push(result);
    }

    let arms = compile_constructor_cases(env, rows, bvar, cases, ty);

    let mut new_arms = vec![];
    for (mut res, mut arm) in results.into_iter().zip(arms.into_iter()) {
        replace_default_expr(&mut res, arm.body);
        arm.body = res;
        new_arms.push(arm);
    }

    core::Expr::EMatch {
        expr: Box::new(bvar.to_core()),
        arms: new_arms,
        default: None,
        ty: ty.clone(),
    }
}

fn compile_tuple_case(
    env: &Env,
    rows: Vec<Row>,
    bvar: &Variable,
    typs: &[Ty],
    ty: &Ty,
) -> core::Expr {
    let names = typs.iter().map(|_| env.gensym("x")).collect::<Vec<_>>();
    let mut new_rows = vec![];

    let hole = core::eunit();
    // Create a let expression that extracts tuple elements
    let mut result = hole;

    // For each element in the tuple, create a binding
    for (i, name) in names.iter().enumerate().rev() {
        result = core::Expr::ELet {
            name: name.clone(),
            value: Box::new(core::Expr::EProj {
                tuple: Box::new(bvar.to_core()),
                index: i,
                ty: typs[i].clone(),
            }),
            body: Box::new(result),
            ty: ty.clone(),
        };
    }

    for row in rows {
        let mut cols = vec![];
        for Column { var, pat } in row.columns {
            if var == bvar.name {
                if let PTuple { items, ty: _ } = pat {
                    for (i, item) in items.into_iter().enumerate() {
                        cols.push(Column {
                            var: names[i].clone(),
                            pat: item,
                        });
                    }
                } else {
                    // since the type of bvar.ty is Tuple,
                    // so we should not reach here
                    unreachable!()
                }
            } else {
                cols.push(Column { var, pat });
            }
        }
        new_rows.push(Row {
            columns: cols,
            body: row.body,
        });
    }

    let inner = compile_rows(env, new_rows, ty);

    // Replace the empty default result with the actual compiled rows
    replace_default_expr(&mut result, inner);
    result
}

fn compile_unit_case(env: &Env, rows: Vec<Row>, bvar: &Variable) -> core::Expr {
    let mut new_rows = vec![];
    for mut r in rows {
        #[allow(clippy::redundant_pattern_matching)]
        if let Some(_) = r.remove_column(&bvar.name) {
            new_rows.push(r);
        } else {
            new_rows.push(r);
        }
    }
    core::Expr::EMatch {
        expr: Box::new(bvar.to_core()),
        arms: vec![core::Arm {
            lhs: core::eunit(),
            body: compile_rows(env, new_rows, &bvar.ty),
        }],
        default: None,
        ty: bvar.ty.clone(),
    }
}

fn compile_bool_case(env: &Env, rows: Vec<Row>, bvar: &Variable) -> core::Expr {
    let mut true_rows = vec![];
    let mut false_rows = vec![];
    for mut r in rows {
        if let Some(col) = r.remove_column(&bvar.name) {
            if let Pat::PBool { value, ty: _ } = col.pat {
                if value {
                    true_rows.push(r);
                } else {
                    false_rows.push(r);
                }
            }
        } else {
            true_rows.push(r.clone());
            false_rows.push(r);
        }
    }
    core::Expr::EMatch {
        expr: Box::new(bvar.to_core()),
        arms: vec![
            core::Arm {
                lhs: core::ebool(true),
                body: compile_rows(env, true_rows, &bvar.ty),
            },
            core::Arm {
                lhs: core::ebool(false),
                body: compile_rows(env, false_rows, &bvar.ty),
            },
        ],
        default: None,
        ty: bvar.ty.clone(),
    }
}

fn compile_rows(env: &Env, mut rows: Vec<Row>, ty: &Ty) -> core::Expr {
    if rows.is_empty() {
        return emissing(ty);
    }
    for row in &mut rows {
        move_variable_patterns(row);
    }

    if rows.first().is_some_and(|c| c.columns.is_empty()) {
        let row = rows.remove(0);
        return compile_expr(&row.body, env);
    }

    let bvar = branch_variable(&rows);
    match &bvar.ty {
        Ty::TVar(..) => unreachable!(),
        Ty::TUnit => compile_unit_case(env, rows, &bvar),
        Ty::TBool => compile_bool_case(env, rows, &bvar),
        Ty::TInt => {
            todo!()
        }
        Ty::TString => {
            todo!()
        }
        Ty::TApp { name, args: _ } => compile_enum_case(env, rows, &bvar, ty, name),
        Ty::TTuple { typs } => compile_tuple_case(env, rows, &bvar, typs, ty),
        Ty::TFunc { .. } => unreachable!(),
        Ty::TParam { .. } => unreachable!(),
    }
}

// Helper function to replace the default empty expression with the actual compiled inner expression
fn replace_default_expr(expr: &mut core::Expr, replacement: core::Expr) {
    match expr {
        core::Expr::ELet { body, .. } => replace_default_expr(body, replacement),
        _ => *expr = replacement,
    }
}

pub fn compile_file(env: &Env, file: &File) -> core::File {
    let mut toplevels = vec![];
    for f in file.toplevels.iter() {
        toplevels.push(core::Fn {
            name: f.name.clone(),
            params: f
                .params
                .iter()
                .map(|(name, ty)| (name.clone(), ty.clone()))
                .collect(),
            ret_ty: f.ret_ty.clone(),
            body: compile_expr(&f.body, env),
        })
    }
    core::File { toplevels }
}

fn compile_expr(e: &Expr, env: &Env) -> core::Expr {
    match e {
        EVar {
            name,
            ty,
            astptr: _,
        } => core::Expr::EVar {
            name: name.to_string(),
            ty: ty.clone(),
        },
        EUnit { .. } => core::Expr::EUnit {
            ty: core::Ty::TUnit,
        },
        EBool { value, ty: _ } => core::Expr::EBool {
            value: *value,
            ty: Ty::TBool,
        },
        EInt { value, ty: _ } => core::Expr::EInt {
            value: *value,
            ty: Ty::TInt,
        },
        EString { value, ty: _ } => core::Expr::EString {
            value: value.clone(),
            ty: Ty::TString,
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
            pat:
                Pat::PVar {
                    name,
                    ty: _pat_ty,
                    astptr: _,
                },
            value,
            body,
            ty,
        } => core::Expr::ELet {
            name: name.clone(),
            value: Box::new(compile_expr(value, env)),
            body: Box::new(compile_expr(body, env)),
            ty: ty.clone(),
        },
        ELet {
            pat,
            value,
            body,
            ty,
        } => {
            let core_value = compile_expr(value, env);
            let x = env.gensym("mtmp");
            let rows = vec![
                Row {
                    columns: vec![Column {
                        var: x.clone(),
                        pat: pat.clone(),
                    }],
                    body: *body.clone(),
                },
                Row {
                    columns: vec![Column {
                        var: x.clone(),
                        pat: Pat::PWild { ty: pat.get_ty() },
                    }],
                    body: ECall {
                        func: "missing".to_string(),
                        args: vec![],
                        ty: body.get_ty(),
                    },
                },
            ];
            core::Expr::ELet {
                name: x,
                value: Box::new(core_value),
                body: Box::new(compile_rows(env, rows, ty)),
                ty: ty.clone(),
            }
        }
        EMatch { expr, arms, ty } => match expr.as_ref() {
            EVar {
                name,
                ty: _ty,
                astptr: _,
            } => {
                let rows = make_rows(name, arms);
                compile_rows(env, rows, ty)
            }
            _ => {
                // create a new variable
                // match (a, b, c) { .. }
                // =>
                // let tmp = (a, b, c) in match tmp { ... }
                let mtmp = env.gensym("mtmp");
                let rows = make_rows(mtmp.as_str(), arms);
                let core_expr = compile_expr(expr, env);
                let core_rows = compile_rows(env, rows, ty);
                core::Expr::ELet {
                    name: mtmp,
                    value: Box::new(core_expr),
                    body: Box::new(core_rows),
                    ty: ty.clone(),
                }
            }
        },
        ECall { func, args, ty } => {
            let args = args.iter().map(|arg| compile_expr(arg, env)).collect();
            core::Expr::ECall {
                func: func.clone(),
                args,
                ty: ty.clone(),
            }
        }
        EProj { tuple, index, ty } => {
            let tuple = compile_expr(tuple, env);
            core::Expr::EProj {
                tuple: Box::new(tuple),
                index: *index,
                ty: ty.clone(),
            }
        }
    }
}
