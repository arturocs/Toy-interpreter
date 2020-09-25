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

fn find_matching_parentheses<'a>(i: &mut usize, tokens: &'a [Token]) -> Result<usize, Error> {
    let mut nested_parentheses = -1;
    let mut last_parentheses = 0;
    let mut found = false;

    dbg!(tokens);
    dbg!(*i);

    for index in (0..=*i).rev() {
        dbg!(&tokens[index]);
        match tokens[index] {
            Token::OpenParentheses => {
                nested_parentheses += 1;
                if nested_parentheses == 0 {
                    last_parentheses = index;
                    found = true;
                    break;
                }
            }
            Token::CloseParentheses => nested_parentheses -= 1,

            _ => {}
        }
    }

    dbg!(last_parentheses);
    if found {
        Ok(last_parentheses)
    } else {
        Err("Unable to find matching parentheses")
    }
}

fn find_prioritary_operation<'a>(i: &mut usize, tokens: &'a [Token]) -> Option<(usize, usize)> {
    let mut start_range = 0;
    let mut end_range = 0;
    let mut found = false;
    dbg!(tokens);
    dbg!(*i);
    dbg!(&tokens[*i]);
    for index in (0..=*i).rev() {
        dbg!(index);
        dbg!(&tokens[index]);
        match tokens[index] {
            Token::Mul | Token::Div | Token::Rem => {
                start_range = index - 1;
                if !found {
                    end_range = index + 1;
                    found = true;
                }
            }
            Token::Number(_) => {}
            _ if found => break,
            _ => {}
        }
    }
    if found {
        Some((start_range, end_range))
    } else {
        None
    }
}

fn parse_add<'a>(
    mut i: &mut usize,
    tokens: &'a [Token],
    current_number: ParseNode<'a>,
) -> Result<ParseNode<'a>, Error> {
    dbg!(&i);
    dbg!(tokens);
    let prioritary_range = find_prioritary_operation(&mut i, tokens);
    dbg!(prioritary_range);

    match prioritary_range {
        Some((start, end)) => {
            let priority = &tokens[start..=end];
            let current = &tokens[end + 1..=(*i + 2)];
            let next = &tokens[..start];
            let priority_result = parse(&mut (priority.len() - 1), priority);
            dbg!(&priority_result);
            let without_op = &current[1..];
            let current_result = match dbg!(current.first()) {
                Some(Token::Add) => Ok(ParseNode::Add(Box::new([
                    priority_result?,
                    parse(&mut (without_op.len() - 1), without_op)?,
                ]))),
                None => priority_result,
                a => todo!("{:?}", a),
            };
            dbg!(&current_result);

            let next_result = match dbg!(next.last()) {
                Some(Token::Mul) => Ok(ParseNode::Mul(join_nodes(
                    next.len() - 2,
                    next,
                    current_result?,
                )?)),
                Some(Token::Div) => Ok(ParseNode::Div(join_nodes(
                    next.len() - 2,
                    next,
                    current_result?,
                )?)),
                Some(Token::Add) => Ok(ParseNode::Add(join_nodes(
                    next.len() - 2,
                    next,
                    current_result?,
                )?)),
                Some(Token::Number(_)) => parse_number(&mut i, tokens),
                None => current_result,
                a => todo!("{:?}", a),
            };

            dbg!(priority);
            dbg!(current);
            dbg!(next);

            //dbg!(&priority_result);
            //dbg!(current_result);
            dbg!(&next_result);

            next_result
            //todo!()
        }
        None => Ok(ParseNode::Add(Box::new([
            parse(&mut i, tokens)?,
            current_number,
        ]))),
    }
}

