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
            let (a,b) = (false, ());
            ((b : ()), (a : bool))"#]],
    );
}

#[test]
fn test_003() {
    check(
        "let a = (false, ()) in (a.1, a.0)",
        expect![[r#"
            let a = (false, ());
            ((a : (bool, ())).1, (a : (bool, ())).0)"#]],
    );
}

#[test]
fn test_004() {
    check(
        "let a = (false, ()) in let b = a.1 in b",
        expect![[r#"
            let a = (false, ());
            let b = (a : (bool, ())).1;
            (b : ())"#]],
    );
}

#[test]
fn test_005() {
    check(
        r#"
        match (true, false) {
            (true, false) => true,
            (true, true) => true,
            _ => false,
        }
        "#,
        expect![[r#"
            match (true, false) {
                (true,false) => true,
                (true,true) => true,
                _ : (bool, bool) => false,
            }"#]],
    );
}
