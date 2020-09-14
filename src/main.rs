use regex::{Regex, RegexBuilder};
use std::collections::HashMap;
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
    Unknown(&'a str),
}

#[derive(PartialEq, Debug)]
enum ParseNode<'a> {
    If(&'a str, Vec<ParseNode<'a>>, Option<Vec<ParseNode<'a>>>),
    While(&'a str, Vec<ParseNode<'a>>),
    Assignation(&'a str, &'a str),
    OpenCBrackets,
    CloseCBrackets,
    Unknown(&'a str),
}

fn is_assignation(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\w+ *= *.+").unwrap();
    }
    RE.is_match(text)
}

fn tokenizer(source_code: &str) -> Vec<Token> {
    let patterns = [
        "if",
        "else",
        "while",
        r"\{",
        r"\}",
        r"\w+ *= *.+", //Assignation
        ".+",          //Everything else
    ]
    .join("|");

    RegexBuilder::new(&patterns)
        // .unicode(false)
        .build()
        .unwrap()
        .captures_iter(source_code)
        .map(|cap| cap.get(0).unwrap().as_str().trim())
        .map(|cap| match cap {
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "{" => Token::OpenCBrackets,
            "}" => Token::CloseCBrackets,
            a if is_assignation(a) => Token::Assignation(a),
            unknown => Token::Unknown(unknown),
        })
        .collect()
}

fn parse(instructions: Vec<Token>) -> Vec<ParseNode> {
    //let variables: HashMap<&str, VarType> = HashMap::new();
    let mut ast = vec![];
    let mut instruction_iter = instructions.into_iter().peekable();
    while instruction_iter.peek().is_some() {
        match instruction_iter
            .next()
            .expect(&format!("Something is very wrong at line {}", line!()))
        {
            Token::If => match instruction_iter.peek().unwrap() {
                Token::Unknown(a) => {
                    match instruction_iter
                        .skip(1)
                        .next()
                        .expect("Expected { after if expression")
                    {
                        //Wrong, need to check nested blocks
                        Token::OpenCBrackets => {
                            let mut body = vec![];
                            while *instruction_iter.peek().expect("Expected }")
                                != Token::CloseCBrackets
                            {
                                body.push(instruction_iter.next().unwrap());
                            }
                            ast.push(ParseNode::If(a, body, None));
                        }
                        
                        Token::If => {}
                        Token::Else => {}
                        Token::While => {}
                        Token::Assignation(_) => {}
                        Token::CloseCBrackets => {}
                        Token::Unknown(_) => {}
                    }
                }
                _ => panic!("Expected expression after if"),
            },
            Token::Else => {}
            Token::While => {}
            Token::Assignation(a) => {}
            Token::OpenCBrackets => {}
            Token::CloseCBrackets => {}

            Token::Unknown(s) => {
                eprint!("What is \"{}\"?", s);
            }
        }
    }
    unimplemented!()
}

fn main() {
    let mut e = Eval::default().insert("v", "1+3+5/3").unwrap();
    dbg!(e.eval("v").unwrap());
    //  e=e.insert("v", "[1,1]").unwrap();
    //assert_eq!(e.eval("v").unwrap(), Value::Int(1));
    let filename = env::args().nth(1).expect("falta argumento");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let instructions = tokenizer(&contents);
    dbg!(instructions);
    // execute(instructions);
}
