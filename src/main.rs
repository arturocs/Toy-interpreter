use regex::{Regex, RegexBuilder};
use std::{env, fmt, fs, str::FromStr};
use v_eval::{Eval, Value};
#[macro_use]
extern crate lazy_static;

#[derive(PartialEq, Debug)]
enum Token<'a> {
    If,
    Else,
    While,
    Assignation(&'a str),
    OpenCBrackets,
    CloseCBrackets,
    Expression(&'a str),
    Print,
}

#[derive(PartialEq, Debug)]
enum Val {
    Bool(bool),
    Number(f64),
    Str(String),
    Vec(Vec<Val>),
    Null,
}

impl FromStr for Val {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s.trim() {
            "true" => Ok(Val::Bool(true)),
            "false" => Ok(Val::Bool(false)),
            "Null" => Ok(Val::Null),
            st if st.starts_with('"') && st.ends_with('"') => Ok(Val::Str(st.to_string())),
            _ => Err("Error while parsing Value"),
        };
        parse_f64_and_vec(s, res)
    }
}

fn parse_f64_and_vec(s: &str, res: Result<Val, &'static str>) -> Result<Val, &'static str> {
    if res.is_err() {
        match s.parse::<f64>().map_err(|_| "Error parsing number") {
            Ok(n) => Ok(Val::Number(n)),
            Err(_) => match s {
                st if st.starts_with('[') && s.ends_with(']') => to_vec(st),
                _ => res,
            },
        }
    } else {
        res
    }
}

fn to_vec(s: &str) -> Result<Val, &'static str> {
    if s.starts_with('[') && s.ends_with(']') {
        Ok(Val::Vec(
            s.trim_matches(|c| c == '[' || c == ']')
                .split(",")
                .map(|v| Ok(v.parse::<Val>()?))
                .collect::<Result<Vec<Val>, &'static str>>()?,
        ))
    } else {
        Err("Error while parsing Vector")
    }
}
impl fmt::Display for Val {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            Val::Bool(b) => b.to_string(),
            Val::Number(f) => f.to_string(),
            Val::Str(s) => s.clone(),
            Val::Vec(v) => {
                "[".to_string()
                    + &v.into_iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                    + "]"
            }
            Val::Null => "Null".to_string(),
        };
        write!(fmt, "{}", val)
    }
}

#[derive(PartialEq, Debug, Clone)]
enum ParseNode<'a> {
    If(&'a str, Vec<ParseNode<'a>>, Option<Vec<ParseNode<'a>>>), //If(Expression, If block, Else Block)
    While(&'a str, Vec<ParseNode<'a>>),                          // While(Condition, Block)
    Assignation(&'a str, &'a str),
    Expression(&'a str),
    Print(Box<ParseNode<'a>>),
}

fn is_assignation(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^\{\}\n=]+\s*=\s*[^\{\}\n=]+").unwrap();
    }
    RE.is_match(text)
}

fn check_result<T>(condition: bool, result: T, error: &'static str) -> Result<T, &'static str> {
    if condition {
        Ok(result)
    } else {
        Err(error)
    }
}

fn find_matching_bracket(tokens: &[Token]) -> Result<usize, &'static str> {
    let mut nested_brackets = -1;
    let mut index = 0;
    for i in 0..tokens.len() {
        match tokens[i] {
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

fn tokenizer(source_code: &str) -> Vec<Token> {
    let patterns = [
        r"\s*if\s+",
        r"\s*else\s+",
        r"\s*while\s+",
        r"\s*print\s+",
        r"\{",
        r"\}",
        r"[^\{\}\n=]+\s*=\s*[^\{\}\n=]+", //Assignation
        r"[^\{\}\n]+",                    //Everything else
    ]
    .join("|");

    RegexBuilder::new(&patterns)
        //.unicode(false)
        .build()
        .unwrap()
        .captures_iter(source_code)
        .map(|cap| cap.get(0).unwrap().as_str().trim())
        .filter(|s| s != &"")
        .map(|cap| match cap.trim() {
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "{" => Token::OpenCBrackets,
            "}" => Token::CloseCBrackets,
            "print" => Token::Print,
            a if is_assignation(a) => Token::Assignation(a),
            expression => Token::Expression(expression),
        })
        .collect()
}

fn parse_if<'a>(tokens: &'a [Token], i: &mut usize) -> Result<Vec<ParseNode<'a>>, &'static str> {
    let mut ast = vec![];
    let mut error = "";
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
                    ast.push(ParseNode::If(exp, if_body, Some(else_body)));
                    *i = else_block_end;
                } else {
                    ast.push(ParseNode::If(exp, if_body, None));
                    *i = if_block_end - 1;
                }
            }
            _ => error = "Expected bracket after if expression",
        },
        _ => error = "Expected expression after if",
    }
    check_result(error == "", ast, error)
}

fn parse_print<'a>(tokens: &'a [Token], i: &mut usize) -> Result<ParseNode<'a>, &'static str> {
    *i += 1;
    match tokens[*i] {
        Token::Expression(e) => Ok(ParseNode::Print(Box::new(ParseNode::Expression(e)))),
        _ => Err("Expression to print not found"),
    }
}

