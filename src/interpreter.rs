use crate::core;

#[derive(Debug, Clone)]
pub enum Value {
    VUnit,
    VBool(bool),
    VConstr(usize, Vec<Value>),
    VTuple(Vec<Value>),
}

pub fn eval(env: &im::HashMap<String, Value>, stdout: &mut String, e: &core::Expr) -> Value {
    match e {
        core::Expr::EVar { name, ty: _ } => {
            let v = env.get(name).unwrap();
            v.clone()
        }
        core::Expr::EUnit { ty: _ } => Value::VUnit,
        core::Expr::EBool { value, ty } => {
            if *value {
                Value::VBool(true)
            } else {
                Value::VBool(false)
            }
        }
        core::Expr::EConstr { index, args, ty: _ } => {
            let mut values = Vec::new();
            for arg in args {
                let v = eval(env, stdout, arg);
                values.push(v);
            }
            Value::VConstr(*index, values)
        }
        core::Expr::ETuple { items, ty: _ } => {
            let mut values = Vec::new();
            for item in items {
                let v = eval(env, stdout, item);
                values.push(v);
            }
            Value::VTuple(values)
        }
        core::Expr::ELet {
            name,
            value,
            body,
            ty,
        } => {
            let v = eval(env, stdout, value);
            let mut new_env = env.clone();
            new_env.insert(name.clone(), v);
            eval(&new_env, stdout, body)
        }
        core::Expr::EMatch {
            expr,
            arms,
            default,
            ty,
        } => {
            let v = eval(env, stdout, expr);
            match expr.get_ty() {
                core::Ty::TUnit => {
                    todo!()
                }
                core::Ty::TBool => {
                    let bool_value = match v {
                        Value::VBool(b) => b,
                        _ => unreachable!(),
                    };
                    match bool_value {
                        true => eval(env, stdout, &arms[0].body),
                        false => eval(env, stdout, &arms[1].body),
                    }
                }
                core::Ty::TConstr { name } => {
                    todo!()
                }
                core::Ty::TTuple { typs } => {
                    unreachable!()
                }
                core::Ty::TVar(..) => {
                    unreachable!()
                }
            }
        }
        core::Expr::EPrim { func, args, ty: _ } => match func.as_str() {
            "print_unit" => {
                let arg = eval(env, stdout, &args[0]);
                match arg {
                    Value::VUnit => {
                        stdout.push_str("()");
                        Value::VUnit
                    }
                    _ => unreachable!(),
                }
            }
            "print_bool" => {
                let arg = eval(env, stdout, &args[0]);
                match arg {
                    Value::VBool(b) => {
                        stdout.push_str(&b.to_string());
                        Value::VUnit
                    }
                    _ => unreachable!(),
                }
            }
            _ => {
                unreachable!()
            }
        },
        core::Expr::EProj {
            tuple,
            index,
            ty: _,
        } => {
            let tuple_value = eval(env, stdout, tuple);
            match tuple_value {
                Value::VTuple(values) => values.get(*index).cloned().unwrap_or(Value::VUnit),
                _ => unreachable!(),
            }
        }
    }
}
