use std::{env, fs};
use v_eval::Eval;
mod tokenizer;
mod val;
mod parser;
mod runtime;
#[macro_use]
extern crate lazy_static;
use crate::tokenizer::*;
use crate::parser::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Eval::default();
    let filename = env::args().nth(1).ok_or("Missing argument")?;
    let contents = fs::read_to_string(filename)?;
    let instructions = tokenizer(&contents);
    //let instructions = dbg!(instructions);
    let ast = parse(&instructions)?;
    //dbg!(&ast);
    runtime::execute(&ast, env)?;
    Ok(())
}
