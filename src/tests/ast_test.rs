use expect_test::{Expect, expect};

fn check(src: &str, expected: Expect) {
    let expr_parser = crate::grammar::ExprParser::new();
    let ast = expr_parser.parse(src).unwrap();
    expected.assert_debug_eq(&ast);
}

fn check_file(src: &str, expected: Expect) {
    let expr_parser = crate::grammar::FileParser::new();
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
                    index: EInt {
                        value: 1,
                    },
                },
                index: EInt {
                    value: 2,
                },
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

#[test]
fn test_007() {
    check_file(
        r#"
        let a = true in let b = false in or(a, b)
        "#,
        expect![[r#"
            File {
                enum_defs: [],
                expr: ELet {
                    pat: PVar {
                        name: Lident(
                            "a",
                        ),
                    },
                    value: EBool {
                        value: true,
                    },
                    body: ELet {
                        pat: PVar {
                            name: Lident(
                                "b",
                            ),
                        },
                        value: EBool {
                            value: false,
                        },
                        body: EPrim {
                            func: Lident(
                                "or",
                            ),
                            args: [
                                EVar {
                                    name: Lident(
                                        "a",
                                    ),
                                },
                                EVar {
                                    name: Lident(
                                        "b",
                                    ),
                                },
                            ],
                        },
                    },
                },
            }
        "#]],
    );
}

#[test]
fn test_008() {
    check_file(
        r#"
        let a = let b = let c = false in c in b in a
        "#,
        expect![[r#"
            File {
                enum_defs: [],
                expr: ELet {
                    pat: PVar {
                        name: Lident(
                            "a",
                        ),
                    },
                    value: ELet {
                        pat: PVar {
                            name: Lident(
                                "b",
                            ),
                        },
                        value: ELet {
                            pat: PVar {
                                name: Lident(
                                    "c",
                                ),
                            },
                            value: EBool {
                                value: false,
                            },
                            body: EVar {
                                name: Lident(
                                    "c",
                                ),
                            },
                        },
                        body: EVar {
                            name: Lident(
                                "b",
                            ),
                        },
                    },
                    body: EVar {
                        name: Lident(
                            "a",
                        ),
                    },
                },
            }
        "#]],
    );
}

#[test]
fn test_009() {
    check_file(
        r#"
        enum Color {
            Red,
            Green,
            Blue,
        }
        let c = (Red, Green) in
        match c {
            (Red, Green) => print_bool(true),
            (Red, Blue) => print_bool(false),
            _ => print_unit(()),
        }
        "#,
        expect![[r#"
            File {
                enum_defs: [
                    EnumDef {
                        name: Uident(
                            "Color",
                        ),
                        variants: [
                            (
                                Uident(
                                    "Red",
                                ),
                                [],
                            ),
                            (
                                Uident(
                                    "Green",
                                ),
                                [],
                            ),
                            (
                                Uident(
                                    "Blue",
                                ),
                                [],
                            ),
                        ],
                    },
                ],
                expr: ELet {
                    pat: PVar {
                        name: Lident(
                            "c",
                        ),
                    },
                    value: ETuple {
                        items: [
                            EConstr {
                                vcon: Uident(
                                    "Red",
                                ),
                                args: [],
                            },
                            EConstr {
                                vcon: Uident(
                                    "Green",
                                ),
                                args: [],
                            },
                        ],
                    },
                    body: EMatch {
                        expr: EVar {
                            name: Lident(
                                "c",
                            ),
                        },
                        arms: [
                            Arm {
                                pat: PTuple {
                                    pats: [
                                        PConstr {
                                            vcon: Uident(
                                                "Red",
                                            ),
                                            args: [],
                                        },
                                        PConstr {
                                            vcon: Uident(
                                                "Green",
                                            ),
                                            args: [],
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
                                pat: PTuple {
                                    pats: [
                                        PConstr {
                                            vcon: Uident(
                                                "Red",
                                            ),
                                            args: [],
                                        },
                                        PConstr {
                                            vcon: Uident(
                                                "Blue",
                                            ),
                                            args: [],
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
                    },
                },
            }
        "#]],
    );
}
