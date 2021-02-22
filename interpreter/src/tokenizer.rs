use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    If,
    Else,
    While,
    Assignation(&'a str),
    OpenCBrackets,
    CloseCBrackets,
    Expression(&'a str),
    Print,
}

fn is_assignation(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^\{\}\n=]+\s*=\s*[^\{\}\n=]+").unwrap();
    }
    RE.is_match(text)
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let patterns = [
        r"\s*if\s+",                      //if
        r"\s*else\s+",                    //else
        r"\s*while\s+",                   //while
        r"\s*print\s+",                   //print
        r"\{|\}",                         //Curly brackets
        r"[^\{\}\n=]+\s*=\s*[^\{\}\n=]+", //Assignation
        r"[^\{\}\n]+",                    //Everything else
    ]
    .join("|");

    RegexBuilder::new(&patterns)
        //.unicode(false)
        .build()
        .unwrap()
        .find_iter(source_code)
        .map(|m| source_code[m.start()..m.end()].trim())
        .filter(|&s| !s.is_empty())
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
