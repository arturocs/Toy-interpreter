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
