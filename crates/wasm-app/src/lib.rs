use cst::cst::CstNode;
use parser::syntax::MySyntaxNode;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn execute(src: &str) -> String {
    let result = parser::parse(&std::path::PathBuf::from("dummy"), src);
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
    let result = parser::parse(&std::path::PathBuf::from("dummy"), src);
    let root = MySyntaxNode::new_root(result.green_node);
    let cst = cst::cst::File::cast(root).unwrap();
    let ast = ast::lower::lower(cst).unwrap();

    let (tast, env) = compiler::typer::check_file(ast);
    let core = compiler::compile_match::compile_file(&env, &tast);

    core.to_pretty(&env, 120)
}

#[wasm_bindgen]
pub fn compile_to_anf(src: &str) -> String {
    let result = parser::parse(&std::path::PathBuf::from("dummy"), src);
    let root = MySyntaxNode::new_root(result.green_node);
    let cst = cst::cst::File::cast(root).unwrap();
    let ast = ast::lower::lower(cst).unwrap();

    let (tast, env) = compiler::typer::check_file(ast);
    let core = compiler::compile_match::compile_file(&env, &tast);
    let anf = compiler::anf::anf_file(&env, core);

    anf.to_pretty(&env, 120)
}

#[wasm_bindgen]
pub fn get_cst(src: &str) -> String {
    let result = parser::parse(&std::path::PathBuf::from("dummy"), src);
    parser::debug_tree(&result.green_node)
}

#[wasm_bindgen]
pub fn get_ast(src: &str) -> String {
    let result = parser::parse(&std::path::PathBuf::from("dummy"), src);
    let root = MySyntaxNode::new_root(result.green_node);
    let cst = cst::cst::File::cast(root).unwrap();
    let ast = ast::lower::lower(cst).unwrap();
    format!("{:#?}", ast)
}

#[wasm_bindgen]
pub fn get_tast(src: &str) -> String {
    let result = parser::parse(&std::path::PathBuf::from("dummy"), src);
    let root = MySyntaxNode::new_root(result.green_node);
    let cst = cst::cst::File::cast(root).unwrap();
    let ast = ast::lower::lower(cst).unwrap();

    let (tast, env) = compiler::typer::check_file(ast);
    tast.to_pretty(&env, 120)
}

#[wasm_bindgen]
pub fn hover(src: &str, line: u32, col: u32) -> Option<String> {
    compiler::query::hover_type(src, line, col)
}
