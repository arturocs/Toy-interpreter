#[macro_use]
extern crate lazy_static;
pub mod evaluator;
pub mod exprtoken_processor;
pub mod parser;
pub mod tokenizer;
pub mod val;

#[cfg(test)]
mod tests {
    use crate::{evaluator::Environment, parser::parse_expr, tokenizer::tokenize_expr, val::Val};

    #[test]
    fn four_divided_by_2_plus_2() {
        let tokens = tokenize_expr("4/2+2").unwrap();
        let ast = parse_expr(&tokens).unwrap();
        //dbg!(&ast);
        let mut env = Environment::new();
        let result = env.evaluate(&ast).unwrap();
        assert_eq!(Val::Number(4.0 / 2.0 + 2.0), result);
    }

    #[test]
    fn true_() {
        let tokens = tokenize_expr("true").unwrap();
        //dbg!(&tokens);
        let ast = parse_expr(&tokens).unwrap();
        //dbg!(&ast);
        let mut env = Environment::new();
        let result = env.evaluate(&ast).unwrap();
        assert_eq!(Val::Bool(true), result);
    }

    #[test]
    fn four_equals_2() {
        let tokens = tokenize_expr("4==2").unwrap();
        let ast = parse_expr(&tokens).unwrap();
        // dbg!(&ast);
        let mut env = Environment::new();
        let result = env.evaluate(&ast).unwrap();
        assert_eq!(Val::Bool(4 == 2), result);
    }
    #[test]
    fn four_plus_1_gtoe_5_and_2_lt_3() {
        let tokens = tokenize_expr("4+1 >= 5 &&  2<3").unwrap();
        let ast = parse_expr(&tokens).unwrap();
        // dbg!(&ast);
        let mut env = Environment::new();
        let result = env.evaluate(&ast).unwrap();
        assert_eq!(Val::Bool(4 + 1 >= 5 && 2 < 3), result);
    }

    #[test]
    fn two_x_3_plus_4_x_5() {
        let tokens = tokenize_expr("2*3+4*5").unwrap();
        let ast = parse_expr(&tokens).unwrap();
        //dbg!(&ast);
        let mut env = Environment::new();
        let result = env.evaluate(&ast).unwrap();
        assert_eq!(Val::Number(2.0 * 3.0 + 4.0 * 5.0), result);
    }

    #[test]
    fn negative_number() {
        let tokens = tokenize_expr("-2+1").unwrap();
        let ast = parse_expr(&tokens).unwrap();
        //dbg!(&ast);
        let mut env = Environment::new();
        let result = env.evaluate(&ast).unwrap();
        assert_eq!(Val::Number(-2.0 + 1.0), result);
    }

    #[test]
    fn subtract_number() {
        let tokens = tokenize_expr("2-1").unwrap();
        let ast = parse_expr(&tokens).unwrap();
        //dbg!(&ast);
        let mut env = Environment::new();
        let result = env.evaluate(&ast).unwrap();
        assert_eq!(Val::Number(2.0 - 1.0), result);
    }

    #[test]
    fn two_x_3_plus_4_x_5_parentheses() {
        let tokens = tokenize_expr("(2*3)+(4*5)").unwrap();
        let ast = parse_expr(&tokens).unwrap();
        //dbg!(&ast);
        let mut env = Environment::new();
        let result = env.evaluate(&ast).unwrap();
        assert_eq!(Val::Number((2.0 * 3.0) + (4.0 * 5.0)), result);
    }

    #[test]
    #[allow(unused_parens)]
    fn lot_of_parentheses() {
        let tokens = tokenize_expr("((1+2)*3/(5*(3+1)))").unwrap();
        let ast = parse_expr(&tokens).unwrap();
        // dbg!(&ast);
        let mut env = Environment::new();
        let result = env.evaluate(&ast).unwrap();
        assert_eq!(
            Val::Number(((1.0 + 2.0) * 3.0 / (5.0 * (3.0 + 1.0)))),
            result
        );
    }
    #[test]
    fn zero_x_1_plus_2_x_3_x_4_plus_5_plus_6() {
        let tokens = tokenize_expr("0*1+2*3*4+5+6").unwrap();
        let ast = parse_expr(&tokens).unwrap();
        let mut env = Environment::new();
        let result = env.evaluate(&ast).unwrap();
        assert_eq!(Val::Number(0.0 * 1.0 + 2.0 * 3.0 * 4.0 + 5.0 + 6.0), result);
    }

    #[test]
    fn three_plus_4_divided_by_5() {
        let tokens = tokenize_expr("3+4/5").unwrap();
        let ast = parse_expr(&tokens).unwrap();
        //dbg!(&ast);
        let mut env = Environment::new();
        let result = env.evaluate(&ast).unwrap();
        assert_eq!(Val::Number(3.0 + 4.0 / 5.0), result);
    }

    #[test]
    fn threee_plus_4_divided_by_5_parentheses() {
        let tokens = tokenize_expr("(3+4)/5").unwrap();
        let ast = parse_expr(&tokens).unwrap();
        //dbg!(&ast);
        let mut env = Environment::new();
        let result = env.evaluate(&ast).unwrap();
        assert_eq!(Val::Number((3.0 + 4.0) / 5.0), result);
    }
}
