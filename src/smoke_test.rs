#[cfg(test)]
mod tests {

    use std::vec;

    use crate::tast::Arm;
    use crate::tast::Expr::{self, *};
    use crate::tast::Pat::{self, *};
    use crate::tast::Ty::{self, *};

    pub fn evar(name: &str, ty: Ty) -> Expr {
        Expr::EVar {
            name: name.to_string(),
            ty,
        }
    }

    #[allow(unused)]
    pub fn eunit() -> Expr {
        Expr::EUnit { ty: TUnit }
    }

    #[allow(unused)]
    pub fn etuple(items: Vec<Expr>, ty: Ty) -> Expr {
        Expr::ETuple { items, ty }
    }

    pub fn estub(name: &str) -> Expr {
        Expr::EPrim {
            func: name.to_string(),
            args: vec![],
            ty: TUnit,
        }
    }

    pub fn pwild(ty: Ty) -> Pat {
        Pat::PWild { ty }
    }

    // enum Color {
    //   Red,
    //   Green,
    //   Blue,
    // }
    fn tcolor() -> Ty {
        TConstr {
            name: "Color".to_string(),
        }
    }

    #[allow(unused)]
    fn red() -> Expr {
        Expr::EConstr {
            index: 0,
            args: vec![],
            ty: tcolor(),
        }
    }

    #[allow(unused)]
    fn green() -> Expr {
        Expr::EConstr {
            index: 1,
            args: vec![],
            ty: tcolor(),
        }
    }

    #[allow(unused)]
    fn blue() -> Expr {
        Expr::EConstr {
            index: 2,
            args: vec![],
            ty: tcolor(),
        }
    }

    fn pred() -> Pat {
        Pat::PConstr {
            index: 0,
            args: vec![],
            ty: tcolor(),
        }
    }

    fn pgreen() -> Pat {
        Pat::PConstr {
            index: 1,
            args: vec![],
            ty: tcolor(),
        }
    }

    fn pblue() -> Pat {
        Pat::PConstr {
            index: 2,
            args: vec![],
            ty: tcolor(),
        }
    }

