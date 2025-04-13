use std::vec;

use ast::ast::Uident;
use expect_test::expect;

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
    TEnum {
        name: Uident::new("Color"),
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

fn check(node: &Expr, pp_ast: expect_test::Expect, pp_core: expect_test::Expect) {
    let env = crate::env::Env::toy_env();
    let ast = node.to_pretty(&env, 120);
    let core = crate::compile_match::compile(&env, node);
    let core = core.to_pretty(&env, 120);
    pp_ast.assert_eq(&ast);
    pp_core.assert_eq(&core);
}

#[test]
fn test_ast() {
    let make_cc_ty = || TTuple {
        typs: vec![tcolor(), tcolor()],
    };
    let make_ccc_ty = || TTuple {
        typs: vec![make_cc_ty(), tcolor(), tcolor()],
    };
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
    check(
        &e,
        expect![[r#"
            match (a : ((Color, Color), Color, Color)) {
                ((Color::Red,Color::Green),Color::Green,Color::Blue) => case1(),
                ((Color::Red,Color::Blue),Color::Red,Color::Blue) => case2(),
                ((Color::Blue,Color::Green),Color::Red,Color::Green) => case3(),
                ((Color::Blue,Color::Red),_ : Color,_ : Color) => case4(),
            }"#]],
        expect![[r#"
            let x0 = a.0 in
            let x1 = a.1 in
            let x2 = a.2 in
            let x3 = x0.0 in
            let x4 = x0.1 in
            match x4 {
              Color::Red => {
                match x3 {
                  Color::Red => {
                    missing()
                  },
                  Color::Green => {
                    missing()
                  },
                  Color::Blue => {
                    case4()
                  },
                }
              },
              Color::Green => {
                match x2 {
                  Color::Red => {
                    missing()
                  },
                  Color::Green => {
                    match x1 {
                      Color::Red => {
                        match x3 {
                          Color::Red => {
                            missing()
                          },
                          Color::Green => {
                            missing()
                          },
                          Color::Blue => {
                            case3()
                          },
                        }
                      },
                      Color::Green => {
                        missing()
                      },
                      Color::Blue => {
                        missing()
                      },
                    }
                  },
                  Color::Blue => {
                    match x1 {
                      Color::Red => {
                        missing()
                      },
                      Color::Green => {
                        match x3 {
                          Color::Red => {
                            case1()
                          },
                          Color::Green => {
                            missing()
                          },
                          Color::Blue => {
                            missing()
                          },
                        }
                      },
                      Color::Blue => {
                        missing()
                      },
                    }
                  },
                }
              },
              Color::Blue => {
                match x2 {
                  Color::Red => {
                    missing()
                  },
                  Color::Green => {
                    missing()
                  },
                  Color::Blue => {
                    match x1 {
                      Color::Red => {
                        match x3 {
                          Color::Red => {
                            case2()
                          },
                          Color::Green => {
                            missing()
                          },
                          Color::Blue => {
                            missing()
                          },
                        }
                      },
                      Color::Green => {
                        missing()
                      },
                      Color::Blue => {
                        missing()
                      },
                    }
                  },
                }
              },
            }"#]],
    );
}

#[test]
fn test_ast_002() {
    let e = EMatch {
        expr: Box::new(evar("a", TUnit)),
        arms: vec![Arm {
            pat: PUnit { ty: TUnit },
            body: estub("case1"),
        }],
        ty: TUnit,
    };
    check(
        &e,
        expect![[r#"
            match (a : ()) {
                () => case1(),
            }"#]],
        expect![[r#"
            match a {
              () => {
                case1()
              },
            }"#]],
    );
}

#[test]
fn test_ast_003() {
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
    check(
        &e,
        expect![[r#"
            match (a : Color) {
                Color::Green => case1(),
                Color::Blue => case2(),
                Color::Red => case3(),
            }"#]],
        expect![[r#"
            match a {
              Color::Red => {
                case3()
              },
              Color::Green => {
                case1()
              },
              Color::Blue => {
                case2()
              },
            }"#]],
    );
}

#[test]
fn test_ast_004() {
    let make_cc_ty = || TTuple {
        typs: vec![tcolor(), tcolor()],
    };
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
    check(
        &e,
        expect![[r#"
            match (a : (Color, Color)) {
                (Color::Green,Color::Green) => case1(),
                (Color::Green,Color::Red) => case2(),
                (Color::Green,t) => case3(),
            }"#]],
        expect![[r#"
            let x0 = a.0 in
            let x1 = a.1 in
            match x0 {
              Color::Red => {
                missing()
              },
              Color::Green => {
                match x1 {
                  Color::Red => {
                    case2()
                  },
                  Color::Green => {
                    case1()
                  },
                  Color::Blue => {
                    let t = x1 in
                    case3()
                  },
                }
              },
              Color::Blue => {
                missing()
              },
            }"#]],
    );
}

#[test]
fn test_ast_005() {
    let make_cb_ty = || TTuple {
        typs: vec![tcolor(), TBool],
    };
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
    check(
        &e,
        expect![[r#"
            match (a : (Color, bool)) {
                (Color::Green,false) => case1(),
                (Color::Green,true) => case2(),
                (Color::Red,true) => case3(),
            }"#]],
        expect![[r#"
            let x0 = a.0 in
            let x1 = a.1 in
            match x1 {
              true => {
                match x0 {
                  Color::Red => {
                    case3()
                  },
                  Color::Green => {
                    case2()
                  },
                  Color::Blue => {
                    missing()
                  },
                }
              },
              false => {
                match x0 {
                  Color::Red => {
                    missing()
                  },
                  Color::Green => {
                    case1()
                  },
                  Color::Blue => {
                    missing()
                  },
                }
              },
            }"#]],
    );
}

#[test]
fn test_ast_006() {
    let make_bb_ty = || TTuple {
        typs: vec![TBool, TBool],
    };
    let e = EMatch {
        expr: Box::new(evar("a", make_bb_ty())),
        arms: vec![
            Arm {
                pat: PTuple {
                    items: vec![
                        PBool {
                            value: false,
                            ty: TBool,
                        },
                        PBool {
                            value: false,
                            ty: TBool,
                        },
                    ],
                    ty: make_bb_ty(),
                },
                body: estub("case1"),
            },
            Arm {
                pat: PTuple {
                    items: vec![
                        PBool {
                            value: false,
                            ty: TBool,
                        },
                        PBool {
                            value: true,
                            ty: TBool,
                        },
                    ],
                    ty: make_bb_ty(),
                },
                body: estub("case2"),
            },
            Arm {
                pat: PTuple {
                    items: vec![
                        PBool {
                            value: true,
                            ty: TBool,
                        },
                        PVar {
                            name: "t".to_string(),
                            ty: TBool,
                        },
                    ],
                    ty: make_bb_ty(),
                },
                body: estub("case3"),
            },
        ],
        ty: make_bb_ty(),
    };
    check(
        &e,
        expect![[r#"
            match (a : (bool, bool)) {
                (false,false) => case1(),
                (false,true) => case2(),
                (true,t) => case3(),
            }"#]],
        expect![[r#"
            let x0 = a.0 in
            let x1 = a.1 in
            match x0 {
              true => {
                let t = x1 in
                case3()
              },
              false => {
                match x1 {
                  true => {
                    case2()
                  },
                  false => {
                    case1()
                  },
                }
              },
            }"#]],
    );
}

#[test]
fn test_ast_007() {
    let expr_ty = || Ty::TEnum {
        name: Uident::new("Expr"),
    };
    let zero = || Pat::PConstr {
        index: 0,
        args: vec![],
        ty: expr_ty(),
    };
    let succ = |e| Pat::PConstr {
        index: 1,
        args: vec![e],
        ty: expr_ty(),
    };
    let add = |e1, e2| Pat::PConstr {
        index: 2,
        args: vec![e1, e2],
        ty: expr_ty(),
    };
    let mul = |e1, e2| Pat::PConstr {
        index: 3,
        args: vec![e1, e2],
        ty: expr_ty(),
    };
    let var = |name: &str| Pat::PVar {
        name: name.to_string(),
        ty: expr_ty(),
    };
    // match a with
    //| Add(Zero, Zero) => e1
    //| Mul(Zero, x) => e2
    //| Add(Succ(x), y) => e3
    //| Mul(x, Zero) => e4
    //| Mul(Add(x, y), z) => e5
    //| Add(x, Zero) => e6
    //| x => e7
    let e = EMatch {
        expr: Box::new(evar("a", expr_ty())),
        arms: vec![
            Arm {
                pat: add(zero(), zero()),
                body: estub("e1"),
            },
            Arm {
                pat: mul(zero(), var("x")),
                body: estub("e2"),
            },
            Arm {
                pat: add(succ(var("x")), var("y")),
                body: estub("e3"),
            },
            Arm {
                pat: mul(var("x"), zero()),
                body: estub("e4"),
            },
            Arm {
                pat: mul(add(var("x"), var("y")), var("z")),
                body: estub("e5"),
            },
            Arm {
                pat: add(var("x"), zero()),
                body: estub("e6"),
            },
            Arm {
                pat: var("x"),
                body: estub("e7"),
            },
        ],
        ty: TUnit,
    };
    check(
        &e,
        expect![[r#"
            match (a : Expr) {
                Expr::Add(Expr::Zero,Expr::Zero) => e1(),
                Expr::Mul(Expr::Zero,x) => e2(),
                Expr::Add(Expr::Succ(x),y) => e3(),
                Expr::Mul(x,Expr::Zero) => e4(),
                Expr::Mul(Expr::Add(x,y),z) => e5(),
                Expr::Add(x,Expr::Zero) => e6(),
                x => e7(),
            }"#]],
        expect![[r#"
            match a {
              Expr::Zero => {
                let x = a in
                e7()
              },
              Expr::Succ(x0) => {
                let x0 = Expr_1_0(a) in
                let x = a in
                e7()
              },
              Expr::Add(x1,x2) => {
                let x2 = Expr_2_1(a) in
                let x1 = Expr_2_0(a) in
                match x2 {
                  Expr::Zero => {
                    match x1 {
                      Expr::Zero => {
                        e1()
                      },
                      Expr::Succ(x10) => {
                        let x10 = Expr_1_0(x1) in
                        let x = x10 in
                        let y = x2 in
                        e3()
                      },
                      Expr::Add(x11,x12) => {
                        let x12 = Expr_2_1(x1) in
                        let x11 = Expr_2_0(x1) in
                        let x = x1 in
                        e6()
                      },
                      Expr::Mul(x13,x14) => {
                        let x14 = Expr_3_1(x1) in
                        let x13 = Expr_3_0(x1) in
                        let x = x1 in
                        e6()
                      },
                    }
                  },
                  Expr::Succ(x5) => {
                    let x5 = Expr_1_0(x2) in
                    match x1 {
                      Expr::Zero => {
                        let x = a in
                        e7()
                      },
                      Expr::Succ(x15) => {
                        let x15 = Expr_1_0(x1) in
                        let x = x15 in
                        let y = x2 in
                        e3()
                      },
                      Expr::Add(x16,x17) => {
                        let x17 = Expr_2_1(x1) in
                        let x16 = Expr_2_0(x1) in
                        let x = a in
                        e7()
                      },
                      Expr::Mul(x18,x19) => {
                        let x19 = Expr_3_1(x1) in
                        let x18 = Expr_3_0(x1) in
                        let x = a in
                        e7()
                      },
                    }
                  },
                  Expr::Add(x6,x7) => {
                    let x7 = Expr_2_1(x2) in
                    let x6 = Expr_2_0(x2) in
                    match x1 {
                      Expr::Zero => {
                        let x = a in
                        e7()
                      },
                      Expr::Succ(x20) => {
                        let x20 = Expr_1_0(x1) in
                        let x = x20 in
                        let y = x2 in
                        e3()
                      },
                      Expr::Add(x21,x22) => {
                        let x22 = Expr_2_1(x1) in
                        let x21 = Expr_2_0(x1) in
                        let x = a in
                        e7()
                      },
                      Expr::Mul(x23,x24) => {
                        let x24 = Expr_3_1(x1) in
                        let x23 = Expr_3_0(x1) in
                        let x = a in
                        e7()
                      },
                    }
                  },
                  Expr::Mul(x8,x9) => {
                    let x9 = Expr_3_1(x2) in
                    let x8 = Expr_3_0(x2) in
                    match x1 {
                      Expr::Zero => {
                        let x = a in
                        e7()
                      },
                      Expr::Succ(x25) => {
                        let x25 = Expr_1_0(x1) in
                        let x = x25 in
                        let y = x2 in
                        e3()
                      },
                      Expr::Add(x26,x27) => {
                        let x27 = Expr_2_1(x1) in
                        let x26 = Expr_2_0(x1) in
                        let x = a in
                        e7()
                      },
                      Expr::Mul(x28,x29) => {
                        let x29 = Expr_3_1(x1) in
                        let x28 = Expr_3_0(x1) in
                        let x = a in
                        e7()
                      },
                    }
                  },
                }
              },
              Expr::Mul(x3,x4) => {
                let x4 = Expr_3_1(a) in
                let x3 = Expr_3_0(a) in
                match x3 {
                  Expr::Zero => {
                    let x = x4 in
                    e2()
                  },
                  Expr::Succ(x30) => {
                    let x30 = Expr_1_0(x3) in
                    match x4 {
                      Expr::Zero => {
                        let x = x3 in
                        e4()
                      },
                      Expr::Succ(x35) => {
                        let x35 = Expr_1_0(x4) in
                        let x = a in
                        e7()
                      },
                      Expr::Add(x36,x37) => {
                        let x37 = Expr_2_1(x4) in
                        let x36 = Expr_2_0(x4) in
                        let x = a in
                        e7()
                      },
                      Expr::Mul(x38,x39) => {
                        let x39 = Expr_3_1(x4) in
                        let x38 = Expr_3_0(x4) in
                        let x = a in
                        e7()
                      },
                    }
                  },
                  Expr::Add(x31,x32) => {
                    let x32 = Expr_2_1(x3) in
                    let x31 = Expr_2_0(x3) in
                    match x4 {
                      Expr::Zero => {
                        let x = x3 in
                        e4()
                      },
                      Expr::Succ(x40) => {
                        let x40 = Expr_1_0(x4) in
                        let y = x32 in
                        let x = x31 in
                        let z = x4 in
                        e5()
                      },
                      Expr::Add(x41,x42) => {
                        let x42 = Expr_2_1(x4) in
                        let x41 = Expr_2_0(x4) in
                        let y = x32 in
                        let x = x31 in
                        let z = x4 in
                        e5()
                      },
                      Expr::Mul(x43,x44) => {
                        let x44 = Expr_3_1(x4) in
                        let x43 = Expr_3_0(x4) in
                        let y = x32 in
                        let x = x31 in
                        let z = x4 in
                        e5()
                      },
                    }
                  },
                  Expr::Mul(x33,x34) => {
                    let x34 = Expr_3_1(x3) in
                    let x33 = Expr_3_0(x3) in
                    match x4 {
                      Expr::Zero => {
                        let x = x3 in
                        e4()
                      },
                      Expr::Succ(x45) => {
                        let x45 = Expr_1_0(x4) in
                        let x = a in
                        e7()
                      },
                      Expr::Add(x46,x47) => {
                        let x47 = Expr_2_1(x4) in
                        let x46 = Expr_2_0(x4) in
                        let x = a in
                        e7()
                      },
                      Expr::Mul(x48,x49) => {
                        let x49 = Expr_3_1(x4) in
                        let x48 = Expr_3_0(x4) in
                        let x = a in
                        e7()
                      },
                    }
                  },
                }
              },
            }"#]],
    );
}

#[test]
fn test_ast_008() {
    let e = EMatch {
        expr: Box::new(evar("a", tcolor())),
        arms: vec![
            Arm {
                pat: pgreen(),
                body: estub("case1"),
            },
            Arm {
                pat: PWild { ty: tcolor() },
                body: estub("case2"),
            },
        ],
        ty: TUnit,
    };
    check(
        &e,
        expect![[r#"
            match (a : Color) {
                Color::Green => case1(),
                _ : Color => case2(),
            }"#]],
        expect![[r#"
            match a {
              Color::Red => {
                case2()
              },
              Color::Green => {
                case1()
              },
              Color::Blue => {
                case2()
              },
            }"#]],
    );
}

#[test]
fn test_ast_009() {
    let make_cc_ty = || TTuple {
        typs: vec![tcolor(), tcolor(), tcolor()],
    };

    let e = EMatch {
        expr: Box::new(evar("a", make_cc_ty())),
        arms: vec![
            Arm {
                pat: PTuple {
                    items: vec![pred(), pwild(tcolor()), pred()],
                    ty: make_cc_ty(),
                },
                body: estub("case1"),
            },
            Arm {
                pat: PTuple {
                    items: vec![pwild(tcolor()), pblue(), pwild(tcolor())],
                    ty: make_cc_ty(),
                },
                body: estub("case2"),
            },
            Arm {
                pat: PWild { ty: make_cc_ty() },
                body: estub("case3"),
            },
        ],
        ty: TUnit,
    };
    check(
        &e,
        expect![[r#"
            match (a : (Color, Color, Color)) {
                (Color::Red,_ : Color,Color::Red) => case1(),
                (_ : Color,Color::Blue,_ : Color) => case2(),
                _ : (Color, Color, Color) => case3(),
            }"#]],
        expect![[r#"
            let x0 = a.0 in
            let x1 = a.1 in
            let x2 = a.2 in
            match x2 {
              Color::Red => {
                match x0 {
                  Color::Red => {
                    case1()
                  },
                  Color::Green => {
                    match x1 {
                      Color::Red => {
                        case3()
                      },
                      Color::Green => {
                        case3()
                      },
                      Color::Blue => {
                        case2()
                      },
                    }
                  },
                  Color::Blue => {
                    match x1 {
                      Color::Red => {
                        case3()
                      },
                      Color::Green => {
                        case3()
                      },
                      Color::Blue => {
                        case2()
                      },
                    }
                  },
                }
              },
              Color::Green => {
                match x1 {
                  Color::Red => {
                    case3()
                  },
                  Color::Green => {
                    case3()
                  },
                  Color::Blue => {
                    case2()
                  },
                }
              },
              Color::Blue => {
                match x1 {
                  Color::Red => {
                    case3()
                  },
                  Color::Green => {
                    case3()
                  },
                  Color::Blue => {
                    case2()
                  },
                }
              },
            }"#]],
    );
}

#[test]
fn test_ast_010() {
    let e = ELet {
        pat: PBool {
            value: true,
            ty: TBool,
        },
        value: Box::new(EVar {
            name: "x".to_string(),
            ty: TBool,
        }),
        body: Box::new(EUnit { ty: TUnit }),
        ty: TUnit,
    };
    check(
        &e,
        expect![[r#"
            let true = (x : bool);
            ()"#]],
        expect![[r#"
            let mtmp0 = x in
            match mtmp0 {
              true => {
                ()
              },
              false => {
                missing()
              },
            }"#]],
    );
}

#[test]
fn test_ast_011() {
    let make_bb_ty = || TTuple {
        typs: vec![TBool, TBool],
    };
    let e = ELet {
        pat: PTuple {
            items: vec![
                PVar {
                    name: "a".to_string(),
                    ty: TBool,
                },
                PBool {
                    value: false,
                    ty: TBool,
                },
            ],
            ty: make_bb_ty(),
        },
        value: Box::new(EVar {
            name: "bools".to_string(),
            ty: make_bb_ty(),
        }),
        body: Box::new(EPrim {
            func: "print_bool".to_string(),
            args: vec![EVar {
                name: "a".to_string(),
                ty: TBool,
            }],
            ty: TUnit,
        }),
        ty: TUnit,
    };
    check(
        &e,
        expect![[r#"
            let (a,false) = (bools : (bool, bool));
            print_bool( (a : bool) )"#]],
        expect![[r#"
            let mtmp0 = bools in
            let x1 = mtmp0.0 in
            let x2 = mtmp0.1 in
            match x2 {
              true => {
                missing()
              },
              false => {
                let a = x1 in
                print_bool(a)
              },
            }"#]],
    );
}
