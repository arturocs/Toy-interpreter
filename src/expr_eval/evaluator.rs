use crate::expr_eval::parser::*;
use std::collections::BTreeMap;

#[allow(unused_imports)]
use super::tokenizer::tokenize;
use crate::expr_eval::val::Val;

type Error = &'static str;

pub(crate) struct Environment {
    variables: BTreeMap<String, Val>,
    //functions : std::collections::HashMap<&str, _>
}

impl Environment {
    pub(crate) fn new() -> Environment {
        Environment {
            variables: BTreeMap::new(),
        }
    }
    /*pub(crate) fn with_capacity(capacity: usize) -> Environment {
        Environment {
            variables: BTreeMap::with_capacity(capacity),
        }
    }*/
    pub(crate) fn insert(&mut self, variable: String, value: Val) {
        &self.variables.insert(variable, value);
    }
    pub(crate) fn execute<'a>(&self, node: &ParseNode) -> Result<Val, Error> {
        match node {
            ParseNode::VarName(a) => {

                Ok(self.variables.get(a).ok_or("Undeclared variable")?.clone())
            }
            ParseNode::Number(n) => Ok(n.clone()),
            ParseNode::String(s) => Ok(s.clone()),
            ParseNode::Bool(b) => Ok(b.clone()),
            ParseNode::Neg(n) => Ok(self.execute(&n)?.minus()?),
            ParseNode::Mul(s) => self.execute(&s[0])?.mul(self.execute(&s[1])?),
            ParseNode::Div(s) => self.execute(&s[0])?.div(self.execute(&s[1])?),
            ParseNode::Rem(s) => self.execute(&s[0])?.rem(self.execute(&s[1])?),
            ParseNode::Add(s) => self.execute(&s[0])?.add(self.execute(&s[1])?),
            ParseNode::Sub(s) => self.execute(&s[0])?.sub(self.execute(&s[1])?),
            ParseNode::Eq(s) => Ok(Val::Bool(self.execute(&s[0])?.eq(&self.execute(&s[1])?))),
            ParseNode::NotEq(s) => Ok(Val::Bool(self.execute(&s[0])?.ne(&self.execute(&s[1])?))),
            ParseNode::And(s) => self.execute(&s[0])?.and(self.execute(&s[1])?),
            ParseNode::Or(s) => self.execute(&s[0])?.or(self.execute(&s[1])?),
            ParseNode::Not(b) => Ok(self.execute(&b)?.not()?),
            ParseNode::Gt(s) => Ok(Val::Bool(self.execute(&s[0])? > self.execute(&s[1])?)),
            ParseNode::Lt(s) => Ok(Val::Bool(self.execute(&s[0])? < self.execute(&s[1])?)),
            ParseNode::Gtoe(s) => Ok(Val::Bool(self.execute(&s[0])? >= self.execute(&s[1])?)),
            ParseNode::Ltoe(s) => Ok(Val::Bool(self.execute(&s[0])? <= self.execute(&s[1])?)),
        }
    }
}

#[test]
fn four_divided_by_2_plus_2() {
    let tokens = tokenize("4/2+2").unwrap();
    let processed_tokens = process_tokens(&tokens).unwrap();
    let ast = parse(&processed_tokens).unwrap();
    //dbg!(&ast);
    let env = Environment::new();
    let result = env.execute(&ast).unwrap();
    assert_eq!(Val::Number(4.0 / 2.0 + 2.0), result);
}

#[test]
fn true_() {
    let tokens = tokenize("true").unwrap();
    //dbg!(&tokens);
    let processed_tokens = process_tokens(&tokens).unwrap();
    let ast = parse(&processed_tokens).unwrap();
    //dbg!(&ast);
    let env = Environment::new();
    let result = env.execute(&ast).unwrap();
    assert_eq!(Val::Bool(true), result);
}

