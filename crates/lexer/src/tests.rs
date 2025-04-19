use super::*;
use expect_test::{Expect, expect};

#[allow(unused)]
fn check(input: &str, expect: Expect) {
    let toks = lex(input);
    expect.assert_debug_eq(&toks)
}

#[test]
fn test_1() {
    check(
        "let a = 123",
        expect![[r#"
            [
                {kind: LetKeyword, text: "let"},
                {kind: Whitespace, text: " "},
                {kind: Lident, text: "a"},
                {kind: Whitespace, text: " "},
                {kind: Eq, text: "="},
                {kind: Whitespace, text: " "},
                {kind: Int, text: "123"},
            ]
        "#]],
    )
}

#[test]
fn test_2() {
    check(
        "fn f1(x: i32,
                fn f2(x: i32,, z: i32) {}
                fn f3() {}",
        expect![[r#"
            [
                {kind: FnKeyword, text: "fn"},
                {kind: Whitespace, text: " "},
                {kind: Lident, text: "f1"},
                {kind: LParen, text: "("},
                {kind: Lident, text: "x"},
                {kind: Colon, text: ":"},
                {kind: Whitespace, text: " "},
                {kind: Lident, text: "i32"},
                {kind: Comma, text: ","},
                {kind: Whitespace, text: "\n                "},
                {kind: FnKeyword, text: "fn"},
                {kind: Whitespace, text: " "},
                {kind: Lident, text: "f2"},
                {kind: LParen, text: "("},
                {kind: Lident, text: "x"},
                {kind: Colon, text: ":"},
                {kind: Whitespace, text: " "},
                {kind: Lident, text: "i32"},
                {kind: Comma, text: ","},
                {kind: Comma, text: ","},
                {kind: Whitespace, text: " "},
                {kind: Lident, text: "z"},
                {kind: Colon, text: ":"},
                {kind: Whitespace, text: " "},
                {kind: Lident, text: "i32"},
                {kind: RParen, text: ")"},
                {kind: Whitespace, text: " "},
                {kind: LBrace, text: "{"},
                {kind: RBrace, text: "}"},
                {kind: Whitespace, text: "\n                "},
                {kind: FnKeyword, text: "fn"},
                {kind: Whitespace, text: " "},
                {kind: Lident, text: "f3"},
                {kind: LParen, text: "("},
                {kind: RParen, text: ")"},
                {kind: Whitespace, text: " "},
                {kind: LBrace, text: "{"},
                {kind: RBrace, text: "}"},
            ]
        "#]],
    )
}

#[test]
fn test_error_token() {
    check(
        "let a = $ + 123",
        expect![[r#"
            [
                {kind: LetKeyword, text: "let"},
                {kind: Whitespace, text: " "},
                {kind: Lident, text: "a"},
                {kind: Whitespace, text: " "},
                {kind: Eq, text: "="},
                {kind: Whitespace, text: " "},
                {kind: Error, text: "$"},
                {kind: Whitespace, text: " "},
                {kind: Plus, text: "+"},
                {kind: Whitespace, text: " "},
                {kind: Int, text: "123"},
            ]
        "#]],
    )
}

#[test]
fn test_comment() {
    check(
        "// let a = $
            let a = 1
            // comment",
        expect![[r#"
            [
                {kind: Comment, text: "// let a = $"},
                {kind: Whitespace, text: "\n            "},
                {kind: LetKeyword, text: "let"},
                {kind: Whitespace, text: " "},
                {kind: Lident, text: "a"},
                {kind: Whitespace, text: " "},
                {kind: Eq, text: "="},
                {kind: Whitespace, text: " "},
                {kind: Int, text: "1"},
                {kind: Whitespace, text: "\n            "},
                {kind: Comment, text: "// comment"},
            ]
        "#]],
    )
}
