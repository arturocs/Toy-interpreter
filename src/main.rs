use std::{env, fs};
use v_eval::Eval;
mod parser;
mod runtime;
mod tokenizer;
mod val;
#[macro_use]
extern crate lazy_static;
use crate::parser::parse;
use crate::tokenizer::tokenize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Eval::default();
    let filename = env::args().nth(1).ok_or("Missing argument")?;
    let contents = fs::read_to_string(filename)?;
    let instructions = tokenize(&contents);
    //let instructions = dbg!(instructions);
    let ast = parse(&instructions)?;
    //dbg!(&ast);
    runtime::execute(&ast, env)?;
    Ok(())
}
