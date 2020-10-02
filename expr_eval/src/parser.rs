use crate::tokenizer::ExprToken;
use crate::val::Val;

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
#[derive(PartialEq, Debug, Clone)]
pub enum ProcessedExprToken {
    VarName(String),
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    //FnCall(&'a str, Vec<ProcessedToken<'a>>),
    //Dot
    VecAccess(String, Vec<ProcessedExprToken>),
    Vector(Vec<ProcessedExprToken>),
    OpenSBrackets,
    CloseSBrackets,
    Parentheses(Vec<ProcessedExprToken>),
    Brackets(Vec<ProcessedExprToken>),
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
    Not(Option<Box<ProcessedExprToken>>),
    Comma,
    Neg(Box<ProcessedExprToken>),
}

fn find_matching_parentheses<'a>(i: usize, tokens: &'a [ExprToken]) -> Result<usize, Error> {
    let mut nested_parentheses = -1;
    let mut last_parentheses = 0;
    let mut found = false;
    for index in i + 1..tokens.len() {
        match tokens[index] {
            ExprToken::CloseParentheses => {
                nested_parentheses += 1;
                if nested_parentheses == 0 {
                    last_parentheses = index;
                    found = true;
                    break;
                }
            }
            ExprToken::OpenParentheses => nested_parentheses -= 1,
            _ => {}
        }
    }
    if found {
        Ok(last_parentheses)
    } else {
        Err("Unable to find matching parentheses")
    }
}

fn find_matching_square_bracket<'a>(i: usize, tokens: &'a [ExprToken]) -> Result<usize, Error> {
    let mut nested_bracket = -1;
    let mut last_bracket = 0;
    let mut found = false;
    for index in i + 1..tokens.len() {
       // dbg!(index);
       // dbg!(&tokens[index]);
        match tokens[index] {
            ExprToken::CloseSBrackets => {
                nested_bracket += 1;
                if nested_bracket == 0 {
                    last_bracket = index;
                    found = true;
                    break;
                }
            }
            ExprToken::OpenSBrackets => nested_bracket -= 1,
            _ => {}
        }
    }
    if found {
        Ok(last_bracket)
    } else {
        Err("Unable to find matching square bracket")
    }
}

fn process_vector<'a>(tokens: &'a [ExprToken], i: &mut usize) -> Result<ProcessedExprToken, Error> {
    // dbg!(&i);
   //  dbg!(tokens);
     let bracket_end = find_matching_square_bracket(*i, tokens)?;
     // dbg!(parentheses_end);
     let parentheses_content = &tokens[*i + 1..bracket_end];
     *i = bracket_end;
     Ok(ProcessedExprToken::Vector(process_expr_tokens(
         parentheses_content,
     )?))
 }
 
 fn process_vector_access<'a>(
     tokens: &'a [ExprToken],
     i: &mut usize,
     name: &String,
 ) -> Result<ProcessedExprToken, Error> {
     let bracket_end = find_matching_square_bracket(*i, tokens)?;
     let parentheses_content = &tokens[*i + 1..bracket_end];
 
     *i = bracket_end;
     Ok(ProcessedExprToken::VecAccess(
         name.clone(),
         process_expr_tokens(parentheses_content)?,
     ))
 }
 
fn process_parentheses<'a>(
    tokens: &'a [ExprToken],
    i: &mut usize,
) -> Result<ProcessedExprToken, Error> {
    let parentheses_end = find_matching_parentheses(*i, tokens)?;
    // dbg!(parentheses_end);
    let parentheses_content = &tokens[*i + 1..parentheses_end];
    *i = parentheses_end;
    Ok(ProcessedExprToken::Parentheses(process_expr_tokens(
        parentheses_content,
    )?))
}

