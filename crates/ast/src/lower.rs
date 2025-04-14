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

fn lower_fn(node: cst::Fn) -> Option<ast::Fn> {
    let name = node.lident().unwrap().to_string();
    let params = node.param_list()?.params().flat_map(lower_param).collect();
    let ret_ty = node.return_type().and_then(lower_ty);
    println!("lowering fn params: {:?}", params);
    println!("lowering fn ret_ty: {:?}", ret_ty);
    let body = node.block().and_then(lower_block)?;
    println!("lowering fn body: {:?}", body);
    Some(ast::Fn {
        name: ast::Lident(name),
        params,
        ret_ty,
        body,
    })
}

fn lower_block(node: cst::Block) -> Option<ast::Expr> {
    let cst_e = node.expr();
    let ast_e = cst_e.and_then(lower_expr);
    ast_e
}

fn lower_param(node: cst::Param) -> Option<(ast::Lident, ast::Ty)> {
    let name = node.lident().unwrap().to_string();
    let ty = node.ty().and_then(lower_ty)?;
    Some((ast::Lident(name), ty))
}

fn lower_expr(node: cst::Expr) -> Option<ast::Expr> {
    match node {
        cst::Expr::UnitExpr(_) => Some(ast::Expr::EUnit),
        cst::Expr::BoolExpr(it) => {
            let value = it.value()?.to_string();
            let value = match value.as_str() {
                "true" => Some(ast::Expr::EBool { value: true }),
                "false" => Some(ast::Expr::EBool { value: false }),
                _ => unreachable!(),
            };
            value
        }
        cst::Expr::IntExpr(it) => todo!(),
        cst::Expr::PrimExpr(it) => todo!(),
        cst::Expr::MatchExpr(it) => todo!(),
        cst::Expr::UidentExpr(it) => todo!(),
        cst::Expr::LidentExpr(it) => {
            let name = it.lident().unwrap().to_string();
            Some(ast::Expr::EVar {
                name: ast::Lident(name),
            })
        }
        cst::Expr::TupleExpr(it) => {
            let items = it.exprs().flat_map(lower_expr).collect();
            Some(ast::Expr::ETuple { items })
        }
        cst::Expr::LetExpr(it) => {
            println!("lowering let expr");
            let pat = it.pattern().and_then(lower_pat)?;
            println!("lowering let expr pat: {:?}", pat);
            let value = it.value()?.expr().and_then(lower_expr)?;
            println!("lowering let expr value: {:?}", value);
            let body = it.body()?.expr().and_then(lower_expr)?;
            println!("lowering let expr body: {:?}", body);
            Some(ast::Expr::ELet {
                pat,
                value: Box::new(value),
                body: Box::new(body),
            })
        }
    }
}

fn lower_pat(node: cst::Pattern) -> Option<ast::Pat> {
    match node {
        cst::Pattern::VarPat(it) => {
            let name = it.lident().unwrap().to_string();
            Some(ast::Pat::PVar {
                name: ast::Lident(name),
            })
        }
        cst::Pattern::UnitPat(_) => Some(ast::Pat::PUnit),
        cst::Pattern::BoolPat(it) => {
            todo!()
        }
        cst::Pattern::ConstrPat(it) => todo!(),
        cst::Pattern::TuplePat(it) => todo!(),
        cst::Pattern::WildPat(_) => todo!(),
    }
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
    fn test() {
        let a = (true, false) in
        a
    }
    ";
    let result = parser::parse(&PathBuf::from("dummy"), input);
    let root = MySyntaxNode::new_root(result.green_node);
    let cst: cst::File = cst::File::cast(root).unwrap();
    expect_test::expect![[r#"
        File {
            syntax: File@0..146
              Whitespace@0..5 "\n    "
              Enum@5..81
                EnumKeyword@5..9 "enum"
                Whitespace@9..10 " "
                Uident@10..15 "Color"
                Whitespace@15..16 " "
                LBrace@16..17 "{"
                Whitespace@17..26 "\n        "
                VARIANT_LIST@26..81
                  VARIANT@26..29
                    Uident@26..29 "Red"
                  Comma@29..30 ","
                  Whitespace@30..39 "\n        "
                  VARIANT@39..44
                    Uident@39..44 "Green"
                  Comma@44..45 ","
                  Whitespace@45..54 "\n        "
                  VARIANT@54..69
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
              Fn@81..146
                FnKeyword@81..83 "fn"
                Whitespace@83..84 " "
                Lident@84..88 "test"
                PARAM_LIST@88..91
                  LParen@88..89 "("
                  RParen@89..90 ")"
                  Whitespace@90..91 " "
                BLOCK@91..146
                  LBrace@91..92 "{"
                  Whitespace@92..101 "\n        "
                  EXPR_LET@101..140
                    LetKeyword@101..104 "let"
                    Whitespace@104..105 " "
                    PATTERN_VARIABLE@105..107
                      Lident@105..106 "a"
                      Whitespace@106..107 " "
                    Eq@107..108 "="
                    Whitespace@108..109 " "
                    EXPR_LET_VALUE@109..123
                      EXPR_TUPLE@109..123
                        LParen@109..110 "("
                        EXPR_BOOL@110..114
                          TrueKeyword@110..114 "true"
                        Comma@114..115 ","
                        Whitespace@115..116 " "
                        EXPR_BOOL@116..121
                          FalseKeyword@116..121 "false"
                        RParen@121..122 ")"
                        Whitespace@122..123 " "
                    InKeyword@123..125 "in"
                    Whitespace@125..134 "\n        "
                    EXPR_LET_BODY@134..140
                      EXPR_LIDENT@134..140
                        Lident@134..135 "a"
                        Whitespace@135..140 "\n    "
                  RBrace@140..141 "}"
                  Whitespace@141..146 "\n    "
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
                Fn(
                    Fn {
                        name: Lident(
                            "test",
                        ),
                        params: [],
                        ret_ty: None,
                        body: ELet {
                            pat: PVar {
                                name: Lident(
                                    "a",
                                ),
                            },
                            value: ETuple {
                                items: [
                                    EBool {
                                        value: true,
                                    },
                                    EBool {
                                        value: false,
                                    },
                                ],
                            },
                            body: EVar {
                                name: Lident(
                                    "a",
                                ),
                            },
                        },
                    },
                ),
            ],
            expr: EUnit,
        }
    "#]]
    .assert_debug_eq(&ast);
}
