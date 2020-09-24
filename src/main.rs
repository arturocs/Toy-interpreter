#![allow(dead_code)]
use std::{env, fs};
use v_eval::Eval;
mod parser;
mod runtime;
mod tokenizer;
mod val;
#[macro_use]
extern crate lazy_static;
use crate::parser::*;
use crate::tokenizer::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let t = tokenize_expression("(4*5*(6*7))")?;
    dbg!(&t);
    let a = parse_expr(&t)?;
    dbg!(&a);
    let env = Eval::default();
    let filename = env::args().nth(1).ok_or("Missing argument")?;
    let contents = fs::read_to_string(filename)?;
    let instructions = tokenize(&contents);
    //let instructions = dbg!(instructions);
    let ast = parse(&instructions)?;
    //dbg!(&ast);
    //runtime::execute(&ast, env)?;
    Ok(())
}
