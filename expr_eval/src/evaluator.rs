use crate::{parser::*, val::Val};
use std::collections::BTreeMap;

type Error = &'static str;

pub struct Environment {
    variables: BTreeMap<String, Val>,
    //functions : std::collections::HashMap<&str, _>
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            variables: BTreeMap::new(),
        }
    }
    /*pub(crate) fn with_capacity(capacity: usize) -> Environment {
        Environment {
            variables: BTreeMap::with_capacity(capacity),
        }
    }*/
    pub fn insert(&mut self, variable: String, value: Val) {
        &self.variables.insert(variable, value);
    }

    pub fn get(&mut self, key: &str) -> Result<Val, Error> {
        Ok(self
            .variables
            .get(key)
            .ok_or("Undeclared variable")?
            .clone())
    }

    pub fn execute<'a>(&mut self, node: &ParseExprNode) -> Result<Val, Error> {
        match node {
            ParseExprNode::VarName(a) => self.get(a),
            ParseExprNode::Number(n) => Ok(n.clone()),
            ParseExprNode::String(s) => Ok(s.clone()),
            ParseExprNode::Bool(b) => Ok(b.clone()),
            ParseExprNode::Null => Ok(Val::Null),
            ParseExprNode::Neg(n) => Ok(self.execute(&n)?.minus()?),
            ParseExprNode::Mul(s) => self.execute(&s[0])?.mul(self.execute(&s[1])?),
            ParseExprNode::Div(s) => self.execute(&s[0])?.div(self.execute(&s[1])?),
            ParseExprNode::Rem(s) => self.execute(&s[0])?.rem(self.execute(&s[1])?),
            ParseExprNode::Add(s) => self.execute(&s[0])?.add(self.execute(&s[1])?),
            ParseExprNode::Sub(s) => self.execute(&s[0])?.sub(self.execute(&s[1])?),
            ParseExprNode::Eq(s) => Ok(Val::Bool(self.execute(&s[0])?.eq(&self.execute(&s[1])?))),
            ParseExprNode::NotEq(s) => {
                Ok(Val::Bool(self.execute(&s[0])?.ne(&self.execute(&s[1])?)))
            }
            ParseExprNode::And(s) => self.execute(&s[0])?.and(self.execute(&s[1])?),
            ParseExprNode::Or(s) => self.execute(&s[0])?.or(self.execute(&s[1])?),
            ParseExprNode::Not(b) => Ok(self.execute(&b)?.not()?),
            ParseExprNode::Gt(s) => Ok(Val::Bool(self.execute(&s[0])? > self.execute(&s[1])?)),
            ParseExprNode::Lt(s) => Ok(Val::Bool(self.execute(&s[0])? < self.execute(&s[1])?)),
            ParseExprNode::Gtoe(s) => Ok(Val::Bool(self.execute(&s[0])? >= self.execute(&s[1])?)),
            ParseExprNode::Ltoe(s) => Ok(Val::Bool(self.execute(&s[0])? <= self.execute(&s[1])?)),
            ParseExprNode::VecAccess(_, _) => todo!(),
            ParseExprNode::Vector(v) => Ok(Val::Vec(
                v.into_iter()
                    .map(|n| self.execute(n))
                    .collect::<Result<Vec<_>, _>>()?,
            )),
        }
    }
}
