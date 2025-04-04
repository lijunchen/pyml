use expect_test::{Expect, expect};

fn check(src: &str, expected: Expect) {
    let parser = crate::grammar::FileParser::new();
    let ast = parser.parse(src).unwrap();
    let (tast, env) = crate::typer::check_file(ast);
    expected.assert_debug_eq(&tast);
}

#[test]
fn test_001() {
    check(
        "()",
        expect![[r#"
        EUnit {
            ty: TUnit,
        }
    "#]],
    );
}

#[test]
fn test_002() {
    check(
        "let a = true in a",
        expect![[r#"
            ELet {
                pat: PVar {
                    name: "a",
                    ty: TBool,
                },
                value: EBool {
                    value: true,
                    ty: TBool,
                },
                body: EVar {
                    name: "a",
                    ty: TBool,
                },
                ty: TBool,
            }
        "#]],
    );
}
