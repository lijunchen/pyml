mod ast_test;
mod interp_test;
mod smoke_test;
mod typer_test;

#[test]
fn test_cases() -> anyhow::Result<()> {
    let root_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("Root dir: {}", root_dir.display());
    let cases_dir = root_dir.join("src/tests/cases");
    for entry in std::fs::read_dir(&cases_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file()
            && entry.path().extension().and_then(std::ffi::OsStr::to_str) == Some("aaa")
        {
            let p = entry.path();
            let filename = p.file_name().unwrap().to_str().unwrap();
            let ast_filename = p.with_file_name(format!("{}.ast", filename));
            let tast_filename = p.with_file_name(format!("{}.tast", filename));
            let core_filename = p.with_file_name(format!("{}.core", filename));
            let result_filename = p.with_file_name(format!("{}.out", filename));

            let input = std::fs::read_to_string(entry.path())?;
            let parser = crate::grammar::FileParser::new();
            let ast = parser.parse(&input).unwrap();
            expect_test::expect_file![ast_filename].assert_eq(&ast.to_pretty(120));
            let (tast, env) = crate::typer::check_file(ast);
            expect_test::expect_file![tast_filename].assert_eq(&tast.to_pretty(&env, 120));
            let core = crate::compile::compile(&env, &tast);
            expect_test::expect_file![core_filename].assert_eq(&core.to_pretty(&env, 120));
            let mut buf = String::new();
            let env = im::HashMap::new();
            let result = crate::interpreter::eval(&env, &mut buf, &core);
            expect_test::expect_file![result_filename]
                .assert_eq(&format!("{:?}\n====\n{}", result, buf));
        }
    }
    Ok(())
}
