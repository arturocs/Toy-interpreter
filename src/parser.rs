use crate::tokenizer::*;
use crate::val::Val;

type Error = &'static str;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum ParseNode<'a> {
    If(&'a str, Vec<ParseNode<'a>>, Option<Vec<ParseNode<'a>>>), //If(Expression, If block, Else Block)
    While(&'a str, Vec<ParseNode<'a>>),                          // While(Condition, Block)
    Assignation(&'a str, &'a str),
    Expression(&'a str),
    Print(Box<ParseNode<'a>>),
}

pub(crate) fn check_result<T>(condition: bool, result: T, error: Error) -> Result<T, Error> {
    if condition {
        Ok(result)
    } else {
        panic!(error)
    }
}

fn find_matching_bracket(tokens: &[Token]) -> Result<usize, Error> {
    let mut nested_brackets = -1;
    let mut index = 0;
    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::OpenCBrackets => nested_brackets += 1,
            Token::CloseCBrackets => {
                nested_brackets -= 1;
                if nested_brackets < 0 {
                    index = i;
                    break;
                }
            }
            _ => {}
        }
    }
    check_result(index > 0, index, "Unable to find matching bracket")
}

fn parse_if<'a>(tokens: &'a [Token], i: &mut usize) -> Result<ParseNode<'a>, Error> {
    *i += 1;
    match tokens[*i] {
        Token::Expression(exp) => match tokens[*i + 1] {
            Token::OpenCBrackets => {
                let if_block_end = find_matching_bracket(&tokens[*i..])? + *i + 1;
                let if_body = parse(&tokens[*i + 2..if_block_end - 1])?;
                if if_block_end < tokens.len() && tokens[if_block_end] == Token::Else {
                    let else_block_end =
                        find_matching_bracket(&tokens[if_block_end..])? + if_block_end;
                    let else_body = parse(&tokens[if_block_end + 2..else_block_end])?;
                    *i = else_block_end;
                    Ok(ParseNode::If(exp, if_body, Some(else_body)))
                } else {
                    *i = if_block_end - 1;
                    Ok(ParseNode::If(exp, if_body, None))
                }
            }
            _ => Err("Expected bracket after if expression"),
        },
        _ => Err("Expected expression after if"),
    }
}

fn parse_print<'a>(tokens: &'a [Token], i: &mut usize) -> Result<ParseNode<'a>, Error> {
    *i += 1;
    match tokens[*i] {
        Token::Expression(e) => Ok(ParseNode::Print(Box::new(ParseNode::Expression(e)))),
        _ => Err("Expression to print not found"),
    }
}

fn parse_while<'a>(tokens: &'a [Token], i: &mut usize) -> Result<ParseNode<'a>, Error> {
    *i += 1;
    match tokens[*i] {
        Token::Expression(exp) => match tokens[*i + 1] {
            Token::OpenCBrackets => {
                let block_end = find_matching_bracket(&tokens[*i..])? + *i;
                let body = parse(&tokens[*i + 2..block_end])?;
                *i = block_end;
                Ok(ParseNode::While(exp, body))
            }
            _ => Err("Expected bracket after while expression"),
        },
        _ => Err("Expected expression after while"),
    }
}

fn parse_assignation(assignation_str: &str) -> Result<ParseNode, Error> {
    let mut assignation = assignation_str.split('=');
    let err = "Error parsing asignation";
    Ok(ParseNode::Assignation(
        assignation.next().ok_or(err)?.trim(),
        assignation.next().ok_or(err)?.trim(),
    ))
}

