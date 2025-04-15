use cst::cst::CstNode;
use parser::syntax::MySyntaxNode;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn execute(src: &str) -> String {
    let result = parser::parse(&std::path::PathBuf::from("dummy"), &src);
    let root = MySyntaxNode::new_root(result.green_node);
    let cst = cst::cst::File::cast(root).unwrap();
    let ast = ast::lower::lower(cst).unwrap();

    let (tast, env) = compiler::typer::check_file(ast);
    let core = compiler::compile_match::compile_file(&env, &tast);
    let mut buffer = String::new();
    let _result = compiler::interpreter::eval_file(&im::HashMap::new(), &mut buffer, &core);
    buffer
}

#[wasm_bindgen]
pub fn compile_to_core(src: &str) -> String {
    let result = parser::parse(&std::path::PathBuf::from("dummy"), &src);
    let root = MySyntaxNode::new_root(result.green_node);
    let cst = cst::cst::File::cast(root).unwrap();
    let ast = ast::lower::lower(cst).unwrap();

    let (tast, env) = compiler::typer::check_file(ast);
    let core = compiler::compile_match::compile_file(&env, &tast);

    core.to_pretty(&env, 120)
}
