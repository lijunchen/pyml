use expect_test::{Expect, expect};

fn check(src: &str, expected: Expect) {
    let parser = crate::grammar::FileParser::new();
    let ast = parser.parse(src).unwrap();
    expected.assert_eq(&ast.to_pretty(120));
}

#[test]
fn test_001() {
    check("()", expect!["()"]);
}

#[test]
fn test_002() {
    check("(true, false, true)", expect!["(true, false, true)"]);
}

#[test]
fn test_003() {
    check("a.1.2", expect!["proj(proj(a, 1), 2)"]);
}

#[test]
fn test_004() {
    check("f(true, false)", expect!["f(true, false)"]);
}

#[test]
fn test_005() {
    check(
        "(F, F(), F(true, false))",
        expect!["(F, F, F(true, false))"],
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
            match a {
                (false, true) => print_bool(false),
                (false, false) => print_bool(true),
                _ => print_unit(()),
            }"#]],
    );
}

#[test]
fn test_007() {
    check(
        r#"
        let a = true in let b = false in or(a, b)
        "#,
        expect![[r#"
            let a = true in
            let b = false in
            or(a, b)"#]],
    );
}

#[test]
fn test_008() {
    check(
        r#"
        let a = let b = let c = false in c in b in a
        "#,
        expect![[r#"
            let a = let b = let c = false in
            c in
            b in
            a"#]],
    );
}

#[test]
fn test_009() {
    check(
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
            enum Color {
                Red,
                Green,
                Blue
            }

            let c = (Red, Green) in
            match c {
                (Red, Green) => print_bool(true),
                (Red, Blue) => print_bool(false),
                _ => print_unit(()),
            }"#]],
    );
}
