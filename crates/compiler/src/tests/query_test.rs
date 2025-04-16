use expect_test::{Expect, expect};

use crate::query::hover_type;

fn check(src: &str, line: u32, col: u32, expected: Expect) {
    let result = hover_type(src, line, col);
    expected.assert_debug_eq(&result.unwrap_or("<None>".to_string()));
}

#[test]
#[rustfmt::skip]
fn smoke_test() {
    let src = r#"enum Color { Red, Green, Blue }

fn main() {
    let a = 1 in
    let a = (true, 2) in
    let a = Green in
    ()
}
"#;

    check(src, 3, 8,expect![[r#"
        "Int"
    "#]],);
    check(src, 3, 9,expect![[r#"
        "Int"
    "#]],);
    check(src, 3, 10,expect![[r#"
        "<None>"
    "#]],);

    check(src, 4, 8, expect![[r#"
        "(Bool, Int)"
    "#]]);

    check(src, 5, 8, expect![[r#"
        "Color"
    "#]]);
}