fn parse_while<'a>(tokens: &'a [Token], i: &mut usize) -> Result<Vec<ParseNode<'a>>, &'static str> {
    let mut ast = vec![];
    let mut error = "";
    *i += 1;
    match tokens[*i] {
        Token::Expression(exp) => match tokens[*i + 1] {
            Token::OpenCBrackets => {
                let block_end = find_matching_bracket(&tokens[*i..])? + *i;
                let body = parse(&tokens[*i + 2..block_end])?;
                ast.push(ParseNode::While(exp, body));
                *i = block_end;
            }
            _ => error = "Expected bracket after while expression",
        },
        _ => error = "Expected expression after while",
    }
    check_result(error == "", ast, error)
}

fn parse_assignation(assignation_str: &str) -> Result<ParseNode, &'static str> {
    let mut assignation = assignation_str.split("=");
    let err = "Error parsing asignation";
    Ok(ParseNode::Assignation(
        assignation.next().ok_or(err)?.trim(),
        assignation.next().ok_or(err)?.trim(),
    ))
}

fn parse<'a>(tokens: &'a [Token]) -> Result<Vec<ParseNode<'a>>, &'static str> {
    let mut ast = vec![];
    let mut error = "";
    let mut i: usize = 0;
    while i < tokens.len() {
        match tokens[i] {
            Token::If => ast.extend(parse_if(&tokens, &mut i)?),
            Token::Else => error = "Unmatched else",
            Token::While => ast.extend(parse_while(&tokens, &mut i)?),
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

fn execute_if(
    expr: &str,
    if_block: &[ParseNode],
    else_block: &Option<Vec<ParseNode>>,
    mut env: Eval,
) -> Result<Eval, &'static str> {
    let mut error = "";
    match env.eval(expr).ok_or("Error evaluating if expression")? {
        Value::Bool(true) => {
            env = execute(if_block, env)?;
        }
        Value::Bool(false) => match else_block {
            Some(e) => {
                env = execute(e, env)?;
            }
            None => {}
        },
        _ => error = "if statement only works with booleans",
    }
    check_result(error == "", env, error)
}

fn execute_while(expr: &str, block: &[ParseNode], mut env: Eval) -> Result<Eval, &'static str> {
    while env.eval(expr).ok_or("Error evaluating while expression")? == Value::Bool(true) {
        env = execute(block, env)?
    }
    Ok(env)
}

fn value_vec_to_string(vector: Vec<Value>) -> String {
    "[".to_string()
        + &vector
            .into_iter()
            .map(value_to_string)
            .collect::<Vec<_>>()
            .join(", ")
        + "]"
}

fn value_to_string(val: Value) -> String {
    match val {
        Value::Bool(b) => b.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Str(s) => s,
        Value::Range(r) => format!("{:?}", r),
        Value::Vec(v) => value_vec_to_string(v),
        Value::None => "None".to_string(),
    }
}

fn execute_assignation(variable: &str, value: &str, env: Eval) -> Result<Eval, &'static str> {
    let b = value_to_string(env.eval(value).ok_or("Error evaluating expression")?);
    Ok(env
        .insert(variable, &b)
        .map_err(|_| "Error assigning variable")?)
}

fn execute_print(expression: &Box<ParseNode>, env: Eval) -> Result<Eval, &'static str> {
    let mut error = "";
    match expression.as_ref() {
        ParseNode::Expression(expr) => {
            println!(
                "{}",
                value_to_string(env.eval(expr).ok_or("Error evaluating print expression")?)
            );
        }
        _ => error = "Only expressions can be printed",
    }
    check_result(error == "", env, error)
}

fn execute_expression(expr: &str, env: Eval) -> Result<Eval, &'static str> {
    env.eval(expr).ok_or("Error evaluating expression")?;
    Ok(env)
}

fn execute(ast: &[ParseNode], mut env: Eval) -> Result<Eval, &'static str> {
    let mut i: usize = 0;
    while i < ast.len() {
        //dbg!(&ast[i]);
        match &ast[i] {
            ParseNode::If(expr, if_block, else_block) => {
                env = execute_if(expr, if_block, else_block, env)?
            }
            ParseNode::While(expr, block) => env = execute_while(expr, block, env)?,
            ParseNode::Assignation(variable, value) => {
                env = execute_assignation(variable, value, env)?;
            }
            ParseNode::Expression(expr) => env = execute_expression(expr, env)?,
            ParseNode::Print(expression) => env = execute_print(expression, env)?,
        }
        i += 1;
    }
    Ok(env)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Eval::default();
    let filename = env::args().nth(1).ok_or("Missing argument")?;
    let contents = fs::read_to_string(filename)?;
    let instructions = tokenizer(&contents);
    //let instructions = dbg!(instructions);
    let ast = parse(&instructions)?;
    //dbg!(&ast);
    execute(&ast, env)?;
    Ok(())
}
