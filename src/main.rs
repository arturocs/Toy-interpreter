use regex::{Regex, RegexBuilder};
use std::env;
use std::fs;
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
        static ref RE: Regex = Regex::new(r"\w+ += +.+").unwrap();
    }
    RE.is_match(text)
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
    if index > 0 {
        Ok(index)
    } else {
        Err("Unable to find matching bracket")
    }
}

fn tokenizer(source_code: &str) -> Vec<Token> {
    let patterns = [
        r"\s*if\s+",
        r"\s*else\s+",
        r"\s*while\s+",
        r"\s*print\s+",
        r"\{",
        r"\}",
        r"\w+ += +.+", //Assignation
        r"[^\{\}\n]+", //Everything else
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

fn parse_if<'a>(tokens: &'a [Token], i: &mut usize) -> Vec<ParseNode<'a>> {
    let mut ast = vec![];
    match tokens[*i + 1] {
        Token::Expression(exp) => match tokens[*i + 2] {
            Token::OpenCBrackets => {
                let if_block_end = find_matching_bracket(&tokens[*i + 1..]).unwrap() + *i + 2;
                let if_body = parse(&tokens[*i + 3..if_block_end - 1]);
                if if_block_end < tokens.len() && tokens[if_block_end] == Token::Else {
                    let else_block_end =
                        find_matching_bracket(&tokens[if_block_end..]).unwrap() + if_block_end;
                    let else_body = parse(&tokens[if_block_end + 2..else_block_end]);
                    ast.push(ParseNode::If(exp, if_body, Some(else_body)));
                    *i = else_block_end;
                } else {
                    ast.push(ParseNode::If(exp, if_body, None));
                    *i = if_block_end - 1;
                }
            }
            _ => panic!("Expected bracket after if expression"),
        },
        _ => panic!("Expected expression after if"),
    }
    ast
}

fn parse_print<'a>(tokens: &'a [Token], i: &mut usize) -> ParseNode<'a> {
    match tokens[*i + 1] {
        Token::Expression(e) => {
            *i += 1;
            ParseNode::Print(Box::new(ParseNode::Expression(e)))
        }
        _ => panic!(
            "Expression to print not found i:{}, tokens: {:#?}",
            i, tokens
        ),
    }
}

fn parse_while<'a>(tokens: &'a [Token], i: &mut usize) -> Vec<ParseNode<'a>> {
    let mut ast = vec![];
    match tokens[*i + 1] {
        Token::Expression(exp) => match tokens[*i + 2] {
            Token::OpenCBrackets => {
                let block_end = find_matching_bracket(&tokens[*i + 1..]).unwrap() + *i + 2;
                let body = parse(&tokens[*i + 3..block_end - 1]);
                ast.push(ParseNode::While(exp, body));
                *i = block_end - 1;
            }
            _ => panic!("Expected bracket after while expression"),
        },
        _ => panic!("Expected expression after while"),
    }
    ast
}

fn parse<'a>(tokens: &'a [Token]) -> Vec<ParseNode<'a>> {
    let mut ast = vec![];
    let mut i: usize = 0;
    while i < tokens.len() {
        match tokens[i] {
            Token::If => ast.extend(parse_if(&tokens, &mut i)),
            Token::Else => panic!(
                "Unmatched else i:{}, ast:{:#?}, tokens: {:#?}",
                i, ast, tokens
            ),
            Token::While => ast.extend(parse_while(&tokens, &mut i)),
            Token::Assignation(a) => {
                let mut assignation = a.split("=");
                ast.push(ParseNode::Assignation(
                    assignation.next().unwrap().trim(),
                    assignation.next().unwrap().trim(),
                ))
            }
            Token::OpenCBrackets => panic!(
                "Unmatched {{ i:{}, ast: {:#?}, tokens: {:#?}",
                i, ast, tokens
            ),
            Token::CloseCBrackets => panic!(
                "Unmatched }} i:{}, ast: {:#?}, tokens: {:#?}",
                i, ast, tokens
            ),

            Token::Expression(exp) => ast.push(ParseNode::Expression(exp)),
            Token::Print => ast.push(parse_print(&tokens, &mut i)),
        }
        i += 1;
    }
    ast
}

fn execute_if(
    expr: &str,
    if_block: &[ParseNode],
    else_block: &Option<Vec<ParseNode>>,
    mut env: Eval,
) -> Eval {
    match env.eval(expr).unwrap() {
        Value::Bool(true) => {
            env = execute(if_block, env);
        }
        Value::Bool(false) => match else_block {
            Some(e) => {
                env = execute(e, env);
            }
            None => {}
        },
        _ => panic!("if statement only works with booleans"),
    }
    env
}

fn execute_while(expr: &str, block: &[ParseNode], mut env: Eval) -> Eval {
    while env.eval(expr).unwrap() == Value::Bool(true) {
        env = execute(block, env)
    }
    env
}

fn value_to_string(val: Value) -> String {
    match val {
        Value::Bool(b) => b.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Str(s) => s,
        Value::Range(r) => format!("{:?}", r),
        Value::Vec(v) => format!("{:?}", v),
        Value::None => "None".to_string(),
    }
}

fn execute_assignation(variable: &str, value: &str, env: Eval) -> Eval {
    let b = value_to_string(env.eval(value).unwrap());
    env.insert(variable, &b).unwrap()
}

fn execute_print(expression: &Box<ParseNode>, env: Eval) -> Eval {
    match expression.as_ref() {
        ParseNode::Expression(expr) => {
            println!("{:?}", value_to_string(env.eval(expr).unwrap()));
        }
        _ => panic!("Only expressions can be printed"),
    }
    env
}

fn execute(ast: &[ParseNode], mut env: Eval) -> Eval {
    let mut i: usize = 0;
    while i < ast.len() {
        //dbg!(&ast[i]);
        match &ast[i] {
            ParseNode::If(expr, if_block, else_block) => {
                env = execute_if(expr, if_block, else_block, env)
            }
            ParseNode::While(expr, block) => env = execute_while(expr, block, env),
            ParseNode::Assignation(variable, value) => {
                env = execute_assignation(variable, value, env);
            }
            ParseNode::Expression(expr) => panic!("Unused expression {}", expr),
            ParseNode::Print(expression) => env = execute_print(expression, env),
        }
        i += 1;
    }
    env
}

fn main() {
    let env = Eval::default();
    let filename = env::args().nth(1).expect("falta argumento");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let instructions = tokenizer(&contents);
    //let instructions = dbg!(instructions);
    let ast = parse(&instructions);
    // dbg!(&ast);
    execute(&ast, env);
}
