use crate::core;

#[derive(Clone)]
pub enum Value {
    VUnit,
    VBool(bool),
    VInt(i32),
    VString(String),
    VConstr(usize, Vec<Value>),
    VTuple(Vec<Value>),
    VFunc(String, Vec<(String, core::Ty)>, core::Ty, core::Expr),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::VUnit => write!(f, "()"),
            Value::VBool(b) => write!(f, "{}", b),
            Value::VInt(i) => write!(f, "{}", i),
            Value::VString(s) => write!(f, "{}", s),
            Value::VConstr(index, args) => {
                write!(f, "VConstr({}, {:?})", index, args)
            }
            Value::VTuple(values) => {
                write!(f, "(")?;
                for (i, value) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", value)?;
                }
                write!(f, ")")
            }
            Value::VFunc(name, params, ret_ty, body) => {
                write!(f, "VFunc({}, {:?}, {:?}, {:?})", name, params, ret_ty, body)
            }
        }
    }
}

pub fn eval_file(
    env: &im::HashMap<String, Value>,
    stdout: &mut String,
    file: &core::File,
) -> Value {
    let mut new_env = env.clone();
    for f in file.toplevels.iter() {
        new_env.insert(
            f.name.clone(),
            Value::VFunc(
                f.name.clone(),
                f.params.clone(),
                f.ret_ty.clone(),
                f.body.clone(),
            ),
        );
    }
    for f in file.toplevels.iter() {
        if f.name == "main" {
            let v = eval(&new_env, stdout, &f.body);
            return v;
        }
    }
    panic!("Main function not found");
}

fn eval(env: &im::HashMap<String, Value>, stdout: &mut String, e: &core::Expr) -> Value {
    match e {
        core::Expr::EVar { name, ty: _ } => {
            let v = env
                .get(name)
                .unwrap_or_else(|| panic!("Variable {} not found in environment", name));
            v.clone()
        }
        core::Expr::EUnit { ty: _ } => Value::VUnit,
        core::Expr::EBool { value, ty: _ } => {
            if *value {
                Value::VBool(true)
            } else {
                Value::VBool(false)
            }
        }
        core::Expr::EInt { value, ty: _ } => Value::VInt(*value),
        core::Expr::EString { value, ty: _ } => Value::VString(value.clone()),
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
            ty: _,
        } => {
            let v = eval(env, stdout, value);
            let mut new_env = env.clone();
            new_env.insert(name.clone(), v);
            eval(&new_env, stdout, body)
        }
        core::Expr::EMatch {
            expr,
            arms,
            default: _,
            ty: _,
        } => {
            let v = eval(env, stdout, expr);
            match expr.get_ty() {
                core::Ty::TUnit => {
                    let _ = match v {
                        Value::VUnit => v,
                        _ => unreachable!(),
                    };

                    eval(env, stdout, &arms[0].body)
                }
                core::Ty::TString => {
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
                core::Ty::TInt => {
                    todo!()
                }
                core::Ty::TEnum { name: _ } => {
                    let constr_value = match v {
                        Value::VConstr(index, args) => (index, args),
                        _ => unreachable!(),
                    };

                    for core::Arm { lhs, body } in arms {
                        let constr = match lhs {
                            core::Expr::EConstr {
                                index,
                                args: _,
                                ty: _,
                            } => *index,
                            _ => unreachable!(),
                        };
                        if constr == constr_value.0 {
                            return eval(env, stdout, body);
                        }
                    }
                    unreachable!()
                }
                core::Ty::TTuple { typs: _ } => {
                    unreachable!()
                }
                core::Ty::TVar(..) => {
                    unreachable!()
                }
                core::Ty::TFunc {
                    params: _,
                    ret_ty: _,
                } => {
                    unreachable!()
                }
            }
        }
        core::Expr::ECall { func, args, ty: _ } => match func.as_str() {
            "unit_to_string" => {
                let arg = eval(env, stdout, &args[0]);
                match arg {
                    Value::VUnit => Value::VString("()".to_string()),
                    _ => unreachable!(),
                }
            }
            "bool_to_string" => {
                let arg = eval(env, stdout, &args[0]);
                match arg {
                    Value::VBool(b) => {
                        if b {
                            Value::VString("true".to_string())
                        } else {
                            Value::VString("false".to_string())
                        }
                    }
                    _ => unreachable!(),
                }
            }
            "int_to_string" => {
                let arg = eval(env, stdout, &args[0]);
                match arg {
                    Value::VInt(i) => Value::VString(i.to_string()),
                    _ => unreachable!(),
                }
            }
            "int_add" => {
                let arg1 = eval(env, stdout, &args[0]);
                let arg2 = eval(env, stdout, &args[1]);
                match (arg1, arg2) {
                    (Value::VInt(i1), Value::VInt(i2)) => {
                        let result = i1 + i2;
                        Value::VInt(result)
                    }
                    _ => unreachable!(),
                }
            }
            "int_sub" => {
                let arg1 = eval(env, stdout, &args[0]);
                let arg2 = eval(env, stdout, &args[1]);
                match (arg1, arg2) {
                    (Value::VInt(i1), Value::VInt(i2)) => {
                        let result = i1 - i2;
                        Value::VInt(result)
                    }
                    _ => unreachable!(),
                }
            }
            "int_less" => {
                let arg1 = eval(env, stdout, &args[0]);
                let arg2 = eval(env, stdout, &args[1]);
                match (arg1, arg2) {
                    (Value::VInt(i1), Value::VInt(i2)) => {
                        let result = i1 < i2;
                        Value::VBool(result)
                    }
                    _ => unreachable!(),
                }
            }
            "print" => {
                let arg = eval(env, stdout, &args[0]);
                match arg {
                    Value::VString(s) => {
                        stdout.push_str(&s);
                        Value::VUnit
                    }
                    _ => unreachable!(),
                }
            }
            "println" => {
                let arg = eval(env, stdout, &args[0]);
                match arg {
                    Value::VString(s) => {
                        stdout.push_str(&s);
                        stdout.push('\n');
                        Value::VUnit
                    }
                    _ => unreachable!(),
                }
            }
            "missing" => {
                stdout.push_str("missing");
                Value::VUnit
            }
            udf => {
                if !env.contains_key(udf) {
                    panic!("Function {} not found in environment", udf);
                }
                let udf_value = env.get(udf).unwrap();
                let (_name, params, _ret_ty, body) = match udf_value {
                    Value::VFunc(name, params, ret_ty, body) => (name, params, ret_ty, body),
                    _ => unreachable!(),
                };
                let new_env = env.clone();

                let mut new_env = new_env.clone();
                for (i, param) in params.iter().enumerate() {
                    let arg = eval(env, stdout, &args[i]);
                    new_env.insert(param.0.clone(), arg);
                }

                eval(&new_env, stdout, body)
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
        core::Expr::EConstrGet {
            expr,
            variant_index,
            field_index,
            ty: _,
        } => {
            let constr_value = eval(env, stdout, expr);
            match constr_value {
                Value::VConstr(index, args) => {
                    if index == *variant_index {
                        if let Some(arg) = args.get(*field_index) {
                            arg.clone()
                        } else {
                            unreachable!()
                        }
                    } else {
                        unreachable!()
                    }
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }
}
