use crate::expr_eval::parser::*;

#[allow(unused_imports)]
use super::tokenizer::tokenize;
use crate::expr_eval::val::Val;

type Error = &'static str;

pub(crate) fn evaluate<'a>(node: &ParseNode<'a>) -> Result<Val, Error> {
    match node {
        ParseNode::VarName(_) => todo!(),
        ParseNode::Number(n) => Ok(n.clone()),
        ParseNode::String(_) => todo!(),
        ParseNode::Bool(b) => Ok(b.clone()),
        ParseNode::Neg(_) => todo!(),
        ParseNode::Mul(m) => evaluate(&m[0])?.mul(evaluate(&m[1])?),
        ParseNode::Div(d) => evaluate(&d[0])?.div(evaluate(&d[1])?),
        ParseNode::Rem(r) => evaluate(&r[0])?.rem(evaluate(&r[1])?),
        ParseNode::Add(a) => evaluate(&a[0])?.add(evaluate(&a[1])?),
        ParseNode::Sub(_) => todo!(),
        ParseNode::Eq(_) => todo!(),
        ParseNode::NotEq(_) => todo!(),
        ParseNode::And(_) => todo!(),
        ParseNode::Or(_) => todo!(),
        ParseNode::Not(_) => todo!(),
    }
}
#[test]
fn two_x_3_plus_4_x_5() {
    let tokens = tokenize("2*3+4*5").unwrap();
    let ast = parse(&mut (tokens.len() - 1), &tokens).unwrap();
    let result = evaluate(&ast).unwrap();
    assert_eq!(Val::Number(26.0), result);
}

#[test]
fn two_x_3_plus_4_x_5_parentheses() {
    let tokens = tokenize("(2*3)+(4*5)").unwrap();
    let ast = parse(&mut (tokens.len() - 1), &tokens).unwrap();
    let result = evaluate(&ast).unwrap();
    assert_eq!(Val::Number(26.0), result);
}

#[test]
fn zero_x_1_plus_2_x_3_x_4_plus_5_plus_6() {
    let tokens = tokenize("0*1+2*3*4+5+6").unwrap();
    let ast = parse(&mut (tokens.len() - 1), &tokens).unwrap();
    let result = evaluate(&ast).unwrap();
    assert_eq!(Val::Number(35.0), result);
}

#[test]
fn three_plus_4_divided_by_5() {
    let tokens = tokenize("3+4/5").unwrap();
    let ast = parse(&mut (tokens.len() - 1), &tokens).unwrap();
    let result = evaluate(&ast).unwrap();
    assert_eq!(Val::Number(3.8), result);
}

#[test]
fn threee_plus_4_divided_by_5_parentheses() {
    let tokens = tokenize("(3+4)/5").unwrap();
    let ast = parse(&mut (tokens.len() - 1), &tokens).unwrap();
    let result = evaluate(&ast).unwrap();
    assert_eq!(Val::Number(1.4), result);
}
