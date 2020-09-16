use std::ops::{Add, Div, Index, Mul, Not, Rem, Sub};
use std::{cmp::Ordering, fmt, str::FromStr};
#[derive(PartialEq, Debug)]
pub(crate) enum Val {
    Bool(bool),
    Number(f64),
    Str(String),
    Vec(Vec<Val>),
    Null,
}

impl Add for Val {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Val::Number(a), Val::Number(b)) => Val::Number(a + b),
            (Val::Str(a), Val::Str(b)) => Val::Str(a + &b),
            (Val::Vec(mut a), Val::Vec(b)) => {
                a.extend(b);
                Val::Vec(a)
            }
            _ => Val::Null,
        }
    }
}

impl Sub for Val {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Val::Number(a), Val::Number(b)) => Val::Number(a - b),
            _ => Val::Null,
        }
    }
}

impl Mul for Val {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Val::Number(a), Val::Number(b)) => Val::Number(a * b),
            _ => Val::Null,
        }
    }
}

impl Div for Val {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Val::Number(a), Val::Number(b)) => Val::Number(a / b),
            _ => Val::Null,
        }
    }
}
impl Rem for Val {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        match (self, other) {
            (Val::Number(a), Val::Number(b)) => Val::Number(a % b),
            _ => Val::Null,
        }
    }
}

impl Not for Val {
    type Output = Val;
    fn not(self) -> Self::Output {
        match self {
            Val::Bool(a) => Val::Bool(!a),
            _ => Val::Null,
        }
    }
}


impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Val) -> Option<Ordering> {
        match (self, other) {
            (Val::Number(a), Val::Number(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl Index<usize> for Val {
    type Output = Self;
    fn index(&self, i: usize) -> &Self::Output {
        match self {
            Val::Vec(v) => &v[i],
            _ => &Val::Null,
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

fn parse_f64_and_vec(s: &str, res: Result<Val, &'static str>) -> Result<Val, &'static str> {
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

fn to_vec(s: &str) -> Result<Val, &'static str> {
    if s.starts_with('[') && s.ends_with(']') {
        Ok(Val::Vec(
            s.trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .map(|v| Ok(v.parse::<Val>()?))
                .collect::<Result<Vec<Val>, &'static str>>()?,
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
    fn and(&self, other: Val) -> Val {
        match (self, other) {
            (Val::Bool(a), Val::Bool(b)) => Val::Bool(*a && b),
            _ => Val::Null,
        }
    }
    fn or(&self, other: Val) -> Val {
        match (self, other) {
            (Val::Bool(a), Val::Bool(b)) => Val::Bool(*a || b),
            _ => Val::Null,
        }
    }
}