pub(crate) fn parse<'a>(tokens: &'a [Token]) -> Result<Vec<ParseNode<'a>>, Error> {
    let mut ast = vec![];
    let mut error = "";
    let mut i: usize = 0;
    while i < tokens.len() {
        match tokens[i] {
            Token::If => ast.push(parse_if(&tokens, &mut i)?),
            Token::Else => error = "Unmatched else",
            Token::While => ast.push(parse_while(&tokens, &mut i)?),
            Token::Assignation(a) => ast.push(parse_assignation(a)?),
            Token::OpenCBrackets => error = "Unmatched {",
            Token::CloseCBrackets => error = "Unmatched }",
            Token::Expression(exp) => ast.push(ParseNode::Expression(exp)),
            Token::Print => ast.push(parse_print(&tokens, &mut i)?),
        }
        i += 1;
    }
    check_result(error == "", ast, error)
}

#[derive(PartialEq, Debug)]
pub(crate) enum ParseExprNode<'a> {
    VarName(&'a str),
    Number(Val),
    String(Val),
    Bool(Val),
    //FnCallStart(&'a str),
    //VecAccessStart(&'a str),
    //Dot
    Parentheses(Vec<ParseExprNode<'a>>),
    Neg(Box<ParseExprNode<'a>>),
    Mul(Box<ParseExprNode<'a>>, Box<ParseExprNode<'a>>),
    Div(Box<ParseExprNode<'a>>, Box<ParseExprNode<'a>>),
    Rem(Box<ParseExprNode<'a>>, Box<ParseExprNode<'a>>),
    Add(Box<ParseExprNode<'a>>, Box<ParseExprNode<'a>>),
    Sub(Box<ParseExprNode<'a>>, Box<ParseExprNode<'a>>),
    Eq(Box<ParseExprNode<'a>>, Box<ParseExprNode<'a>>),
    NotEq(Box<ParseExprNode<'a>>, Box<ParseExprNode<'a>>),
    And(Box<ParseExprNode<'a>>, Box<ParseExprNode<'a>>),
    Or(Box<ParseExprNode<'a>>, Box<ParseExprNode<'a>>),
    Not(Box<ParseExprNode<'a>>),
}

fn find_matching_parentheses(tokens: &[ExprToken]) -> Result<usize, Error> {
    println!("find matching parentheses tokens {:#?}", &tokens);
    let mut nested_parentheses = -1;
    let mut index = 0;
    for (i, token) in tokens.iter().enumerate() {
        match token {
            ExprToken::OpenParentheses => nested_parentheses += 1,
            ExprToken::CloseParentheses => {
                nested_parentheses -= 1;
                if nested_parentheses < 0 {
                    index = i;
                    break;
                }
            }
            _ => {}
        }
    }
    check_result(index > 0, index + 1, "Unable to find matching bracket")
}
/*
pub(crate) enum ExprToken<'a> {
    VarName(&'a str),
    Number(f64),
    String(&'a str),
    Bool(bool),
    //FnCallStart(&'a str),
    //VecAccessStart(&'a str),
    //Dot
    //OpenSBrackets,
    //CloseSBrackets,

    OpenParentheses,
    CloseParentheses,
    Mul,
    Div,
    Rem,
    Add,
    Sub,
    Eq,
    NotEq,
    And,
    Or,
    Not,
    Comma,
}*/

fn parse_sub<'a>(tokens: &'a [ExprToken], i: &mut usize) -> Result<ParseExprNode<'a>, Error> {
    if *i == 0 {
        //Primer token de tokens
        *i += 1;
        match tokens[*i] {
            ExprToken::Number(n) => Ok(ParseExprNode::Neg(Box::new(ParseExprNode::Number(
                Val::Number(-n),
            )))),
            ExprToken::VarName(s) => todo!("Variables not implemented yet"),
            _ => Err("Negation operator can only be applied to numbers"),
        }
    } else {
        let mut j = *i + 1;
        while j < tokens.len() {
            match tokens[j] {
                ExprToken::VarName(s) => {}
                ExprToken::Number(n) => {}
                ExprToken::String(s) => {}
                ExprToken::Bool(b) => {}
                ExprToken::OpenParentheses => {}
                ExprToken::CloseParentheses => {}
                ExprToken::Mul => {}
                ExprToken::Div => {}
                ExprToken::Rem => {}
                ExprToken::Add => {}
                ExprToken::Sub => {}
                ExprToken::Eq => {}
                ExprToken::NotEq => {}
                ExprToken::And => {}
                ExprToken::Or => {}
                ExprToken::Not => {}
                ExprToken::Comma => {}
            }
            j += 1;
        }
        todo!()
        // Err("Error while parsing negation operator")
    }
}

fn parse_not<'a>(tokens: &'a [ExprToken], i: &mut usize) -> Result<ParseExprNode<'a>, Error> {
    *i += 1;
    match tokens[*i] {
        ExprToken::Bool(b) => Ok(ParseExprNode::Not(Box::new(ParseExprNode::Bool(
            Val::Bool(!b),
        )))),

        ExprToken::OpenParentheses => {
            Ok(ParseExprNode::Not(Box::new(parse_par(&tokens, &mut *i)?)))
        }
        ExprToken::VarName(s) => todo!("Variables not implemented yet"),
        _ => Err("Not operator can only be applied to booleans"),
    }
}

