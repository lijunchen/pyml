use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn execute(src: &str) -> String {
    let parser = compiler::grammar::FileParser::new();
    let ast = parser.parse(src).unwrap();
    let (tast, env) = compiler::typer::check_file(ast);
    let core = compiler::compile_match::compile(&env, &tast);
    let mut buffer = String::new();
    let _result = compiler::interpreter::eval(&im::HashMap::new(), &mut buffer, &core);
    return buffer;
}

#[wasm_bindgen]
pub fn compile_to_core(src: &str) -> String {
    let parser = compiler::grammar::FileParser::new();
    let ast = parser.parse(src).unwrap();
    let (tast, env) = compiler::typer::check_file(ast);
    let core = compiler::compile_match::compile(&env, &tast);
    let mut buffer = String::new();
    let _result = compiler::interpreter::eval(&im::HashMap::new(), &mut buffer, &core);
    let core_str = core.to_pretty(&env, 120);
    return core_str;
}
