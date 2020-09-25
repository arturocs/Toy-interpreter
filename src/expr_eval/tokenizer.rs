use regex::Regex;
#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Token<'a> {
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
    Gt,
    Lt,
    Gtoe,
    Ltoe,
    And,
    Or,
    Not,
    Comma,
}

fn check_var_and_f64(capture: &str) -> Result<Token, &'static str> {
    lazy_static! {
        static ref VAR_REGEX: Regex = Regex::new(r"[^\{\}\n=\(\)]").unwrap();
    }
    let is_f64 = capture.parse::<f64>();
    if is_f64.is_ok() {
        Ok(Token::Number(is_f64.unwrap()))
    } else if VAR_REGEX.is_match(capture) {
        Ok(Token::VarName(capture))
    } else {
        Err("Unable to match expression")
    }
}

pub(crate) fn tokenize(expr: &str) -> Result<Vec<Token>, &'static str> {
    lazy_static! {
        static ref PATTERNS : String = [
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
            r"\-",             //Subtraction operator
            r"==",             //Equality operator
            r"!=",             //Not equal operator
            r"<=",             //Less than or equal to operator
            r">=",             //Greater than or equal to operator
            r"<",             //Less than operator
            r">",             //Greater than operator
            r"&&",             //Logical and operator
            r"\|\|",             //Logical or operator
            r",",             //Comma operator
            r"[^\{\}\n=\(\)]", //Variable
        ]
        .join("|");
        static ref EXPR_REGEX: Regex = Regex::new(&PATTERNS).unwrap();
    }
    EXPR_REGEX
        .captures_iter(expr)
        .map(|cap| cap.get(0).unwrap().as_str().trim())
        .filter(|s| s != &"")
        .map(|capture| match capture {
            "true" => Ok(Token::Bool(true)),
            "false" => Ok(Token::Bool(false)),
            "(" => Ok(Token::OpenParentheses),
            ")" => Ok(Token::CloseParentheses),
            //"[" => Ok(ExprToken::OpenSBrackets),
            //"]" => Ok(ExprToken::CloseSBrackets),
            "*" => Ok(Token::Mul),
            "+" => Ok(Token::Add),
            "-" => Ok(Token::Sub),
            "==" => Ok(Token::Eq),
            "!=" => Ok(Token::NotEq),
            "&&" => Ok(Token::And),
            "||" => Ok(Token::Or),
            "," => Ok(Token::Comma),
            "!" => Ok(Token::Not),
            "/" => Ok(Token::Div),
            "%" => Ok(Token::Rem),
            "<=" => Ok(Token::Ltoe),
            ">=" => Ok(Token::Gtoe),
            "<" => Ok(Token::Lt),
            ">" => Ok(Token::Gt),
            _ => check_var_and_f64(capture),
        })
        .collect()
}
