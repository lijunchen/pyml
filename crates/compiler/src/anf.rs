pub type Ty = crate::tast::Ty;
use crate::{core, env::Env};

#[derive(Debug)]
pub struct File {
    pub toplevels: Vec<Fn>,
}

#[derive(Debug)]
pub struct Fn {
    pub name: String,
    pub params: Vec<(String, Ty)>,
    pub ret_ty: Ty,
    pub body: AExpr,
}

#[derive(Debug, Clone)]
pub enum ImmExpr {
    ImmVar { name: String, ty: Ty },
    ImmUnit { ty: Ty },
    ImmBool { value: bool, ty: Ty },
    ImmInt { value: i32, ty: Ty },
    ImmString { value: String, ty: Ty },
}

#[derive(Debug, Clone)]
pub enum CExpr {
    CImm {
        imm: ImmExpr,
    },

    EConstr {
        index: usize,
        args: Vec<ImmExpr>,
        ty: Ty,
    },
    ETuple {
        items: Vec<ImmExpr>,
        ty: Ty,
    },
    EMatch {
        expr: Box<ImmExpr>,
        arms: Vec<Arm>,
        default: Option<Box<AExpr>>,
        ty: Ty,
    },
    EIf {
        cond: Box<ImmExpr>,
        then: Box<AExpr>,
        else_: Box<AExpr>,
        ty: Ty,
    },
    EConstrGet {
        expr: Box<ImmExpr>,
        variant_index: usize,
        field_index: usize,
        ty: Ty,
    },
    ECall {
        func: String,
        args: Vec<ImmExpr>,
        ty: Ty,
    },
    EProj {
        tuple: Box<ImmExpr>,
        index: usize,
        ty: Ty,
    },
}

#[derive(Debug, Clone)]
pub enum AExpr {
    ACExpr {
        expr: CExpr,
    },
    ALet {
        name: String,
        value: Box<CExpr>,
        body: Box<AExpr>,
        ty: Ty,
    },
}

#[derive(Debug, Clone)]
pub struct Arm {
    pub lhs: ImmExpr,
    pub body: AExpr,
}

// Helper function to convert core immediate expressions to ANF immediate expressions
// Assumes the input core::Expr is guaranteed to be an immediate variant.
fn core_imm_to_anf_imm(core_imm: core::Expr) -> ImmExpr {
    match core_imm {
        core::Expr::EVar { name, ty } => ImmExpr::ImmVar { name, ty },
        core::Expr::EUnit { ty } => ImmExpr::ImmUnit { ty },
        core::Expr::EBool { value, ty } => ImmExpr::ImmBool { value, ty },
        core::Expr::EInt { value, ty } => ImmExpr::ImmInt { value, ty },
        core::Expr::EString { value, ty } => ImmExpr::ImmString { value, ty },
        // Other core::Expr variants are not immediate and should not appear as match arm LHS patterns.
        _ => panic!(
            "Expected an immediate expression for match arm LHS, found {:?}",
            core_imm
        ),
    }
}

