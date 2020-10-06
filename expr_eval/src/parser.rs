use crate::val::Val;
use crate::{
    exprtoken_processor::{process_expr_tokens, ProcessedExprToken},
    tokenizer::ExprToken,
};
type Error = &'static str;

#[derive(PartialEq, Debug, Clone)]
pub enum ParseExprNode {
    VarName(String),
    Number(Val),
    String(Val),
    Bool(Val),
    Null,
    //FnCallStart(&'a str),
    VecAccess(String, Vec<ParseExprNode>),
    //Dot
    Vector(Vec<ParseExprNode>),
    Neg(Box<ParseExprNode>),
    Mul(Box<[ParseExprNode; 2]>),
    Div(Box<[ParseExprNode; 2]>),
    Rem(Box<[ParseExprNode; 2]>),
    Add(Box<[ParseExprNode; 2]>),
    Sub(Box<[ParseExprNode; 2]>),
    Eq(Box<[ParseExprNode; 2]>),
    NotEq(Box<[ParseExprNode; 2]>),
    Gt(Box<[ParseExprNode; 2]>),
    Lt(Box<[ParseExprNode; 2]>),
    Gtoe(Box<[ParseExprNode; 2]>),
    Ltoe(Box<[ParseExprNode; 2]>),
    And(Box<[ParseExprNode; 2]>),
    Or(Box<[ParseExprNode; 2]>),
    Not(Box<ParseExprNode>),
}
fn parse_vector(vector: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    Ok(ParseExprNode::Vector(
        vector
            .split(|x| *x == ProcessedExprToken::Comma)
            .map(|t| parse_and(t))
            .collect::<Result<Vec<_>, _>>()?,
    ))
}

fn parse_vector_read(
    name: &str,
    index_expr: &[Vec<ProcessedExprToken>],
) -> Result<ParseExprNode, Error> {
    let indexes: Result<Vec<_>, _> = index_expr.iter().map(|e| parse_and(e)).collect();
    Ok(ParseExprNode::VecAccess(name.to_string(), indexes?))
}

fn neg_to_node(a: &ProcessedExprToken) -> Result<ParseExprNode, Error> {
    let token_slice = std::slice::from_ref(a);
    let node = parse_add(token_slice)?;
    Ok(ParseExprNode::Neg(Box::new(node)))
}

fn parse_final_element(final_element: &ProcessedExprToken) -> Result<ParseExprNode, Error> {
    match final_element {
        ProcessedExprToken::VecAccess(name, index_expr) => parse_vector_read(name, index_expr),
        ProcessedExprToken::Vector(v) => parse_vector(v),
        ProcessedExprToken::Null => Ok(ParseExprNode::Null),
        ProcessedExprToken::Bool(a) => Ok(ParseExprNode::Bool(Val::Bool(*a))),
        ProcessedExprToken::String(a) => Ok(ParseExprNode::String(Val::Str(a.clone()))),
        ProcessedExprToken::Neg(a) => neg_to_node(a),
        ProcessedExprToken::Number(a) => Ok(ParseExprNode::Number(Val::Number(*a))),
        ProcessedExprToken::Parentheses(a) => parse_and(a),
        ProcessedExprToken::VarName(a) => Ok(ParseExprNode::VarName(a.clone())),
        _ => Err("Error parsing final element"),
    }
}

fn parse_mul(tokens: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Mul)
        .map(|x| parse_final_element(&x[0]))
        .fold_first(|a, b| Ok(ParseExprNode::Mul(Box::new([a?, b?]))))
        .ok_or("Error parsing multiplication")?
}

fn parse_div(tokens: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Div)
        .map(|x| parse_mul(x))
        .fold_first(|a, b| Ok(ParseExprNode::Div(Box::new([a?, b?]))))
        .ok_or("Error parsing division")?
}

fn parse_rem(tokens: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Rem)
        .map(|x| parse_div(x))
        .fold_first(|a, b| Ok(ParseExprNode::Rem(Box::new([a?, b?]))))
        .ok_or("Error parsing division")?
}

fn parse_sub(tokens: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Sub)
        .map(|x| parse_rem(x))
        .fold_first(|a, b| Ok(ParseExprNode::Sub(Box::new([a?, b?]))))
        .ok_or("Error parsing subtraction")?
}

fn parse_add(tokens: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Add)
        .map(|x| parse_sub(x))
        .fold_first(|a, b| Ok(ParseExprNode::Add(Box::new([a?, b?]))))
        .ok_or("Error parsing subtraction")?
}

fn parse_ltoe(tokens: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Ltoe)
        .map(|x| parse_add(x))
        .fold_first(|a, b| Ok(ParseExprNode::Ltoe(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_lt(tokens: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Lt)
        .map(|x| parse_ltoe(x))
        .fold_first(|a, b| Ok(ParseExprNode::Lt(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_gtoe(tokens: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Gtoe)
        .map(|x| parse_lt(x))
        .fold_first(|a, b| Ok(ParseExprNode::Gtoe(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_gt(tokens: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Gt)
        .map(|x| parse_gtoe(x))
        .fold_first(|a, b| Ok(ParseExprNode::Gt(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_noteq(tokens: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::NotEq)
        .map(|x| parse_gt(x))
        .fold_first(|a, b| Ok(ParseExprNode::NotEq(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_eq(tokens: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Eq)
        .map(|x| parse_noteq(x))
        .fold_first(|a, b| Ok(ParseExprNode::Eq(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_or(tokens: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Or)
        .map(|x| parse_eq(x))
        .fold_first(|a, b| Ok(ParseExprNode::Or(Box::new([a?, b?]))))
        .ok_or("Error parsing logical or")?
}

fn parse_and(tokens: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::And)
        .map(|x| parse_or(x))
        .fold_first(|a, b| Ok(ParseExprNode::And(Box::new([a?, b?]))))
        .ok_or("Error parsing logical and")?
}

pub fn parse_expr(tokens: &[ExprToken]) -> Result<ParseExprNode, Error> {
    let processed_tokens = process_expr_tokens(tokens)?;
    // dbg!(&processed_tokens);
    parse_and(&processed_tokens)
}
