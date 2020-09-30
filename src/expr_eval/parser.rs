use crate::expr_eval::tokenizer::Token;
use crate::expr_eval::val::Val;

type Error = &'static str;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum ParseNode {
    VarName(String),
    Number(Val),
    String(Val),
    Bool(Val),
    //FnCallStart(&'a str),
    //VecAccessStart(&'a str),
    //Dot
    Neg(Box<ParseNode>),
    Mul(Box<[ParseNode; 2]>),
    Div(Box<[ParseNode; 2]>),
    Rem(Box<[ParseNode; 2]>),
    Add(Box<[ParseNode; 2]>),
    Sub(Box<[ParseNode; 2]>),
    Eq(Box<[ParseNode; 2]>),
    NotEq(Box<[ParseNode; 2]>),
    Gt(Box<[ParseNode; 2]>),
    Lt(Box<[ParseNode; 2]>),
    Gtoe(Box<[ParseNode; 2]>),
    Ltoe(Box<[ParseNode; 2]>),
    And(Box<[ParseNode; 2]>),
    Or(Box<[ParseNode; 2]>),
    Not(Box<ParseNode>),
}
#[derive(PartialEq, Debug, Clone)]
pub(crate) enum ProcessedToken {
    VarName(String),
    Number(f64),
    String(String),
    Bool(bool),
    //FnCall(&'a str, Vec<ProcessedToken<'a>>),
    //VecAccess(&'a str, Vec<ProcessedToken<'a>>),
    //Dot
    Parentheses(Vec<ProcessedToken>),
    Brackets(Vec<ProcessedToken>),
    OpenParentheses,
    CloseParentheses,
    Mul,
    Div,
    Rem,
    Add,
    Sub,
    Eq,
    NotEq,
    Gt,
    Lt,
    Gtoe,
    Ltoe,
    And,
    Or,
    Not(Option<Box<ProcessedToken>>),
    Comma,
    Neg(Box<ProcessedToken>),
}

fn find_matching_parentheses<'a>(i: usize, tokens: &'a [Token]) -> Result<usize, Error> {
    let mut nested_parentheses = -1;
    let mut last_parentheses = 0;
    let mut found = false;
    for index in i + 1..tokens.len() {
        match tokens[index] {
            Token::CloseParentheses => {
                nested_parentheses += 1;
                if nested_parentheses == 0 {
                    last_parentheses = index;
                    found = true;
                    break;
                }
            }
            Token::OpenParentheses => nested_parentheses -= 1,

            _ => {}
        }
    }
    if found {
        Ok(last_parentheses)
    } else {
        Err("Unable to find matching parentheses")
    }
}

fn process_parentheses<'a>(tokens: &'a [Token], i: &mut usize) -> Result<ProcessedToken, Error> {
    let parentheses_end = find_matching_parentheses(*i, tokens)?;
    // dbg!(parentheses_end);
    let parentheses_content = &tokens[*i + 1..parentheses_end];
    *i = parentheses_end;
    Ok(ProcessedToken::Parentheses(process_tokens(
        parentheses_content,
    )?))
}

fn process_not_and_negatives<'a>(tokens: Vec<ProcessedToken>) -> Vec<ProcessedToken> {
    let mut processed_tokens = Vec::with_capacity(tokens.len());
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            ProcessedToken::Sub => {
                if i == 0 {
                    processed_tokens.push(ProcessedToken::Neg(Box::new(tokens[i + 1].clone())));
                    i += 1;
                } else {
                    match tokens[i - 1] {
                        ProcessedToken::Number(_)
                        | ProcessedToken::CloseParentheses
                        | ProcessedToken::Neg(_)
                        | ProcessedToken::VarName(_) => processed_tokens.push(ProcessedToken::Sub),
                        _ => {
                            processed_tokens
                                .push(ProcessedToken::Neg(Box::new(tokens[i + 1].clone())));
                            i += 1;
                        }
                    }
                }
            }
            ProcessedToken::Not(_) => {
                processed_tokens.push(ProcessedToken::Not(Some(Box::new(tokens[i + 1].clone()))));
                i += 1;
            }
            a => processed_tokens.push(a.clone()),
        }
        i += 1;
    }
    processed_tokens
}