fn process_not_and_negatives<'a>(tokens: Vec<ProcessedExprToken>) -> Vec<ProcessedExprToken> {
    let mut processed_tokens = Vec::with_capacity(tokens.len());
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            ProcessedExprToken::Sub => {
                if i == 0 {
                    processed_tokens.push(ProcessedExprToken::Neg(Box::new(tokens[i + 1].clone())));
                    i += 1;
                } else {
                    match tokens[i - 1] {
                        ProcessedExprToken::Number(_)
                        | ProcessedExprToken::CloseParentheses
                        | ProcessedExprToken::Neg(_)
                        | ProcessedExprToken::VarName(_) => {
                            processed_tokens.push(ProcessedExprToken::Sub)
                        }
                        _ => {
                            processed_tokens
                                .push(ProcessedExprToken::Neg(Box::new(tokens[i + 1].clone())));
                            i += 1;
                        }
                    }
                }
            }
            ProcessedExprToken::Not(_) => {
                processed_tokens.push(ProcessedExprToken::Not(Some(Box::new(
                    tokens[i + 1].clone(),
                ))));
                i += 1;
            }
            a => processed_tokens.push(a.clone()),
        }
        i += 1;
    }
    processed_tokens
}

pub fn process_expr_tokens<'a>(tokens: &'a [ExprToken]) -> Result<Vec<ProcessedExprToken>, Error> {
    let mut processed_tokens = Vec::with_capacity(tokens.len());
    let mut index = 0;
    let mut error = "";
    while index < tokens.len() {
        match &tokens[index] {
            ExprToken::VarName(a) => processed_tokens.push(ProcessedExprToken::VarName(a.clone())),
            ExprToken::Number(a) => processed_tokens.push(ProcessedExprToken::Number(*a)),
            ExprToken::String(a) => processed_tokens.push(ProcessedExprToken::String(
                a.trim_matches('"').replace("\\n", "\n"),
            )),
            ExprToken::Bool(a) => processed_tokens.push(ProcessedExprToken::Bool(*a)),
            ExprToken::OpenParentheses => {
                processed_tokens.push(process_parentheses(tokens, &mut index)?)
            }
            ExprToken::CloseParentheses => {
                error = "Unmatched )";
                break;
            }
            ExprToken::Mul => processed_tokens.push(ProcessedExprToken::Mul),
            ExprToken::Div => processed_tokens.push(ProcessedExprToken::Div),
            ExprToken::Rem => processed_tokens.push(ProcessedExprToken::Rem),
            ExprToken::Add => processed_tokens.push(ProcessedExprToken::Add),
            ExprToken::Sub => processed_tokens.push(ProcessedExprToken::Sub),
            ExprToken::Eq => processed_tokens.push(ProcessedExprToken::Eq),
            ExprToken::NotEq => processed_tokens.push(ProcessedExprToken::NotEq),
            ExprToken::Gt => processed_tokens.push(ProcessedExprToken::Gt),
            ExprToken::Lt => processed_tokens.push(ProcessedExprToken::Lt),
            ExprToken::Gtoe => processed_tokens.push(ProcessedExprToken::Gtoe),
            ExprToken::Ltoe => processed_tokens.push(ProcessedExprToken::Ltoe),
            ExprToken::And => processed_tokens.push(ProcessedExprToken::And),
            ExprToken::Or => processed_tokens.push(ProcessedExprToken::Or),
            ExprToken::Not => processed_tokens.push(ProcessedExprToken::Not(None)),
            ExprToken::VecAccessStart(name) => {
                processed_tokens.push(process_vector_access(tokens, &mut index, name)?)
            }
            ExprToken::OpenSBrackets => processed_tokens.push(process_vector(tokens, &mut index)?),
            ExprToken::CloseSBrackets => {
                error = "Unmatched ]";
                break;
            }
            ExprToken::Comma => processed_tokens.push(ProcessedExprToken::Comma),
            
            ExprToken::Null => processed_tokens.push(ProcessedExprToken::Null),
        }
        index += 1;
    }
    if error == "" {
        Ok(process_not_and_negatives(processed_tokens))
    } else {
        Err(error)
    }
}

fn neg_to_node<'a>(a: &'a Box<ProcessedExprToken>) -> Result<ParseExprNode, Error> {
    let token_slice = std::slice::from_ref(a.as_ref());
    let node = parse_add(token_slice)?;
    Ok(ParseExprNode::Neg(Box::new(node)))
}

fn parse_vector(vector: &[ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    Ok(ParseExprNode::Vector(
        vector
            .split(|x| *x == ProcessedExprToken::Comma)
            .map(|t| parse(t))
            .collect::<Result<Vec<_>, _>>()?,
    ))
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
            ProcessedExprToken::Parentheses(a) => parse(a),
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

pub fn parse<'a>(tokens: &'a [ProcessedExprToken]) -> Result<ParseExprNode, Error> {
    parse_and(tokens)
}
