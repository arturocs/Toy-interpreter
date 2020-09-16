use regex::{Regex, RegexBuilder};

#[derive(PartialEq, Debug)]
pub(crate) enum Token<'a> {
    If,
    Else,
    While,
    Assignation(&'a str),
    OpenCBrackets,
    CloseCBrackets,
    Expression(&'a str),
    Print,
}

pub(crate) fn tokenize(source_code: &str) -> Vec<Token> {
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

fn is_assignation(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^\{\}\n=]+\s*=\s*[^\{\}\n=]+").unwrap();
    }
    RE.is_match(text)
}
