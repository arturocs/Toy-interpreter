use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
#[macro_use]
extern crate lazy_static;
#[derive(PartialEq, Debug)]
enum VarType<'a> {
    Number(f64),
    Bool(bool),
    String(&'a str),
    Null,
    Vector(Vec<VarType<'a>>),
}
struct Expression<'a> {
    first: VarType<'a>,
    second: VarType<'a>,
    opperator: &'a str,
}

impl 

impl Expression {
    fn evaluate(&self) -> VarType {
        match self.opperator {
            "==" => VarType::Bool(self.first == self.second),
            "!=" => VarType::Bool(self.first != self.second),
            "!" => VarType::Bool(!self.first),
            "<" => VarType::Bool(self.first < self.second),
            ">" => VarType::Bool(self.first > self.second),
            "+" => VarType::Number(self.first + self.second),
            "-" => VarType::Number(self.first - self.second),
            "*" => VarType::Number(self.first * self.second),
            x => panic!("Unrecognized operator {}", x),
        }
    }
}
enum Thing {}
#[derive(PartialEq, Debug)]
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
    Comma,
    Number(f64),
    String(&'a str),
    NewLine,
    Print,
}
fn is_string(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#""(\w*\s*\d*)*""#).unwrap();
    }
    RE.is_match(text)
}

fn is_a_variable(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\w+").unwrap();
    }
    RE.is_match(text)
}

fn tokenizer(source_code: &str) -> Vec<Token> {
    let re = Regex::new(
        r#"\n|var\s|if\s|else\s|while\s|==|!=|!|=|<|>|\+|-|\*|\{|\}|[|]|\(|\)|\strue\s|\sfalse\s|,|-?\d+.?\d*|"(\w*\s*\d*)*"|\w+"#,
    )
    .unwrap();
    re.captures_iter(source_code)
        .map(|cap| {
            cap.get(0)
                .unwrap()
                .as_str()
                .trim_matches(|c| c == ' ' || c == '\t')
        })
        .map(|cap| match cap {
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
            "print" => Token::Print,
            "\n" => Token::NewLine,
            x if x.parse::<f64>().is_ok() => Token::Number(x.parse::<f64>().unwrap()),
            x if is_string(x) => Token::String(x),
            x if is_a_variable(x) => Token::VarName(x),
            unknown => Token::Unknown(unknown),
        })
        .collect()
}

fn parse(instructions: Vec<Token>) -> Vec<Thing> {
    //let variables: HashMap<&str, VarType> = HashMap::new();
    let instruction_iter = instructions.into_iter().peekable();
    while instruction_iter.peek().is_some() {
        match instruction_iter
            .next()
            .expect(&format!("Something is very wrong at line {}", line!()))
        {
            Token::VarDeclaration => {}
            Token::If => {}
            Token::Else => {}
            Token::While => {}
            Token::OpAssignation => {}
            Token::OpEqual => {}
            Token::OpNotEqual => {}
            Token::OpNot => {}
            Token::OpLessThan => {}
            Token::OpGreaterThan => {}
            Token::OpAdd => {}
            Token::OpNeg => {}
            Token::OpMul => {}
            Token::OpenCBrackets => {}
            Token::CloseCBrackets => {}
            Token::OpenBrackets => {}
            Token::CloseBrackets => {}
            Token::OpenParentheses => {}
            Token::CloseParentheses => {}
            Token::Unknown(s) => {
                eprint!("What is \"{}\"?", s);
            }
            Token::VarName(s) => {}
            Token::True => {}
            Token::False => {}
            Token::Comma => {}
            Token::Number(n) => {}
            Token::String(s) => {}
            Token::NewLine => {}
            Token::Print => {}
        }
    }
}

fn execute(ast: Vec<Thing>) {}

fn main() {
    let filename = env::args().nth(1).expect("falta argumento");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let instructions = tokenizer(&contents);
    dbg!(instructions);
    // execute(instructions);
}
