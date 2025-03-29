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

    pub fn estub(name: &str) -> Expr {
        Expr::EPrim {
            func: name.to_string(),
            args: vec![],
            ty: TUnit,
        }
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
                    body: estub("case1"),
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
                    body: estub("case2"),
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
                    body: estub("case3"),
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
                    body: estub("case4"),
                },
            ],
            ty: TUnit,
        };
        expect_test::expect![[r#"
            match a {
                ((Color[0],Color[1]),Color[1],Color[2]) => case1(),
                ((Color[0],Color[2]),Color[0],Color[2]) => case2(),
                ((Color[2],Color[1]),Color[0],Color[1]) => case3(),
                ((Color[2],Color[0]),_,_) => case4(),
            }"#]]
        .assert_eq(&e.to_pretty(120));
        let result = crate::compile::compile_expr(&e);
        expect_test::expect![[r#"
            match x2 {
              Color[0] => missing(),
              Color[1] => match x1 {
                Color[0] => match x4 {
                  Color[0] => missing(),
                  Color[1] => match x3 { Color[0] => missing(), Color[1] => missing(), Color[2] => case3(), },
                  Color[2] => missing(),
                },
                Color[1] => missing(),
                Color[2] => missing(),
              },
              Color[2] => match x1 {
                Color[0] => match x6 {
                  Color[0] => missing(),
                  Color[1] => missing(),
                  Color[2] => match x5 { Color[0] => case2(), Color[1] => missing(), Color[2] => missing(), },
                },
                Color[1] => match x8 {
                  Color[0] => missing(),
                  Color[1] => match x7 { Color[0] => case1(), Color[1] => missing(), Color[2] => missing(), },
                  Color[2] => missing(),
                },
                Color[2] => missing(),
              },
            }"#]]
        .assert_eq(&result.to_pretty(120))
    }
}
