use crate::ast;

use cst::cst;

pub fn lower(node: cst::File) -> Option<ast::File> {
    let items = node.items().flat_map(lower_item).collect();
    let ast = ast::File {
        toplevels: items,
        expr: ast::Expr::EUnit,
    };
    Some(ast)
}

fn lower_item(node: cst::Item) -> Option<ast::Item> {
    match node {
        cst::Item::Enum(it) => Some(ast::Item::EnumDef(lower_enum(it)?)),
        cst::Item::Fn(it) => Some(ast::Item::Fn(lower_fn(it)?)),
    }
}

fn lower_enum(node: cst::Enum) -> Option<ast::EnumDef> {
    let name = node.uident().unwrap().to_string();
    let variants = node
        .variant_list()?
        .variants()
        .flat_map(lower_variant)
        .collect();
    Some(ast::EnumDef {
        name: ast::Uident::new(&name),
        variants,
    })
}

fn lower_variant(node: cst::Variant) -> Option<(ast::Uident, Vec<ast::Ty>)> {
    let name = node.uident().unwrap().to_string();
    let typs = match node.type_list() {
        None => vec![],
        Some(xs) => xs.types().flat_map(lower_ty).collect(),
    };
    Some((ast::Uident::new(&name), typs))
}

fn lower_ty(node: cst::Type) -> Option<ast::Ty> {
    match node {
        cst::Type::UnitTy(_) => Some(ast::Ty::TUnit),
        cst::Type::BoolTy(_) => Some(ast::Ty::TBool),
        cst::Type::IntTy(_) => Some(ast::Ty::TInt),
        cst::Type::TupleTy(it) => Some(ast::Ty::TTuple {
            typs: it.types().flat_map(lower_ty).collect(),
        }),
        cst::Type::EnumTy(it) => Some(ast::Ty::TEnum {
            name: ast::Uident::new(&it.uident().unwrap().to_string()),
        }),
        cst::Type::FuncTy(..) => todo!(),
    }
}

fn lower_fn(_node: cst::Fn) -> Option<ast::Fn> {
    None
}

#[allow(unused)]
fn lower_expr(_node: cst::Expr) -> Option<ast::Expr> {
    None
}

#[allow(unused)]
fn lower_pat(_node: cst::Pattern) -> Option<ast::Pat> {
    None
}

#[test]
fn smoke_test() {
    use ::cst::cst::CstNode;
    use parser::syntax::MySyntaxNode;
    use std::path::PathBuf;

    let input = "
    enum Color {
        Red,
        Green,
        Blue(Int, Bool),
    }
    ()
    ";
    let result = parser::parse(&PathBuf::from("dummy"), input);
    let root = MySyntaxNode::new_root(result.green_node);
    let cst: cst::File = cst::File::cast(root).unwrap();
    expect_test::expect![[r#"
        File {
            syntax: File@0..88
              Whitespace@0..5 "\n    "
              Enum@5..81
                EnumKeyword@5..9 "enum"
                Whitespace@9..10 " "
                Uident@10..15 "Color"
                Whitespace@15..16 " "
                LBrace@16..17 "{"
                Whitespace@17..26 "\n        "
                VariantList@26..81
                  Variant@26..29
                    Uident@26..29 "Red"
                  Comma@29..30 ","
                  Whitespace@30..39 "\n        "
                  Variant@39..44
                    Uident@39..44 "Green"
                  Comma@44..45 ","
                  Whitespace@45..54 "\n        "
                  Variant@54..69
                    Uident@54..58 "Blue"
                    TYPE_LIST@58..69
                      LParen@58..59 "("
                      TYPE_INT@59..62
                        IntKeyword@59..62 "Int"
                      Comma@62..63 ","
                      Whitespace@63..64 " "
                      TYPE_BOOL@64..68
                        BoolKeyword@64..68 "Bool"
                      RParen@68..69 ")"
                  Comma@69..70 ","
                  Whitespace@70..75 "\n    "
                  RBrace@75..76 "}"
                  Whitespace@76..81 "\n    "
              ExprUnit@81..88
                LParen@81..82 "("
                RParen@82..83 ")"
                Whitespace@83..88 "\n    "
            ,
        }
    "#]]
    .assert_debug_eq(&cst);
    let ast: ast::File = lower(cst).unwrap();
    expect_test::expect![[r#"
        File {
            toplevels: [
                EnumDef(
                    EnumDef {
                        name: Uident(
                            "Color",
                        ),
                        variants: [
                            (
                                Uident(
                                    "Red",
                                ),
                                [],
                            ),
                            (
                                Uident(
                                    "Green",
                                ),
                                [],
                            ),
                            (
                                Uident(
                                    "Blue",
                                ),
                                [
                                    TInt,
                                    TBool,
                                ],
                            ),
                        ],
                    },
                ),
            ],
            expr: EUnit,
        }
    "#]]
    .assert_debug_eq(&ast);
}
