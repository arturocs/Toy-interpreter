use crate::expr_eval::parser::*;

#[allow(unused_imports)]
use super::tokenizer::tokenize;
use crate::expr_eval::val::Val;

type Error = &'static str;

pub(crate) fn evaluate<'a>(node: &ParseNode<'a>) -> Result<Val, Error> {
    match node {
        ParseNode::VarName(_) => todo!("Variables not implemented"),
        ParseNode::Number(n) => Ok(n.clone()),
        ParseNode::String(s) => Ok(s.clone()),
        ParseNode::Bool(b) => Ok(b.clone()),
        ParseNode::Neg(n) => Ok(evaluate(&n)?.minus()?),
        ParseNode::Mul(s) => evaluate(&s[0])?.mul(evaluate(&s[1])?),
        ParseNode::Div(s) => evaluate(&s[0])?.div(evaluate(&s[1])?),
        ParseNode::Rem(s) => evaluate(&s[0])?.rem(evaluate(&s[1])?),
        ParseNode::Add(s) => evaluate(&s[0])?.add(evaluate(&s[1])?),
        ParseNode::Sub(s) => evaluate(&s[0])?.sub(evaluate(&s[1])?),
        ParseNode::Eq(s) => Ok(Val::Bool(evaluate(&s[0])?.eq(&evaluate(&s[1])?))),
        ParseNode::NotEq(s) => Ok(Val::Bool(evaluate(&s[0])?.ne(&evaluate(&s[1])?))),
        ParseNode::And(s) => evaluate(&s[0])?.and(evaluate(&s[1])?),
        ParseNode::Or(s) => evaluate(&s[0])?.or(evaluate(&s[1])?),
        ParseNode::Not(b) => Ok(evaluate(&b)?.not()?),
        ParseNode::Gt(s) => Ok(Val::Bool(evaluate(&s[0])? > evaluate(&s[1])?)),
        ParseNode::Lt(s) => Ok(Val::Bool(evaluate(&s[0])? < evaluate(&s[1])?)),
        ParseNode::Gtoe(s) => Ok(Val::Bool(evaluate(&s[0])? >= evaluate(&s[1])?)),
        ParseNode::Ltoe(s) => Ok(Val::Bool(evaluate(&s[0])? <= evaluate(&s[1])?)),
    }
}
#[test]
fn two_x_3_plus_4_x_5() {
    let tokens = tokenize("2*3+4*5").unwrap();
    let ast = parse(&mut (tokens.len() - 1), &tokens).unwrap();
    dbg!(&ast);
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