    #[test]
    fn test_ast() {
        let env = crate::env::Env::toy_env();
        let make_cc_ty = || TTuple(vec![tcolor(), tcolor()]);
        let make_ccc_ty = || TTuple(vec![make_cc_ty(), tcolor(), tcolor()]);
        let e = EMatch {
            expr: Box::new(evar("a", make_ccc_ty())),
            arms: vec![
                Arm {
                    pat: PTuple {
                        items: vec![
                            PTuple {
                                items: vec![pred(), pgreen()],
                                ty: make_cc_ty(),
                            },
                            pgreen(),
                            pblue(),
                        ],
                        ty: make_ccc_ty(),
                    },
                    body: estub("case1"),
                },
                Arm {
                    pat: PTuple {
                        items: vec![
                            PTuple {
                                items: vec![pred(), pblue()],
                                ty: make_cc_ty(),
                            },
                            pred(),
                            pblue(),
                        ],
                        ty: make_ccc_ty(),
                    },
                    body: estub("case2"),
                },
                Arm {
                    pat: PTuple {
                        items: vec![
                            PTuple {
                                items: vec![pblue(), pgreen()],
                                ty: make_cc_ty(),
                            },
                            pred(),
                            pgreen(),
                        ],
                        ty: make_ccc_ty(),
                    },
                    body: estub("case3"),
                },
                Arm {
                    pat: PTuple {
                        items: vec![
                            PTuple {
                                items: vec![pblue(), pred()],
                                ty: make_cc_ty(),
                            },
                            pwild(tcolor()),
                            pwild(tcolor()),
                        ],
                        ty: make_ccc_ty(),
                    },
                    body: estub("case4"),
                },
            ],
            ty: TUnit,
        };
        expect_test::expect![[r#"
            match a {
                ((Color::Red,Color::Green),Color::Green,Color::Blue) => case1(),
                ((Color::Red,Color::Blue),Color::Red,Color::Blue) => case2(),
                ((Color::Blue,Color::Green),Color::Red,Color::Green) => case3(),
                ((Color::Blue,Color::Red),_,_) => case4(),
            }"#]]
        .assert_eq(&e.to_pretty(&env, 120));
        let result = crate::compile::compile(&env, &e);
        expect_test::expect![[r#"
            match x2 {
              Color::Red => missing(),
              Color::Green => match x1 {
                Color::Red => match x4 {
                  Color::Red => missing(),
                  Color::Green => match x3 {
                    Color::Red => missing(),
                    Color::Green => missing(),
                    Color::Blue => case3(),
                  },
                  Color::Blue => missing(),
                },
                Color::Green => missing(),
                Color::Blue => missing(),
              },
              Color::Blue => match x1 {
                Color::Red => match x6 {
                  Color::Red => missing(),
                  Color::Green => missing(),
                  Color::Blue => match x5 {
                    Color::Red => case2(),
                    Color::Green => missing(),
                    Color::Blue => missing(),
                  },
                },
                Color::Green => match x8 {
                  Color::Red => missing(),
                  Color::Green => match x7 {
                    Color::Red => case1(),
                    Color::Green => missing(),
                    Color::Blue => missing(),
                  },
                  Color::Blue => missing(),
                },
                Color::Blue => missing(),
              },
            }"#]]
        .assert_eq(&result.to_pretty(&env, 120))
    }

    #[test]
    fn test_ast_002() {
        let env = crate::env::Env::toy_env();
        let e = EMatch {
            expr: Box::new(evar("a", TUnit)),
            arms: vec![Arm {
                pat: PUnit,
                body: estub("case1"),
            }],
            ty: TUnit,
        };
        expect_test::expect![[r#"
            match a {
                () => case1(),
            }"#]]
        .assert_eq(&e.to_pretty(&env, 120));
        let c = crate::compile::compile(&env, &e);
        expect_test::expect![[r#"
            match a {
              () => case1(),
            }"#]]
        .assert_eq(&c.to_pretty(&env, 120));
    }

    #[test]
    fn test_ast_003() {
        let env = crate::env::Env::toy_env();
        let e = EMatch {
            expr: Box::new(evar("a", tcolor())),
            arms: vec![
                Arm {
                    pat: PConstr {
                        index: 1,
                        args: vec![],
                        ty: tcolor(),
                    },
                    body: estub("case1"),
                },
                Arm {
                    pat: PConstr {
                        index: 2,
                        args: vec![],
                        ty: tcolor(),
                    },
                    body: estub("case2"),
                },
                Arm {
                    pat: PConstr {
                        index: 0,
                        args: vec![],
                        ty: tcolor(),
                    },
                    body: estub("case3"),
                },
            ],
            ty: TUnit,
        };
        expect_test::expect![[r#"
            match a {
                Color::Green => case1(),
                Color::Blue => case2(),
                Color::Red => case3(),
            }"#]]
        .assert_eq(&e.to_pretty(&env, 120));
        let c = crate::compile::compile(&env, &e);
        expect_test::expect![[r#"
            match a {
              Color::Red => case3(),
              Color::Green => case1(),
              Color::Blue => case2(),
            }"#]]
        .assert_eq(&c.to_pretty(&env, 120));
    }

    #[test]
    fn test_ast_004() {
        let env = crate::env::Env::toy_env();
        let make_cc_ty = || TTuple(vec![tcolor(), tcolor()]);
        let e = EMatch {
            expr: Box::new(evar("a", make_cc_ty())),
            arms: vec![
                Arm {
                    pat: PTuple {
                        items: vec![
                            PConstr {
                                index: 1,
                                args: vec![],
                                ty: tcolor(),
                            },
                            PConstr {
                                index: 1,
                                args: vec![],
                                ty: tcolor(),
                            },
                        ],
                        ty: make_cc_ty(),
                    },
                    body: estub("case1"),
                },
                Arm {
                    pat: PTuple {
                        items: vec![
                            PConstr {
                                index: 1,
                                args: vec![],
                                ty: tcolor(),
                            },
                            PConstr {
                                index: 0,
                                args: vec![],
                                ty: tcolor(),
                            },
                        ],
                        ty: make_cc_ty(),
                    },
                    body: estub("case2"),
                },
                Arm {
                    pat: PTuple {
                        items: vec![
                            PConstr {
                                index: 1,
                                args: vec![],
                                ty: tcolor(),
                            },
                            PVar {
                                name: "t".to_string(),
                                ty: tcolor(),
                            },
                        ],
                        ty: make_cc_ty(),
                    },
                    body: estub("case3"),
                },
            ],
            ty: make_cc_ty(),
        };
        expect_test::expect![[r#"
            match a {
                (Color::Green,Color::Green) => case1(),
                (Color::Green,Color::Red) => case2(),
                (Color::Green,t) => case3(),
            }"#]]
        .assert_eq(&e.to_pretty(&env, 120));
        let c = crate::compile::compile(&env, &e);
        expect_test::expect![[r#"
            match x1 {
              Color::Red => match x0 {
                Color::Red => missing(),
                Color::Green => case2(),
                Color::Blue => missing(),
              },
              Color::Green => match x0 {
                Color::Red => missing(),
                Color::Green => case1(),
                Color::Blue => missing(),
              },
              Color::Blue => missing(),
            }"#]]
        .assert_eq(&c.to_pretty(&env, 120));
    }

    #[test]
    fn test_ast_005() {
        let env = crate::env::Env::toy_env();
        let make_cb_ty = || TTuple(vec![tcolor(), TBool]);
        let e = EMatch {
            expr: Box::new(evar("a", make_cb_ty())),
            arms: vec![
                Arm {
                    pat: PTuple {
                        items: vec![
                            PConstr {
                                index: 1,
                                args: vec![],
                                ty: tcolor(),
                            },
                            PBool {
                                value: false,
                                ty: TBool,
                            },
                        ],
                        ty: make_cb_ty(),
                    },
                    body: estub("case1"),
                },
                Arm {
                    pat: PTuple {
                        items: vec![
                            PConstr {
                                index: 1,
                                args: vec![],
                                ty: tcolor(),
                            },
                            PBool {
                                value: true,
                                ty: TBool,
                            },
                        ],
                        ty: make_cb_ty(),
                    },
                    body: estub("case2"),
                },
                Arm {
                    pat: PTuple {
                        items: vec![
                            PConstr {
                                index: 0,
                                args: vec![],
                                ty: tcolor(),
                            },
                            PBool {
                                value: true,
                                ty: TBool,
                            },
                        ],
                        ty: make_cb_ty(),
                    },
                    body: estub("case3"),
                },
            ],
            ty: make_cb_ty(),
        };
        expect_test::expect![[r#"
            match a {
                (Color::Green,false) => case1(),
                (Color::Green,true) => case2(),
                (Color::Red,true) => case3(),
            }"#]]
        .assert_eq(&e.to_pretty(&env, 120));
        let c = crate::compile::compile(&env, &e);
        expect_test::expect![[r#"
            match x1 {
              true => match x0 {
                Color::Red => case3(),
                Color::Green => case2(),
                Color::Blue => missing(),
              },
              false => match x0 {
                Color::Red => missing(),
                Color::Green => case1(),
                Color::Blue => missing(),
              },
            }"#]]
        .assert_eq(&c.to_pretty(&env, 120));
    }
}