#[test]
fn four_equals_2() {
    let tokens = tokenize("4==2").unwrap();
    let processed_tokens = process_tokens(&tokens).unwrap();
    let ast = parse(&processed_tokens).unwrap();
    // dbg!(&ast);
    let env = Environment::new();
    let result = env.execute(&ast).unwrap();
    assert_eq!(Val::Bool(4 == 2), result);
}
#[test]
fn four_plus_1_gtoe_5_and_2_lt_3() {
    let tokens = tokenize("4+1 >= 5 &&  2<3").unwrap();
    let processed_tokens = process_tokens(&tokens).unwrap();
    let ast = parse(&processed_tokens).unwrap();
    // dbg!(&ast);
    let env = Environment::new();
    let result = env.execute(&ast).unwrap();
    assert_eq!(Val::Bool(4 + 1 >= 5 && 2 < 3), result);
}

#[test]
fn two_x_3_plus_4_x_5() {
    let tokens = tokenize("2*3+4*5").unwrap();
    let processed_tokens = process_tokens(&tokens).unwrap();
    let ast = parse(&processed_tokens).unwrap();
    //dbg!(&ast);
    let env = Environment::new();
    let result = env.execute(&ast).unwrap();
    assert_eq!(Val::Number(2.0 * 3.0 + 4.0 * 5.0), result);
}

#[test]
fn negative_number() {
    let tokens = tokenize("-2+1").unwrap();
    let processed_tokens = process_tokens(&tokens).unwrap();
    let ast = parse(&processed_tokens).unwrap();
    //dbg!(&ast);
    let env = Environment::new();
    let result = env.execute(&ast).unwrap();
    assert_eq!(Val::Number(-2.0 + 1.0), result);
}

#[test]
fn subtract_number() {
    let tokens = tokenize("2-1").unwrap();
    let processed_tokens = process_tokens(&tokens).unwrap();
    let ast = parse(&processed_tokens).unwrap();
    //dbg!(&ast);
    let env = Environment::new();
    let result = env.execute(&ast).unwrap();
    assert_eq!(Val::Number(2.0 - 1.0), result);
}

#[test]
fn two_x_3_plus_4_x_5_parentheses() {
    let tokens = tokenize("(2*3)+(4*5)").unwrap();
    let processed_tokens = process_tokens(&tokens).unwrap();
    let ast = parse(&processed_tokens).unwrap();
    //dbg!(&ast);
    let env = Environment::new();
    let result = env.execute(&ast).unwrap();
    assert_eq!(Val::Number((2.0 * 3.0) + (4.0 * 5.0)), result);
}

#[test]
#[allow(unused_parens)]
fn lot_of_parentheses() {
    let tokens = tokenize("((1+2)*3/(5*(3+1)))").unwrap();
    let processed_tokens = process_tokens(&tokens).unwrap();
    //dbg!(&processed_tokens);
    let ast = parse(&processed_tokens).unwrap();
    // dbg!(&ast);
    let env = Environment::new();
    let result = env.execute(&ast).unwrap();
    assert_eq!(
        Val::Number(((1.0 + 2.0) * 3.0 / (5.0 * (3.0 + 1.0)))),
        result
    );
}
#[test]
fn zero_x_1_plus_2_x_3_x_4_plus_5_plus_6() {
    let tokens = tokenize("0*1+2*3*4+5+6").unwrap();
    let processed_tokens = process_tokens(&tokens).unwrap();
    let ast = parse(&processed_tokens).unwrap();
    let env = Environment::new();
    let result = env.execute(&ast).unwrap();
    assert_eq!(Val::Number(0.0 * 1.0 + 2.0 * 3.0 * 4.0 + 5.0 + 6.0), result);
}

#[test]
fn three_plus_4_divided_by_5() {
    let tokens = tokenize("3+4/5").unwrap();
    let processed_tokens = process_tokens(&tokens).unwrap();
    //dbg!(&processed_tokens);
    let ast = parse(&processed_tokens).unwrap();
    //dbg!(&ast);
    let env = Environment::new();
    let result = env.execute(&ast).unwrap();
    assert_eq!(Val::Number(3.0 + 4.0 / 5.0), result);
}

#[test]
fn threee_plus_4_divided_by_5_parentheses() {
    let tokens = tokenize("(3+4)/5").unwrap();
    let processed_tokens = process_tokens(&tokens).unwrap();
    dbg!(&processed_tokens);
    let ast = parse(&processed_tokens).unwrap();
    //dbg!(&ast);
    let env = Environment::new();
    let result = env.execute(&ast).unwrap();
    assert_eq!(Val::Number((3.0 + 4.0) / 5.0), result);
}
