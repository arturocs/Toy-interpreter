use crate::tokenizer::ExprToken;
type Error = &'static str;

#[derive(PartialEq, Debug, Clone)]
pub enum ProcessedExprToken {
    VarName(String),
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    //FnCall(&'a str, Vec<ProcessedToken<'a>>),
    //Dot
    VecAccess(String, Vec<Vec<ProcessedExprToken>>),
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

fn find_matching_parentheses(i: usize, tokens: &[ExprToken]) -> Result<usize, Error> {
    let mut nested_parentheses = -1;

    for (index, token) in tokens.iter().enumerate().skip(i + 1) {
        match token {
            ExprToken::CloseParentheses => {
                nested_parentheses += 1;
                if nested_parentheses == 0 {
                    return Ok(index);
                }
            }
            ExprToken::OpenParentheses => nested_parentheses -= 1,
            _ => {}
        }
    }
    Err("Unable to find matching parentheses")
}

fn find_matching_square_bracket(i: usize, tokens: &[ExprToken]) -> Result<usize, Error> {
    let mut nested_bracket = -1;
    for (index, token) in tokens.iter().enumerate().skip(i + 1) {
        match token {
            ExprToken::CloseSBrackets | ExprToken::VecAccessStart(_) => {
                nested_bracket += 1;
                if nested_bracket == 0 {
                    return Ok(index);
                }
            }
            ExprToken::OpenSBrackets => nested_bracket -= 1,
            _ => {}
        }
    }

    Err("Unable to find matching square bracket")
}

fn process_vector<'a>(tokens: &'a [ExprToken], i: &mut usize) -> Result<ProcessedExprToken, Error> {
    let bracket_end = find_matching_square_bracket(*i, tokens)?;
    let parentheses_content = &tokens[*i + 1..bracket_end];
    *i = bracket_end;
    Ok(ProcessedExprToken::Vector(process_expr_tokens(
        parentheses_content,
    )?))
}

fn process_vector_access(
    tokens: &[ExprToken],
    i: &mut usize,
    capture: &str,
) -> Result<ProcessedExprToken, Error> {
    let bracket_end = find_matching_square_bracket(*i, tokens)?;
    let name = capture.trim_end_matches('[').to_string();
    match tokens[bracket_end] {
        ExprToken::VecAccessStart(_) => {
            let mut v = Vec::with_capacity(5);
            //Do while loop
            while {
                let a = find_matching_square_bracket(*i, tokens)?;
                let brackets_content = &tokens[*i + 1..a];
                v.push(process_expr_tokens(brackets_content)?);
                *i = a;
                a < tokens.len() - 1
            } {}
            Ok(ProcessedExprToken::VecAccess(name, v))
        }
        ExprToken::CloseSBrackets => {
            let brackets_content = &tokens[*i + 1..bracket_end];
            *i = bracket_end;
            Ok(ProcessedExprToken::VecAccess(
                name,
                vec![process_expr_tokens(brackets_content)?],
            ))
        }
        _ => Err("Erro preprocessing vector access"),
    }
}

fn process_parentheses(tokens: &[ExprToken], i: &mut usize) -> Result<ProcessedExprToken, Error> {
    let parentheses_end = find_matching_parentheses(*i, tokens)?;
    let parentheses_content = &tokens[*i + 1..parentheses_end];
    *i = parentheses_end;
    Ok(ProcessedExprToken::Parentheses(process_expr_tokens(
        parentheses_content,
    )?))
}

fn process_not_and_negatives(tokens: &[ProcessedExprToken]) -> Vec<ProcessedExprToken> {
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

pub fn process_expr_tokens(tokens: &[ExprToken]) -> Result<Vec<ProcessedExprToken>, Error> {
    let mut processed_tokens = Vec::with_capacity(tokens.len());
    let mut index = 0;
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
            ExprToken::CloseParentheses => return Err("Unmatched )"),
            ExprToken::Div => processed_tokens.push(ProcessedExprToken::Div),
            ExprToken::Mul => processed_tokens.push(ProcessedExprToken::Mul),
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
            ExprToken::CloseSBrackets => return Err("Unmatched ]"),
            ExprToken::Comma => processed_tokens.push(ProcessedExprToken::Comma),

            ExprToken::Null => processed_tokens.push(ProcessedExprToken::Null),
        }
        index += 1;
    }

    Ok(process_not_and_negatives(&processed_tokens))
}
