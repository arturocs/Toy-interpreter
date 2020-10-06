use std::{env, fs};
use expr_eval::evaluator::Environment;
use interpreter::{parser::parse, runtime, tokenizer::tokenize};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut env = Environment::new();
    let filename = env::args().nth(1).ok_or("Missing argument")?;
    let contents = fs::read_to_string(filename)?;
    let instructions = tokenize(&contents);
    //dbg!(&instructions);
    let ast = parse(&instructions)?;
    // dbg!(&ast);
    runtime::execute(&ast, &mut env)?;
    Ok(())
}
