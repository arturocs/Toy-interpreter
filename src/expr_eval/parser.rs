use crate::expr_eval::tokenizer::Token;
use crate::expr_eval::val::Val;

type Error = &'static str;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum ParseNode<'a> {
    VarName(&'a str),
    Number(Val),
    String(Val),
    Bool(Val),
    //FnCallStart(&'a str),
    //VecAccessStart(&'a str),
    //Dot
    Neg(Box<ParseNode<'a>>),
    Mul(Box<[ParseNode<'a>; 2]>),
    Div(Box<[ParseNode<'a>; 2]>),
    Rem(Box<[ParseNode<'a>; 2]>),
    Add(Box<[ParseNode<'a>; 2]>),
    Sub(Box<[ParseNode<'a>; 2]>),
    Eq(Box<[ParseNode<'a>; 2]>),
    NotEq(Box<[ParseNode<'a>; 2]>),
    Gt(Box<[ParseNode<'a>; 2]>),
    Lt(Box<[ParseNode<'a>; 2]>),
    Gtoe(Box<[ParseNode<'a>; 2]>),
    Ltoe(Box<[ParseNode<'a>; 2]>),
    And(Box<[ParseNode<'a>; 2]>),
    Or(Box<[ParseNode<'a>; 2]>),
    Not(Box<ParseNode<'a>>),
}
#[derive(PartialEq, Debug, Clone)]
pub(crate) enum ProcessedToken<'a> {
    VarName(&'a str),
    Number(f64),
    String(&'a str),
    Bool(bool),
    FnCall(&'a str, Vec<ProcessedToken<'a>>),
    VecAccess(&'a str, Vec<ProcessedToken<'a>>),
    //Dot
    Parentheses(Vec<ProcessedToken<'a>>),
    Brackets(Vec<ProcessedToken<'a>>),
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
    Not,
    Comma,
    TempNeg,
    Neg(Box<ProcessedToken<'a>>),
}

fn find_matching_parentheses<'a>(i: usize, tokens: &'a [Token]) -> Result<usize, Error> {
    let mut nested_parentheses = -1;
    let mut last_parentheses = 0;
    let mut found = false;

    //dbg!(tokens);
    //dbg!(i);

    for index in i + 1..tokens.len() {
        //    dbg!(&tokens[index]);
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

    // dbg!(last_parentheses);
    if found {
        Ok(last_parentheses)
    } else {
        Err("Unable to find matching parentheses")
    }
}

fn process_parentheses<'a>(
    tokens: &'a [Token],
    i: &mut usize,
) -> Result<ProcessedToken<'a>, Error> {
    //llamar a process tokens en el nuevo rango
    let parentheses_end = find_matching_parentheses(*i, tokens)?;
    // dbg!(parentheses_end);
    let parentheses_content = &tokens[*i + 1..parentheses_end];
    *i = parentheses_end;
    Ok(ProcessedToken::Parentheses(process_tokens(
        parentheses_content,
    )?))
}

pub(crate) fn process_tokens<'a>(tokens: &'a [Token]) -> Result<Vec<ProcessedToken<'a>>, Error> {
    let mut processed_tokens = Vec::with_capacity(tokens.len());
    let mut index = 0;
    let mut error = "";
    while index < tokens.len() {
        match tokens[index] {
            Token::VarName(a) => processed_tokens.push(ProcessedToken::VarName(a)),
            Token::Number(a) => processed_tokens.push(ProcessedToken::Number(a)),
            Token::String(a) => processed_tokens.push(ProcessedToken::String(a)),
            Token::Bool(a) => processed_tokens.push(ProcessedToken::Bool(a)),
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
            Token::Not => processed_tokens.push(ProcessedToken::Not),
            Token::Comma => todo!("Commas"),
            // Token::FnCallStart(_) => todo!("Functions"),
            // Token::VecAccessStart(_) => todo!("Vectors"),
            // Token::OpenSBrackets => todo!("Vectors"),
            // Token::CloseSBrackets => todo!("Vectors"),
        }
        index += 1;
    }
    if error == "" {
        Ok(processed_tokens)
    } else {
        Err(error)
    }
}

fn parse_mul<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode<'a>, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Mul)
        .map(|x| match &x[0] {
            ProcessedToken::Number(a) => Ok(ParseNode::Number(Val::Number(*a))),
            ProcessedToken::Parentheses(a) => parse_add(a),
            _ => panic!("dafuq"),
        })
        .fold_first(|a, b| Ok(ParseNode::Mul(Box::new([a?, b?]))))
        .ok_or("err")?
}

fn parse_div<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode<'a>, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Div)
        .map(|x| parse_mul(x))
        .fold_first(|a, b| Ok(ParseNode::Div(Box::new([a?, b?]))))
        .ok_or("err")?
}

fn check_negatives<'a>(tokens: &'a [ProcessedToken]) -> Vec<ProcessedToken<'a>> {
    tokens
        .iter()
        .enumerate()
        .map(|(i, t)| {
            if *t == ProcessedToken::Sub {
                if i == 0 {
                    ProcessedToken::TempNeg
                } else {
                    match tokens[i - 1] {
                        ProcessedToken::Number(_)
                        | ProcessedToken::CloseParentheses
                        | ProcessedToken::VarName(_) => ProcessedToken::Sub,
                        _ => ProcessedToken::TempNeg,
                    }
                }
            } else {
                t.clone()
            }
        })
        .collect::<Vec<_>>()
}

fn parse_sub<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode<'a>, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Sub)
        .map(|x| parse_div(x))
        .fold_first(|a, b| Ok(ParseNode::Sub(Box::new([a?, b?]))))
        .ok_or("err")?
}

fn parse_add<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode<'a>, Error> {
    tokens
        .split(|x| *x == ProcessedToken::Add)
        .map(|x| parse_sub(x))
        .fold_first(|a, b| Ok(ParseNode::Add(Box::new([a?, b?]))))
        .ok_or("err")?
}

pub(crate) fn parse<'a>(tokens: &'a [ProcessedToken]) -> Result<ParseNode<'a>, Error> {
    parse_add(tokens)
}
