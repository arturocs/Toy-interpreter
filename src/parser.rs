use crate::tokenizer::*;

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
