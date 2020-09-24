#![allow(dead_code)]
use std::{env, fs};
use v_eval::Eval;
mod expr_eval;
mod parser;
mod runtime;
mod tokenizer;
#[macro_use]
extern crate lazy_static;
use crate::parser::*;
use crate::tokenizer::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Eval::default();
    let filename = env::args().nth(1).ok_or("Missing argument")?;
    let contents = fs::read_to_string(filename)?;
    let instructions = tokenize(&contents);
    let instructions = dbg!(instructions);
    let ast = parse(&instructions)?;
    runtime::execute(&ast, env)?;
    Ok(())
}
