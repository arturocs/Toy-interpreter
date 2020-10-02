use crate::val::Val;
use crate::exprtoken_processor::ProcessedExprToken;
type Error = &'static str;

#[derive(PartialEq, Debug, Clone)]
pub enum ParseExprNode {
    VarName(String),
    Number(Val),
    String(Val),
    Bool(Val),
    Null,
    //FnCallStart(&'a str),
    VecAccess(String, Box<ParseExprNode>),
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
            .map(|t| parse_expr(t))
            .collect::<Result<Vec<_>, _>>()?,
    ))
}

fn neg_to_node<'a>(a: &'a Box<ProcessedExprToken>) -> Result<ParseExprNode, Error> {
    let token_slice = std::slice::from_ref(a.as_ref());
    let node = parse_add(token_slice)?;
    Ok(ParseExprNode::Neg(Box::new(node)))
}


fn parse_mul<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Mul)
        .map(|x| match &x[0] {
            ProcessedExprToken::Vector(v) => parse_vector(v),
            ProcessedExprToken::Null => Ok(ParseExprNode::Null),
            ProcessedExprToken::Bool(a) => Ok(ParseExprNode::Bool(Val::Bool(*a))),
            ProcessedExprToken::String(a) => Ok(ParseExprNode::String(Val::Str(a.clone()))),
            ProcessedExprToken::Neg(a) => neg_to_node(a),
            ProcessedExprToken::Number(a) => Ok(ParseExprNode::Number(Val::Number(*a))),
            ProcessedExprToken::Parentheses(a) => parse_expr(a),
            ProcessedExprToken::VarName(a) => Ok(ParseExprNode::VarName(a.clone())),
            _ => Err("Error parsing final element"),
        })
        .fold_first(|a, b| Ok(ParseExprNode::Mul(Box::new([a?, b?]))))
        .ok_or("Error parsing multiplication")?
}

fn parse_div<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Div)
        .map(|x| parse_mul(x))
        .fold_first(|a, b| Ok(ParseExprNode::Div(Box::new([a?, b?]))))
        .ok_or("Error parsing division")?
}

fn parse_rem<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Rem)
        .map(|x| parse_div(x))
        .fold_first(|a, b| Ok(ParseExprNode::Rem(Box::new([a?, b?]))))
        .ok_or("Error parsing division")?
}

fn parse_sub<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Sub)
        .map(|x| parse_rem(x))
        .fold_first(|a, b| Ok(ParseExprNode::Sub(Box::new([a?, b?]))))
        .ok_or("Error parsing subtraction")?
}

fn parse_add<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Add)
        .map(|x| parse_sub(x))
        .fold_first(|a, b| Ok(ParseExprNode::Add(Box::new([a?, b?]))))
        .ok_or("Error parsing subtraction")?
}

fn parse_ltoe<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Ltoe)
        .map(|x| parse_add(x))
        .fold_first(|a, b| Ok(ParseExprNode::Ltoe(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_lt<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Lt)
        .map(|x| parse_ltoe(x))
        .fold_first(|a, b| Ok(ParseExprNode::Lt(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_gtoe<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Gtoe)
        .map(|x| parse_lt(x))
        .fold_first(|a, b| Ok(ParseExprNode::Gtoe(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_gt<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Gt)
        .map(|x| parse_gtoe(x))
        .fold_first(|a, b| Ok(ParseExprNode::Gt(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_noteq<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::NotEq)
        .map(|x| parse_gt(x))
        .fold_first(|a, b| Ok(ParseExprNode::NotEq(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_eq<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Eq)
        .map(|x| parse_noteq(x))
        .fold_first(|a, b| Ok(ParseExprNode::Eq(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_or<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::Or)
        .map(|x| parse_eq(x))
        .fold_first(|a, b| Ok(ParseExprNode::Or(Box::new([a?, b?]))))
        .ok_or("Error parsing logical or")?
}

fn parse_and<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    tokens
        .split(|x| *x == ProcessedExprToken::And)
        .map(|x| parse_or(x))
        .fold_first(|a, b| Ok(ParseExprNode::And(Box::new([a?, b?]))))
        .ok_or("Error parsing logical and")?
}

pub fn parse_expr<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    parse_and(tokens)
}
