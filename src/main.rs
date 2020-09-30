#![allow(dead_code)]
#![feature(iterator_fold_self)]
use std::{env, fs};
mod expr_eval;
mod parser;
mod runtime;
mod tokenizer;
#[macro_use]
extern crate lazy_static;
use evaluator::Environment;

use crate::expr_eval::*;
use crate::parser::*;
use crate::tokenizer::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::new(); // Eval::default();
    let filename = env::args().nth(1).ok_or("Missing argument")?;
    let contents = fs::read_to_string(filename)?;
    let instructions = tokenize(&contents);
    let ast = parse(&instructions)?;
    runtime::execute(&ast, env)?;
    Ok(())
}
