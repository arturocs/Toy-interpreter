use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub enum ExprToken {
    VarName(String),
    Number(f64),
    String(String),
    Bool(bool),
    Null,
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
    //Comma,
}

fn check_remaining_cases(capture: &str) -> Result<ExprToken, &'static str> {
    lazy_static! {
        static ref VAR_REGEX: Regex = Regex::new(r"[^\{\}\n=\(\)]").unwrap();
        static ref VEC_ACCESS_REGEX: Regex = Regex::new(r"\w+[").unwrap();
    }
    let is_f64 = capture.parse::<f64>();
    if is_f64.is_ok() {
        Ok(ExprToken::Number(is_f64.unwrap()))
    } else if capture.starts_with('"') && capture.ends_with('"') {
        Ok(ExprToken::String(capture.to_owned()))
    } else if VAR_REGEX.is_match(capture) {
        Ok(ExprToken::VarName(capture.to_owned()))
    } else if VEC_ACCESS_REGEX.is_match(capture) {
        Ok(ExprToken::VecAccessStart(capture.to_owned()))
    } else {
        Err("Unable to match expression")
    }
}

pub fn tokenize_expr(expr: &str) -> Result<Vec<ExprToken>, &'static str> {
    lazy_static! {
        static ref PATTERNS : String = [
            r"\d+\.?\d*",             //Number

            r"\s*true\s*|\s*true\s*", //Bool
            r"[^\w\d]null[^\w\d]",  //Null
            //r"[^\{\}\n=]\(",        //Starting part of a function call
            r"\w+\[",        //Starting part of a vector access
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
            //r",",             //Comma operator
            r#""[^"\n]*""#,                //String
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
            "true" => Ok(ExprToken::Bool(true)),
            "false" => Ok(ExprToken::Bool(false)),
            "null" => Ok(ExprToken::Null),
            "(" => Ok(ExprToken::OpenParentheses),
            ")" => Ok(ExprToken::CloseParentheses),
            "[" => Ok(ExprToken::OpenSBrackets),
            "]" => Ok(ExprToken::CloseSBrackets),
            "*" => Ok(ExprToken::Mul),
            "+" => Ok(ExprToken::Add),
            "-" => Ok(ExprToken::Sub),
            "==" => Ok(ExprToken::Eq),
            "!=" => Ok(ExprToken::NotEq),
            "&&" => Ok(ExprToken::And),
            "||" => Ok(ExprToken::Or),
            "," => Ok(ExprToken::Comma),
            "!" => Ok(ExprToken::Not),
            "/" => Ok(ExprToken::Div),
            "%" => Ok(ExprToken::Rem),
            "<=" => Ok(ExprToken::Ltoe),
            ">=" => Ok(ExprToken::Gtoe),
            "<" => Ok(ExprToken::Lt),
            ">" => Ok(ExprToken::Gt),
            _ => check_remaining_cases(capture),
        })
        .collect()
}
