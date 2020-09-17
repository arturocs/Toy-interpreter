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

fn is_assignation(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^\{\}\n=]+\s*=\s*[^\{\}\n=]+").unwrap();
    }
    RE.is_match(text)
}

pub(crate) fn tokenize(source_code: &str) -> Vec<Token> {
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
#[derive(PartialEq, Debug)]
pub(crate) enum ExprToken<'a> {
    VarName(&'a str),
    Number(f64),
    String(&'a str),
    Bool(bool),
    //FnCallStart(&'a str),
    //VecAccessStart(&'a str),
    //Dot
    //OpenSBrackets,
    //CloseSBrackets,
    OpenParentheses,
    CloseParentheses,
    Mul,
    Div,
    Rem,
    Add,
    Sub,
    Eq,
    NotEq,
    And,
    Or,
    Not,
    Comma,
}
pub(crate) fn tokenize_expression(expr: &str) -> Result<Vec<ExprToken>, &'static str> {
    lazy_static! {
        static ref patterns : String = [
            r"\d+\.?\d*",             //Number
            r#"".*""#,                //String
            r"\s*true\s+|\s*true\s+", //Bool
            //r"[^\{\}\n=]\(",        //Starting part of a function call
            //r"[^\{\}\n=]\[",        //Starting part of a vector access
            //r"\."                   //Dot operator
            r"\(|\)",          // Parentheses
            r"\[|\]",          //Square brackets
            r"\*",             // Multiplication operator
            r"/",             // Division operator
            r"%",             //Remainder operator
            r"\+",             //Addition operator
            r"\-",             //Substaction operator
            r"==",             //Equality operator
            r"!=",             //Not equal operator
            r"&&",             //Logical and operator
            r"\|\|",             //Logical or operator
            r",",             //Comma operator
            r"[^\{\}\n=\(\)]", //Variable
        ]
        .join("|");
        static ref EXPR_REGEX: Regex = Regex::new(&patterns).unwrap();
        static ref VAR_REGEX: Regex = Regex::new(r"[^\{\}\n=\(\)]").unwrap();
    }
    EXPR_REGEX
        .captures_iter(expr)
        .map(|cap| cap.get(0).unwrap().as_str().trim())
        .filter(|s| s != &"")
        .map(|m| match m {
            "true" => Ok(ExprToken::Bool(true)),
            "false" => Ok(ExprToken::Bool(false)),
            "(" => Ok(ExprToken::OpenParentheses),
            ")" => Ok(ExprToken::CloseParentheses),
            //"[" => Ok(ExprToken::OpenSBrackets),
            //"]" => Ok(ExprToken::CloseSBrackets),
            "*" => Ok(ExprToken::Mul),
            "+" => Ok(ExprToken::Add),
            "-" => Ok(ExprToken::Sub),
            "==" => Ok(ExprToken::Eq),
            "!=" => Ok(ExprToken::NotEq),
            "&&" => Ok(ExprToken::And),
            "||" => Ok(ExprToken::Or),
            "," => Ok(ExprToken::Comma),
            num if num.parse::<f64>().is_ok() => Ok(ExprToken::Number(num.parse::<f64>().unwrap())),
            var if VAR_REGEX.is_match(var) => Ok(ExprToken::VarName(var)),
            _ => Err("Unable to match expression"),
        })
        .collect()
}
