pub mod parser;
pub mod runtime;
pub mod tokenizer;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use expr_eval::{evaluator::Environment, val::Val};

    use crate::{parser::parse, runtime, tokenizer::tokenize};

    #[test]
    fn while_loop() {
        let mut env = Environment::new();
        let code = "a=0
        while a <10 {
            a = a + 1
        }";
        let instructions = tokenize(&code);
        let ast = parse(&instructions).unwrap();
        runtime::execute(&ast, &mut env).unwrap();
        assert_eq!(env.get("a"), Ok(Val::Number(10.0)));
    }
}