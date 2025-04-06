use expect_test::{Expect, expect};

fn check(src: &str, expected: Expect, stdout: Expect) {
    let parser = crate::grammar::FileParser::new();
    let ast = parser.parse(src).unwrap();
    let (tast, env) = crate::typer::check_file(ast);
    let core = crate::compile::compile(&env, &tast);
    let mut buffer = String::new();
    let result = crate::interpreter::eval(&im::HashMap::new(), &mut buffer, &core);
    expected.assert_debug_eq(&result);
    stdout.assert_eq(&buffer);
}

#[test]
fn test_001() {
    check(
        "()",
        expect![[r#"
        VUnit
    "#]],
        expect![],
    );
}

#[test]
fn test_002() {
    check(
        r#"
    let a = (true, false) in
    match a {
        (false, false) => (true, true),
        (false, true) => (true, false),
        (true, false) => (false, true),
        (true, true) => (false, false),
    }
    "#,
        expect![[r#"
            VTuple(
                [
                    VBool(
                        false,
                    ),
                    VBool(
                        true,
                    ),
                ],
            )
        "#]],
        expect![],
    );
}

#[test]
fn test_003() {
    check(
        r#"
    let _ = print_bool(true) in
    let _ = print_bool(false) in
    ()
    "#,
        expect![[r#"
            VUnit
        "#]],
        expect!["truefalse"],
    );
}