fn parse_number<'a>(i: &mut usize, tokens: &'a [Token]) -> Result<ParseNode<'a>, Error> {
    let token_to_node = |i| match tokens.get(i) {
        Some(Token::Number(n)) => ParseNode::Number(Val::Number(*n)),
        _ => panic!("Expected number"),
    };
    let current_number = token_to_node(*i);
    dbg!(&current_number);

    match (*i).checked_sub(2) {
        Some(mut j) => match tokens[j + 1] {
            Token::Mul => {
                match find_prioritary_operation(&mut (j + 1), tokens) {
                    Some((a, b)) => {
                        match dbg!(&tokens[a + 1]) {
                            Token::Add => Ok(ParseNode::Add(Box::new([
                                parse(&mut j, &tokens[..=a + 1])?,
                                parse(&mut j, &tokens[a..=b])?,
                            ]))),
                            _ => Ok(ParseNode::Mul(join_nodes(a, tokens, current_number)?))//panic!("only add {:?}",&tokens[a..=b]),
                        }
                        // parse(i, &tokens[a..=b])
                    }
                    None => panic!("bug"),
                }
                // todo!()
            } //Ok(ParseNode::Mul(join_nodes(i, tokens, current_number)?)),
            Token::Div => Ok(ParseNode::Div(join_nodes(j, tokens, current_number)?)),
            Token::Rem => Ok(ParseNode::Rem(join_nodes(j, tokens, current_number)?)),
            Token::Add => parse_add(&mut j, tokens, current_number),
            _ => todo!("{:?}", tokens[j + 1]),
        },
        None => Ok(current_number),
    }
}

//Find better name
fn join_nodes<'a>(
    mut i: usize,
    block: &'a [Token],
    next: ParseNode<'a>,
) -> Result<Box<[ParseNode<'a>; 2]>, Error> {
    Ok(Box::new([parse(&mut i, block)?, next]))
}

fn parse_parentheses<'a>(i: &mut usize, tokens: &'a [Token]) -> Result<ParseNode<'a>, Error> {
    *i -= 1;
    let parentheses_end = find_matching_parentheses(i, tokens)?;
    dbg!(parentheses_end);
    let parentheses_slice = &tokens[parentheses_end + 1..=*i];
    let next = &tokens[..parentheses_end];
    dbg!(next);
    dbg!(parentheses_slice);
    let block = parse(&mut (parentheses_slice.len() - 1), parentheses_slice);

    match dbg!(next.last()) {
        Some(Token::Mul) => Ok(ParseNode::Mul(join_nodes(next.len() - 2, next, block?)?)),
        Some(Token::Div) => Ok(ParseNode::Div(join_nodes(next.len() - 2, next, block?)?)),
        Some(Token::Add) => Ok(ParseNode::Add(join_nodes(next.len() - 2, next, block?)?)),
        Some(Token::Number(_)) => parse_number(i, tokens),
        None => block,
        a => todo!("{:?}", a),
    }
}

pub(crate) fn parse<'a>(i: &mut usize, tokens: &'a [Token]) -> Result<ParseNode<'a>, Error> {
    dbg!(*i);
    dbg!(tokens);
    match dbg!(tokens.get(*i)) {
        Some(Token::VarName(_)) => todo!("Variable"),
        Some(Token::Number(_)) => parse_number(i, tokens),
        Some(Token::String(_)) => todo!("String"),
        Some(Token::Bool(_)) => todo!("Bool"),
        Some(Token::OpenParentheses) => panic!("{:#?} \n {} Open par", tokens, i),
        Some(Token::CloseParentheses) => parse_parentheses(i, tokens),
        Some(Token::Mul) => todo!("mul"),
        Some(Token::Div) => todo!("Div"),
        Some(Token::Rem) => todo!("Rem"),
        Some(Token::Add) => {
            panic!("Alone + {:#?} \n {}", tokens, i);
        }
        Some(Token::Sub) => todo!("Sub"),
        Some(Token::Eq) => todo!("Eq"),
        Some(Token::NotEq) => todo!("NotEq"),
        Some(Token::And) => todo!("And"),
        Some(Token::Or) => todo!("Or"),
        Some(Token::Not) => todo!("Not"),
        Some(Token::Comma) => todo!("Comma"),
        Some(Token::Ltoe) => todo!("Ltoe"),
        Some(Token::Gtoe) => todo!("Gtoe"),
        Some(Token::Lt) => todo!("Lt"),
        Some(Token::Gt) => todo!("Gt"),
        None => panic!("No token found"),
    }
}
