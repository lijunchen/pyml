use cst::cst::CstNode;
use parser::{debug_tree, syntax::MySyntaxNode};

#[test]
fn test_cases() -> anyhow::Result<()> {
    let root_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let cases_dir = root_dir.join("src/tests/cases");
    for entry in std::fs::read_dir(&cases_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file()
            && entry.path().extension().and_then(std::ffi::OsStr::to_str) == Some("src")
        {
            let p = entry.path();
            let filename = p.file_name().unwrap().to_str().unwrap();
            let cst_filename = p.with_file_name(format!("{}.cst", filename));
            let ast_filename = p.with_file_name(format!("{}.ast", filename));
            let tast_filename = p.with_file_name(format!("{}.tast", filename));
            let core_filename = p.with_file_name(format!("{}.core", filename));
            let result_filename = p.with_file_name(format!("{}.out", filename));

            let input = std::fs::read_to_string(entry.path())?;

            {
                let result = parser::parse(&p, &input);
                expect_test::expect_file![cst_filename].assert_eq(&debug_tree(&result.green_node));
            }

            let result = parser::parse(&p, &input);
            let root = MySyntaxNode::new_root(result.green_node);
            let cst = cst::cst::File::cast(root).unwrap();
            let ast = ast::lower::lower(cst).unwrap();

            expect_test::expect_file![ast_filename].assert_eq(&ast.to_pretty(120));
            let (tast, env) = crate::typer::check_file(ast);
            expect_test::expect_file![tast_filename].assert_eq(&tast.to_pretty(&env, 120));
            let core = crate::compile_match::compile_file(&env, &tast);
            expect_test::expect_file![core_filename].assert_eq(&core.to_pretty(&env, 120));
            let mut buf = String::new();
            let env = im::HashMap::new();
            let result = crate::interpreter::eval_file(&env, &mut buf, &core);
            expect_test::expect_file![result_filename]
                .assert_eq(&format!("{:?}\n====\n{}", result, buf));
        }
    }
    Ok(())
}
