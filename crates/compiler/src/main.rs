use cst::cst::CstNode;
use parser::syntax::MySyntaxNode;

fn main() {
    let file_path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: {} <file_path>", std::env::args().next().unwrap());
        std::process::exit(1);
    });
    let content = std::fs::read_to_string(&file_path).unwrap_or_else(|_| {
        eprintln!("Error reading file: {}", file_path);
        std::process::exit(1);
    });
    let src = content;
    let result = parser::parse(&std::path::PathBuf::from(file_path), &src);
    let root = MySyntaxNode::new_root(result.green_node);
    let cst = cst::cst::File::cast(root).unwrap();
    let ast = ast::lower::lower(cst).unwrap();

    let (tast, env) = compiler::typer::check_file(ast);
    let core = compiler::compile_match::compile_file(&env, &tast);
    let mut buffer = String::new();
    let result = compiler::interpreter::eval_file(&im::HashMap::new(), &mut buffer, &core);
    println!("stdout: {}", buffer);
    println!("return: {:?}", result);
}
