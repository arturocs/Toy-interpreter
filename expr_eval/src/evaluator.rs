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

    pub fn execute<'a>(&mut self, node: &ParseNode) -> Result<Val, Error> {
        match node {
            ParseNode::VarName(a) => self.get(a),
            ParseNode::Number(n) => Ok(n.clone()),
            ParseNode::String(s) => Ok(s.clone()),
            ParseNode::Bool(b) => Ok(b.clone()),
            ParseNode::Neg(n) => Ok(self.execute(&n)?.minus()?),
            ParseNode::Mul(s) => self.execute(&s[0])?.mul(self.execute(&s[1])?),
            ParseNode::Div(s) => self.execute(&s[0])?.div(self.execute(&s[1])?),
            ParseNode::Rem(s) => self.execute(&s[0])?.rem(self.execute(&s[1])?),
            ParseNode::Add(s) => self.execute(&s[0])?.add(self.execute(&s[1])?),
            ParseNode::Sub(s) => self.execute(&s[0])?.sub(self.execute(&s[1])?),
            ParseNode::Eq(s) => Ok(Val::Bool(self.execute(&s[0])?.eq(&self.execute(&s[1])?))),
            ParseNode::NotEq(s) => Ok(Val::Bool(self.execute(&s[0])?.ne(&self.execute(&s[1])?))),
            ParseNode::And(s) => self.execute(&s[0])?.and(self.execute(&s[1])?),
            ParseNode::Or(s) => self.execute(&s[0])?.or(self.execute(&s[1])?),
            ParseNode::Not(b) => Ok(self.execute(&b)?.not()?),
            ParseNode::Gt(s) => Ok(Val::Bool(self.execute(&s[0])? > self.execute(&s[1])?)),
            ParseNode::Lt(s) => Ok(Val::Bool(self.execute(&s[0])? < self.execute(&s[1])?)),
            ParseNode::Gtoe(s) => Ok(Val::Bool(self.execute(&s[0])? >= self.execute(&s[1])?)),
            ParseNode::Ltoe(s) => Ok(Val::Bool(self.execute(&s[0])? <= self.execute(&s[1])?)),
        }
    }
}
