use std::collections::HashMap;
use std::env;
use std::fs;

enum VarType<'a> {
    Number(f64),
    Bool(bool),
    String(&'a str),
    Null,
    Vector(Vec<VarType<'a>>),
}

#[derive(PartialEq)]
enum Token<'a> {
    VarDeclaration,
    If,
    Else,
    While,
    OpAssignation,
    OpEqual,
    OpNotEqual,
    OpNot,
    OpLessThan,
    OpGreaterThan,
    OpAdd,
    OpNeg,
    OpMul,
    OpenCBrackets,
    CloseCBrackets,
    OpenBrackets,
    CloseBrackets,
    OpenParentheses,
    CloseParentheses,
    Unknown(&'a str),
    VarName(&'a str),
    True,
    False,
}

fn process_unknown(unknown: &str) -> Vec<Token> {
    unimplemented!("string desconocido: {}",unknown)
}

fn parse(source_code: &str) -> Vec<Token>{
    source_code.split_whitespace().map(|word| match word {
        "var" => Token::VarDeclaration,
        "if" => Token::If,
        "else" => Token::Else,
        "while" => Token::While,
        "=" => Token::OpAssignation,
        "==" => Token::OpEqual,
        "!=" => Token::OpNotEqual,
        "!" => Token::OpNot,
        "<" => Token::OpLessThan,
        ">" => Token::OpGreaterThan,
        "+" => Token::OpAdd,
        "-" => Token::OpNeg,
        "*" => Token::OpMul,
        "{" => Token::OpenCBrackets,
        "}" => Token::CloseCBrackets,
        "[" => Token::OpenBrackets,
        "]" => Token::CloseBrackets,
        "(" => Token::OpenParentheses,
        ")" => Token::CloseParentheses,
        "true" => Token::True,
        "false" => Token::False,
        unknown => Token::Unknown(unknown),
    }).flat_map(|token| match token {
        Token::Unknown(unknown_sequence) => process_unknown(unknown_sequence).into_iter(),
        other => vec![other].into_iter(),
    }).collect()
}

fn execute(instructions:Vec<Token>){
    let variables: HashMap<&str, VarType> = HashMap::new();
}

fn main() {
    let filename = env::args().nth(1).expect("falta argumento");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let instructions = parse(&contents);
    execute(instructions);
}
