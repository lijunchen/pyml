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
        ()
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
            (false, true)
        "#]],
        expect![],
    );
}

#[test]
fn test_003() {
    check(
        r#"
    let _ = print_unit(()) in
    let _ = print_bool(true) in
    let _ = print_bool(false) in
    let _ = print_int(123) in
    ()
    "#,
        expect![[r#"
            ()
        "#]],
        expect!["()truefalse123"],
    );
}

#[test]
fn test_004() {
    check(
        r#"
        enum Color {
            Red,
            Green,
            Blue,
        }
        let a = (Blue, Blue) in
        match a {
            (Red, Green) => true,
            (Red, Red) => true,
            (Blue, Blue) => let _ = print_bool(true) in false,
            _ => false,
        }
        "#,
        expect![[r#"
            false
        "#]],
        expect!["true"],
    );
}

#[test]
fn test_005() {
    check(
        r#"
        enum Color {
            Red,
            Green,
            Blue,
        }
        let a = (Blue, Red) in
        match a {
            (Red, Green) => print_int(0),
            (Red, Red) => print_int(1),
            (Blue, Blue) => print_int(2),
            _ => print_int(3),
        }
        "#,
        expect![[r#"
            ()
        "#]],
        expect!["3"],
    );
}

#[test]
fn test_006() {
    check(
        r#"
        let a = (true, false) in
        match a {
            (true, b) => print_bool(b),
            _ => (),
        }
        "#,
        expect![[r#"
            ()
        "#]],
        expect!["false"],
    );
    check(
        r#"
        let a = (true, true) in
        match a {
            (true, b) => print_bool(b),
            _ => (),
        }
        "#,
        expect![[r#"
            ()
        "#]],
        expect!["true"],
    );
}

#[test]
#[ignore]
fn test_007() {
    check(
        r#"
        enum Expr {
            Zero,
            Succ(Expr),
            Add(Expr, Expr),
            Mul(Expr, Expr),
        }
        
        let a = Mul(Add(Zero,Zero),Zero) in
        match a {
            Add(Zero,Zero) => print_int(0),
            Mul(Zero,x) => print_int(1),
            Add(Succ(x),y) => print_int(2),
            Mul(x,Zero) => print_int(3),
            Mul(Add(x,y),z) => print_int(4),
            Add(x,Zero) => print_int(5),
            x => print_int(6),
        }
        "#,
        expect![[r#"
            ()
        "#]],
        expect!["3"],
    );
}