fn parse_par<'a>(tokens: &'a [ExprToken], i: &mut usize) -> Result<ParseExprNode<'a>, Error> {
    // *i += 1;
    println!("parse_par tokens {:#?} {}", &tokens, i);
    let parentheses_end = find_matching_parentheses(&tokens[*i + 1..])? + *i + 1;
    dbg!(parentheses_end);
    let result = Ok(parse_expr(&tokens[*i + 1..parentheses_end])?);
    dbg!(&result);
    *i = parentheses_end + 1;
    //dbg!(parentheses_end);
    result
}

fn parse_mul<'a>(tokens: &'a [ExprToken], i: &mut usize) -> Result<ParseExprNode<'a>, Error> {
    // dbg!(&tokens);
    //dbg!(*i);
    println!("parse_mul {:#?} {}", &tokens, *i);
    //dbg!(&tokens[*i]);
    dbg!(&tokens[*i]);
    if
    /* *i > 1 && */
    tokens.len() >= 3 {
        if tokens[*i + 1] != ExprToken::OpenParentheses {
            //*i += 1;
            Ok(ParseExprNode::Mul(
                Box::new(ParseExprNode::Number(Val::Number(tokens[*i - 1].to_f64()?))),
                Box::new(ParseExprNode::Number(Val::Number(tokens[*i + 1].to_f64()?))),
            ))
        } else {
            *i += 1;
            println!("hello");
            Ok(ParseExprNode::Mul(
                Box::new(ParseExprNode::Number(Val::Number(tokens[*i - 2].to_f64()?))),
                Box::new(parse_par(&tokens[*i - 1..], &mut *i)?),
            ))
        }
    } else {
        Err("Multiplication operator needs two operands")
    }
}

pub(crate) fn parse_expr<'a>(tokens: &'a [ExprToken]) -> Result<ParseExprNode<'a>, Error> {
    //  let mut ast = vec![];
    println!("parse_expr {:#?}", &tokens);
    let mut ret = Ok(ParseExprNode::VarName(""));
    let mut error = "";
    let mut i: usize = 0;
    while i < tokens.len() {
        dbg!(&ret);
        //if ret? != ParseExprNode::VarName("empty"){ ret = *dbg!(&ret);}
        match tokens[i] {
            ExprToken::Sub => ret = Ok(parse_sub(&tokens, &mut i)?),
            ExprToken::OpenParentheses => ret = Ok(parse_par(&tokens, &mut i)?),
            ExprToken::CloseParentheses => panic!("Unmatched )"),
            ExprToken::Mul => ret = Ok(parse_mul(&tokens, &mut i)?),
            ExprToken::Number(a) => {dbg!(a);}
            _ => panic!("Operation not implemented {:?} {}", tokens[i], i),
        }
        i += 1;
    }
    ret
    // check_result(error == "", ast, error)
}
