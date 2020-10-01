use std::{cmp::Ordering, fmt, str::FromStr};
type Error = &'static str;

#[derive(PartialEq, Debug, Clone)]
pub enum Val {
    Bool(bool),
    Number(f64),
    Str(String),
    Vec(Vec<Val>),
    Null,
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Val) -> Option<Ordering> {
        match (self, other) {
            (Val::Number(a), Val::Number(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl FromStr for Val {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s.trim() {
            "true" => Ok(Val::Bool(true)),
            "false" => Ok(Val::Bool(false)),
            "Null" => Ok(Val::Null),
            st if st.starts_with('"') && st.ends_with('"') => Ok(Val::Str(st.to_string())),
            _ => Err("Error while parsing Value"),
        };
        parse_f64_and_vec(s, res)
    }
}

fn parse_f64_and_vec(s: &str, res: Result<Val, Error>) -> Result<Val, Error> {
    if res.is_err() {
        match s.parse::<f64>().map_err(|_| "Error parsing number") {
            Ok(n) => Ok(Val::Number(n)),
            Err(_) => match s {
                st if st.starts_with('[') && s.ends_with(']') => to_vec(st),
                _ => res,
            },
        }
    } else {
        res
    }
}

fn to_vec(s: &str) -> Result<Val, Error> {
    if s.starts_with('[') && s.ends_with(']') {
        Ok(Val::Vec(
            s.trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .map(|v| Ok(v.parse::<Val>()?))
                .collect::<Result<Vec<Val>, Error>>()?,
        ))
    } else {
        Err("Error while parsing Vector")
    }
}
impl fmt::Display for Val {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            Val::Bool(b) => b.to_string(),
            Val::Number(f) => f.to_string(),
            Val::Str(s) => s.clone(),
            Val::Vec(v) => {
                "[".to_string()
                    + &v.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                    + "]"
            }
            Val::Null => "Null".to_string(),
        };
        write!(fmt, "{}", val)
    }
}

impl Val {
    pub fn add(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            (Val::Number(a), Val::Number(b)) => Ok(Val::Number(a + b)),
            (Val::Str(a), Val::Str(b)) => Ok(Val::Str(a + &b)),
            (Val::Str(a), b) => Ok(Val::Str(a + &b.to_string())),
            (Val::Vec(mut a), Val::Vec(b)) => {
                a.extend(b);
                Ok(Val::Vec(a))
            }
            _ => Err("Only numbers, strings and vectors can be added"),
        }
    }
    pub fn sub(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            (Val::Number(a), Val::Number(b)) => Ok(Val::Number(a - b)),
            _ => Err("Only numbers can be subtracted"),
        }
    }

    pub fn mul(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            (Val::Number(a), Val::Number(b)) => Ok(Val::Number(a * b)),
            _ => Err("Only numbers can be multiplied"),
        }
    }

    pub fn div(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            (Val::Number(a), Val::Number(b)) => Ok(Val::Number(a / b)),
            _ => Err("Only numbers can be divided"),
        }
    }

    pub fn rem(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            (Val::Number(a), Val::Number(b)) => Ok(Val::Number(a % b)),
            _ => Err("Remainder operator can only be applied to numbers"),
        }
    }

    pub fn not(self) -> Result<Self, Error> {
        match self {
            Val::Bool(a) => Ok(Val::Bool(!a)),
            _ => Err("Not operator can only be applied to booleans"),
        }
    }
    pub fn minus(self) -> Result<Self, Error> {
        match self {
            Val::Number(a) => Ok(Val::Number(-a)),
            _ => Err("Unary minus can only be applied to numbers"),
        }
    }

    pub fn index(&self, i: usize) -> Result<&Self, Error> {
        match self {
            Val::Vec(v) => Ok(&v[i]),
            _ => Err("Index operator can only be applied to vectors"),
        }
    }

    pub fn and(&self, other: Val) -> Result<Self, Error> {
        match (self, other) {
            (Val::Bool(a), Val::Bool(b)) => Ok(Val::Bool(*a && b)),
            _ => Err("Logical and can only be applied to booleans"),
        }
    }

    pub fn or(&self, other: Val) -> Result<Self, Error> {
        match (self, other) {
            (Val::Bool(a), Val::Bool(b)) => Ok(Val::Bool(*a || b)),
            _ => Err("Logical or can only be applied to booleans"),
        }
    }
    pub fn push(self, element: Val) -> Result<Self, Error> {
        match self {
            Val::Vec(mut v) => {
                v.push(element);
                Ok(Val::Vec(v))
            }
            _ => Err("push() can only be used on vectors"),
        }
    }
    pub fn pop(mut self) -> Result<(Val, Val), Error> {
        match self {
            Val::Vec(ref mut v) => match v.pop() {
                Some(e) => Ok((e, self)),
                None => Err("The vector is empty"),
            },
            _ => Err("pop() can only be used on vectors"),
        }
    }
}
