#[cfg(test)]
mod tests {

    use std::vec;

    use crate::tast::Arm;
    use crate::tast::Color::{self, *};
    use crate::tast::Expr::{self, *};
    use crate::tast::Pat::{self, *};
    use crate::tast::Ty::{self, *};

    pub fn evar(name: &str, ty: Ty) -> Expr {
        Expr::EVar {
            name: name.to_string(),
            ty,
        }
    }

    pub fn ecolor(value: Color) -> Expr {
        Expr::EColor { value, ty: TColor }
    }

    pub fn eunit() -> Expr {
        Expr::EUnit { ty: TUnit }
    }

    pub fn etuple(items: Vec<Expr>, ty: Ty) -> Expr {
        Expr::ETuple { items, ty }
    }

    pub fn pcolor(value: Color) -> Pat {
        match value {
            Red => Pat::PConstr {
                index: 0,
                args: vec![],
                ty: TColor,
            },
            Green => Pat::PConstr {
                index: 1,
                args: vec![],
                ty: TColor,
            },
            Blue => Pat::PConstr {
                index: 2,
                args: vec![],
                ty: TColor,
            },
        }
    }

    pub fn pwild(ty: Ty) -> Pat {
        Pat::PWild { ty }
    }

    #[test]
    fn test_ast() {
        let make_cc_ty = || TTuple(vec![TColor, TColor]);
        let make_ccc_ty = || TTuple(vec![make_cc_ty(), TColor, TColor]);
        let e = EMatch {
            expr: Box::new(evar("a", make_ccc_ty())),
            arms: vec![
                Arm {
                    pat: PTuple {
                        items: vec![
                            PTuple {
                                items: vec![pcolor(Red), pcolor(Green)],
                                ty: make_cc_ty(),
                            },
                            pcolor(Green),
                            pcolor(Blue),
                        ],
                        ty: make_ccc_ty(),
                    },
                    body: eunit(),
                },
                Arm {
                    pat: PTuple {
                        items: vec![
                            PTuple {
                                items: vec![pcolor(Red), pcolor(Blue)],
                                ty: make_cc_ty(),
                            },
                            pcolor(Red),
                            pcolor(Blue),
                        ],
                        ty: make_ccc_ty(),
                    },
                    body: eunit(),
                },
                Arm {
                    pat: PTuple {
                        items: vec![
                            PTuple {
                                items: vec![pcolor(Blue), pcolor(Green)],
                                ty: make_cc_ty(),
                            },
                            pcolor(Red),
                            pcolor(Green),
                        ],
                        ty: make_ccc_ty(),
                    },
                    body: eunit(),
                },
                Arm {
                    pat: PTuple {
                        items: vec![
                            PTuple {
                                items: vec![pcolor(Blue), pcolor(Red)],
                                ty: make_cc_ty(),
                            },
                            pwild(TColor),
                            pwild(TColor),
                        ],
                        ty: make_ccc_ty(),
                    },
                    body: eunit(),
                },
            ],
            ty: TUnit,
        };
        let result = crate::compile::compile_expr(&e);
        expect_test::expect![[r#"
            match x2 {
              C0 => missing(),
              C1 => match x1 {
                C0 => match x4 {
                  C0 => missing(),
                  C1 => match x3 { C0 => missing(), C1 => missing(), C2 => (), },
                  C2 => missing(),
                },
                C1 => missing(),
                C2 => missing(),
              },
              C2 => match x1 {
                C0 => match x6 {
                  C0 => missing(),
                  C1 => missing(),
                  C2 => match x5 { C0 => (), C1 => missing(), C2 => missing(), },
                },
                C1 => match x8 {
                  C0 => missing(),
                  C1 => match x7 { C0 => (), C1 => missing(), C2 => missing(), },
                  C2 => missing(),
                },
                C2 => missing(),
              },
            }"#]]
        .assert_eq(&result.to_pretty(80))
    }
}