pub(crate) fn process_tokens<'a>(tokens: &'a [Token]) -> Result<Vec<ProcessedToken>, Error> {
    let mut processed_tokens = Vec::with_capacity(tokens.len());
    let mut index = 0;
    let mut error = "";
    while index < tokens.len() {
        match &tokens[index] {
            Token::VarName(a) => processed_tokens.push(ProcessedToken::VarName(a.clone())),
            Token::Number(a) => processed_tokens.push(ProcessedToken::Number(*a)),
            Token::String(a) => processed_tokens.push(ProcessedToken::String(a.clone())),
            Token::Bool(a) => processed_tokens.push(ProcessedToken::Bool(*a)),
            Token::OpenParentheses => {
                processed_tokens.push(process_parentheses(tokens, &mut index)?)
            }
            Token::CloseParentheses => {
                error = "Unmatched )";
                break;
            }
            Token::Mul => processed_tokens.push(ProcessedToken::Mul),
            Token::Div => processed_tokens.push(ProcessedToken::Div),
            Token::Rem => processed_tokens.push(ProcessedToken::Rem),
            Token::Add => processed_tokens.push(ProcessedToken::Add),
            Token::Sub => processed_tokens.push(ProcessedToken::Sub),
            Token::Eq => processed_tokens.push(ProcessedToken::Eq),
            Token::NotEq => processed_tokens.push(ProcessedToken::NotEq),
            Token::Gt => processed_tokens.push(ProcessedToken::Gt),
            Token::Lt => processed_tokens.push(ProcessedToken::Lt),
            Token::Gtoe => processed_tokens.push(ProcessedToken::Gtoe),
            Token::Ltoe => processed_tokens.push(ProcessedToken::Ltoe),
            Token::And => processed_tokens.push(ProcessedToken::And),
            Token::Or => processed_tokens.push(ProcessedToken::Or),
            Token::Not => processed_tokens.push(ProcessedToken::Not(None)),
            Token::Comma => todo!("Commas"),
            // Token::FnCallStart(_) => todo!("Functions"),
            // Token::VecAccessStart(_) => todo!("Vectors"),
            // Token::OpenSBrackets => todo!("Vectors"),
            // Token::CloseSBrackets => todo!("Vectors"),
        }
        index += 1;
    }
    if error == "" {
        Ok(process_not_and_negatives(processed_tokens))
    } else {
        Err(error)
    }
}

fn neg_to_node<'a>(a: &'a Box<ProcessedToken>) -> Result<ParseNode, Error> {
    let token_slice = std::slice::from_ref(a.as_ref());
    let node = parse_add(token_slice)?;
    Ok(ParseNode::Neg(Box::new(node)))
}

fn parse_mul<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Mul)
        .map(|x| match &x[0] {
            ProcessedToken::Bool(a) => Ok(ParseNode::Bool(Val::Bool(*a))),
            ProcessedToken::String(a) => Ok(ParseNode::String(Val::Str(a.clone()))),
            ProcessedToken::Neg(a) => neg_to_node(a),
            ProcessedToken::Number(a) => Ok(ParseNode::Number(Val::Number(*a))),
            ProcessedToken::Parentheses(a) => parse_add(a),
            ProcessedToken::VarName(a) => Ok(ParseNode::VarName(a.clone())),
            a => panic!("dafuq {:?}", a),
        })
        .fold_first(|a, b| Ok(ParseNode::Mul(Box::new([a?, b?]))))
        .ok_or("Error parsing multiplication")?
}

fn parse_div<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Div)
        .map(|x| parse_mul(x))
        .fold_first(|a, b| Ok(ParseNode::Div(Box::new([a?, b?]))))
        .ok_or("Error parsing division")?
}

fn parse_rem<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Rem)
        .map(|x| parse_div(x))
        .fold_first(|a, b| Ok(ParseNode::Rem(Box::new([a?, b?]))))
        .ok_or("Error parsing division")?
}

fn parse_sub<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Sub)
        .map(|x| parse_rem(x))
        .fold_first(|a, b| Ok(ParseNode::Sub(Box::new([a?, b?]))))
        .ok_or("Error parsing subtraction")?
}

fn parse_add<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Add)
        .map(|x| parse_sub(x))
        .fold_first(|a, b| Ok(ParseNode::Add(Box::new([a?, b?]))))
        .ok_or("Error parsing subtraction")?
}

fn parse_ltoe<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Ltoe)
        .map(|x| parse_add(x))
        .fold_first(|a, b| Ok(ParseNode::Ltoe(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_lt<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Lt)
        .map(|x| parse_ltoe(x))
        .fold_first(|a, b| Ok(ParseNode::Lt(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_gtoe<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Gtoe)
        .map(|x| parse_lt(x))
        .fold_first(|a, b| Ok(ParseNode::Gtoe(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_gt<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Gt)
        .map(|x| parse_gtoe(x))
        .fold_first(|a, b| Ok(ParseNode::Gt(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_noteq<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    tokens
        .split(|x| *x == ProcessedToken::NotEq)
        .map(|x| parse_gt(x))
        .fold_first(|a, b| Ok(ParseNode::NotEq(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_eq<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Eq)
        .map(|x| parse_noteq(x))
        .fold_first(|a, b| Ok(ParseNode::Eq(Box::new([a?, b?]))))
        .ok_or("Error parsing addition")?
}

fn parse_or<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Or)
        .map(|x| parse_eq(x))
        .fold_first(|a, b| Ok(ParseNode::Or(Box::new([a?, b?]))))
        .ok_or("Error parsing logical or")?
}

fn parse_and<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    tokens
        .split(|x| *x == ProcessedToken::And)
        .map(|x| parse_or(x))
        .fold_first(|a, b| Ok(ParseNode::And(Box::new([a?, b?]))))
        .ok_or("Error parsing logical and")?
}

pub(crate) fn parse<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode, Error> {
    parse_and(tokens)
}
