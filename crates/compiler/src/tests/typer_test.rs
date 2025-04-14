use cst::cst::CstNode;
use expect_test::{Expect, expect};
use parser::syntax::MySyntaxNode;

fn check(src: &str, expected: Expect) {
    let result = parser::parse(&std::path::PathBuf::from("dummy"), &src);
    let root = MySyntaxNode::new_root(result.green_node);
    let cst = cst::cst::File::cast(root).unwrap();
    let ast = ast::lower::lower(cst).unwrap();

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
            ((b : Unit), (a : Bool))"#]],
    );
}

#[test]
fn test_003() {
    check(
        "let a = (false, ()) in (a.1, a.0)",
        expect![[r#"
            let a = (false, ());
            ((a : (Bool, Unit)).1, (a : (Bool, Unit)).0)"#]],
    );
}

#[test]
fn test_004() {
    check(
        "let a = (false, ()) in let b = a.1 in b",
        expect![[r#"
            let a = (false, ());
            let b = (a : (Bool, Unit)).1;
            (b : Unit)"#]],
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
                _ : (Bool, Bool) => false,
            }"#]],
    );
}

#[test]
fn test_006() {
    check(
        r#"
        enum Color {
            Red,
            Green,
            Blue,
        }
        let a = (Red, Green) in
        match a {
            (Red, Green) => true,
            (Red, Red) => true,
            _ => false,
        }
        "#,
        expect![[r#"
            let a = (Color::Red, Color::Green);
            match (a : (Color, Color)) {
                (Color::Red,Color::Green) => true,
                (Color::Red,Color::Red) => true,
                _ : (Color, Color) => false,
            }"#]],
    );
}

#[test]
fn test_007() {
    check(
        r#"
        enum Expr {
            Zero,
            Succ(Expr),
            Add(Expr, Expr),
            Mul(Expr, Expr),
        }
        
        let a = Zero in
        match a {
            Add(Zero,Zero) => (),
            Mul(Zero,x) => (),
            Add(Succ(x),y) => (),
            Mul(x,Zero) => (),
            Mul(Add(x,y),z) => (),
            Add(x,Zero) => (),
            x => (),
        }
        "#,
        expect![[r#"
            let a = Expr::Zero;
            match (a : Expr) {
                Expr::Add(Expr::Zero,Expr::Zero) => (),
                Expr::Mul(Expr::Zero,x) => (),
                Expr::Add(Expr::Succ(x),y) => (),
                Expr::Mul(x,Expr::Zero) => (),
                Expr::Mul(Expr::Add(x,y),z) => (),
                Expr::Add(x,Expr::Zero) => (),
                x => (),
            }"#]],
    );
}