fn anf<'a>(env: &'a Env, e: core::Expr, k: Box<dyn FnOnce(CExpr) -> AExpr + 'a>) -> AExpr {
    let e_ty = e.get_ty();
    match e {
        core::Expr::EVar { name, ty } => k(CExpr::CImm {
            imm: ImmExpr::ImmVar { name, ty },
        }),
        core::Expr::EUnit { ty } => k(CExpr::CImm {
            imm: ImmExpr::ImmUnit { ty },
        }),
        core::Expr::EBool { value, ty } => k(CExpr::CImm {
            imm: ImmExpr::ImmBool { value, ty },
        }),
        core::Expr::EInt { value, ty } => k(CExpr::CImm {
            imm: ImmExpr::ImmInt { value, ty },
        }),
        core::Expr::EString { value, ty } => k(CExpr::CImm {
            imm: ImmExpr::ImmString { value, ty },
        }),

        core::Expr::EConstr { index, args, ty: _ } => anf_list(
            env,
            &args,
            Box::new(move |args| {
                k(CExpr::EConstr {
                    index,
                    args,
                    ty: e_ty,
                })
            }),
        ),
        core::Expr::ETuple { items, ty: _ } => anf_list(
            env,
            &items,
            Box::new(move |items| k(CExpr::ETuple { items, ty: e_ty })),
        ),
        core::Expr::ELet {
            name,
            value,
            body,
            ty: _,
        } => anf(
            env,
            *value,
            Box::new(move |ve| AExpr::ALet {
                name,
                value: Box::new(ve),
                body: Box::new(anf(env, *body, k)),
                ty: e_ty.clone(),
            }),
        ),
        core::Expr::EMatch {
            expr,
            arms,
            default,
            ty: _,
        } => {
            anf_imm(
                env,
                *expr,
                Box::new(move |imm_expr| {
                    // Convert arms
                    let anf_arms = arms
                        .into_iter()
                        .map(|core_arm| {
                            // Assuming core::Arm has fields lhs: core::Expr, body: core::Expr
                            // and lhs is guaranteed to be an immediate expression.
                            let anf_lhs = core_imm_to_anf_imm(core_arm.lhs);
                            let anf_body =
                                anf(env, core_arm.body, Box::new(|c| AExpr::ACExpr { expr: c }));
                            Arm {
                                lhs: anf_lhs,
                                body: anf_body,
                            }
                        })
                        .collect();

                    // Convert default case
                    let anf_default = default.map(|def_body| {
                        Box::new(anf(env, *def_body, Box::new(|c| AExpr::ACExpr { expr: c })))
                    });

                    k(CExpr::EMatch {
                        expr: Box::new(imm_expr),
                        arms: anf_arms,
                        default: anf_default,
                        ty: e_ty, // Use the type of the original match expression
                    })
                }),
            )
        }
        core::Expr::EConstrGet {
            expr,
            variant_index,
            field_index,
            ty: _,
        } => anf_imm(
            env,
            *expr,
            Box::new(move |e| {
                k(CExpr::EConstrGet {
                    expr: Box::new(e),
                    variant_index,
                    field_index,
                    ty: e_ty,
                })
            }),
        ),
        core::Expr::ECall { func, args, ty: _ } => anf_list(
            env,
            &args,
            Box::new(move |args| {
                k(CExpr::ECall {
                    func: func.clone(),
                    args,
                    ty: e_ty,
                })
            }),
        ),
        core::Expr::EProj {
            tuple,
            index,
            ty: _,
        } => anf_imm(
            env,
            *tuple,
            Box::new(move |e| {
                k(CExpr::EProj {
                    tuple: Box::new(e),
                    index,
                    ty: e_ty,
                })
            }),
        ),
    }
}

fn anf_imm<'a>(env: &'a Env, e: core::Expr, k: Box<dyn FnOnce(ImmExpr) -> AExpr + 'a>) -> AExpr {
    match e {
        core::Expr::EVar { name, ty } => k(ImmExpr::ImmVar { name, ty }),
        core::Expr::EUnit { ty } => k(ImmExpr::ImmUnit { ty }),
        core::Expr::EBool { value, ty } => k(ImmExpr::ImmBool { value, ty }),
        core::Expr::EInt { value, ty } => k(ImmExpr::ImmInt { value, ty }),
        core::Expr::EString { value, ty } => k(ImmExpr::ImmString { value, ty }),
        _ => {
            let name = env.gensym("t");
            let ty = e.get_ty();
            anf(
                env,
                e,
                Box::new(move |e| AExpr::ALet {
                    name: name.clone(),
                    value: Box::new(e),
                    body: Box::new(k(ImmExpr::ImmVar {
                        name,
                        ty: ty.clone(),
                    })),
                    ty: ty.clone(),
                }),
            )
        }
    }
}

fn anf_list<'a>(
    env: &'a Env,
    es: &'a [core::Expr],
    k: Box<dyn FnOnce(Vec<ImmExpr>) -> AExpr + 'a>,
) -> AExpr {
    if es.is_empty() {
        k(Vec::new())
    } else {
        let head = &es[0];
        let tail = &es[1..];
        anf_imm(
            env,
            head.clone(),
            Box::new(move |imm_head| {
                anf_list(
                    env,
                    tail,
                    Box::new(move |mut imm_tail| {
                        imm_tail.insert(0, imm_head);
                        k(imm_tail)
                    }),
                )
            }),
        )
    }
}

pub fn anf_file(env: &Env, file: core::File) -> File {
    let mut toplevels = Vec::new();
    for core_fn in file.toplevels {
        let name = core_fn.name;
        let params = core_fn.params;
        let ret_ty = core_fn.ret_ty;
        let body = anf(env, core_fn.body, Box::new(|c| AExpr::ACExpr { expr: c }));
        toplevels.push(Fn {
            name,
            params,
            ret_ty,
            body,
        });
    }
    File { toplevels }
}
