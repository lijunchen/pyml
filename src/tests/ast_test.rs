use expect_test::{Expect, expect};

fn check(src: &str, expected: Expect) {
    let expr_parser = crate::grammar::ExprParser::new();
    let ast = expr_parser.parse(src).unwrap();
    expected.assert_debug_eq(&ast);
}

#[test]
fn test_001() {
    check(
        "()",
        expect![[r#"
        EUnit
    "#]],
    );
}

#[test]
fn test_002() {
    check(
        "(true, false, true)",
        expect![[r#"
            ETuple {
                items: [
                    EBool {
                        value: true,
                    },
                    EBool {
                        value: false,
                    },
                    EBool {
                        value: true,
                    },
                ],
            }
        "#]],
    );
}

#[test]
fn test_003() {
    check(
        "a.1.2",
        expect![[r#"
            EProj {
                tuple: EProj {
                    tuple: EVar {
                        name: Lident(
                            "a",
                        ),
                    },
                    index: 1,
                },
                index: 2,
            }
        "#]],
    );
}

#[test]
fn test_004() {
    check(
        "f(true, false)",
        expect![[r#"
            EPrim {
                func: Lident(
                    "f",
                ),
                args: [
                    EBool {
                        value: true,
                    },
                    EBool {
                        value: false,
                    },
                ],
            }
        "#]],
    );
}

#[test]
fn test_005() {
    check(
        "(F, F(), F(true, false))",
        expect![[r#"
            ETuple {
                items: [
                    EConstr {
                        vcon: Uident(
                            "F",
                        ),
                        args: [],
                    },
                    EConstr {
                        vcon: Uident(
                            "F",
                        ),
                        args: [],
                    },
                    EConstr {
                        vcon: Uident(
                            "F",
                        ),
                        args: [
                            EBool {
                                value: true,
                            },
                            EBool {
                                value: false,
                            },
                        ],
                    },
                ],
            }
        "#]],
    );
}

#[test]
fn test_006() {
    check(
        r#"
        match a {
          (false, true) => print_bool(false),
          (false, false) => print_bool(true),
          _ => print_unit(()),
        }
        "#,
        expect![[r#"
            EMatch {
                expr: EVar {
                    name: Lident(
                        "a",
                    ),
                },
                arms: [
                    Arm {
                        pat: PTuple {
                            pats: [
                                PBool {
                                    value: false,
                                },
                                PBool {
                                    value: true,
                                },
                            ],
                        },
                        body: EPrim {
                            func: Lident(
                                "print_bool",
                            ),
                            args: [
                                EBool {
                                    value: false,
                                },
                            ],
                        },
                    },
                    Arm {
                        pat: PTuple {
                            pats: [
                                PBool {
                                    value: false,
                                },
                                PBool {
                                    value: false,
                                },
                            ],
                        },
                        body: EPrim {
                            func: Lident(
                                "print_bool",
                            ),
                            args: [
                                EBool {
                                    value: true,
                                },
                            ],
                        },
                    },
                    Arm {
                        pat: PWild,
                        body: EPrim {
                            func: Lident(
                                "print_unit",
                            ),
                            args: [
                                EUnit,
                            ],
                        },
                    },
                ],
            }
        "#]],
    );
}
