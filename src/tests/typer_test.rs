use expect_test::{Expect, expect};

fn check(src: &str, expected: Expect) {
    let parser = crate::grammar::FileParser::new();
    let ast = parser.parse(src).unwrap();
    let (tast, env) = crate::typer::check_file(ast);
    expected.assert_eq(&tast.to_pretty(&env, 120));
}

#[test]
fn test_001() {
    check("()", expect!["()"]);
}

#[test]
fn test_002() {
    check(
        "let (a, b) = (false, ()) in (b, a)",
        expect![[r#"
            let (a,b) = ( false,  () );
            ( b : (),  a : bool )"#]],
    );
}
