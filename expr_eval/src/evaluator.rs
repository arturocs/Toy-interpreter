use crate::{parser::*, val::Val};
use std::collections::BTreeMap;

type Error = &'static str;

#[derive(Debug, Default)]
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

    pub fn insert(&mut self, variable: String, value: Val) {
        self.variables.insert(variable, value);
    }

    pub fn get_mut_ref(&mut self, key: &str) -> Result<&mut Val, Error> {
        self.variables.get_mut(key).ok_or("Undeclared variable")
    }

    pub fn get_ref(&mut self, key: &str) -> Result<&Val, Error> {
        self.variables.get(key).ok_or("Undeclared variable")
    }

    fn execute_vec(&mut self, v: &[ParseExprNode]) -> Result<Val, Error> {
        Ok(Val::Vec(
            v.iter()
                .map(|n| self.evaluate(n))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    fn execute_vec_access(&mut self, name: &str, index: &[ParseExprNode]) -> Result<Val, Error> {
        let computed_indexes: Result<Vec<_>, _> = index.iter().map(|n| self.evaluate(n)).collect();
        let mut a = self.get_mut_ref(name)?;
        for i in computed_indexes? {
            a = a.index(i)?
        }
        Ok(a.clone())
    }

    pub fn evaluate(&mut self, node: &ParseExprNode) -> Result<Val, Error> {
        match node {
            ParseExprNode::VarName(a) => Ok(self.get_mut_ref(a)?.clone()),
            ParseExprNode::Number(n) => Ok(n.clone()),
            ParseExprNode::String(s) => Ok(s.clone()),
            ParseExprNode::Bool(b) => Ok(b.clone()),
            ParseExprNode::Null => Ok(Val::Null),
            ParseExprNode::VecAccess(name, index) => self.execute_vec_access(name, index),
            ParseExprNode::Vector(v) => self.execute_vec(v),
            ParseExprNode::Neg(n) => Ok(self.evaluate(&n)?.minus()?),
            ParseExprNode::Mul(s) => self.evaluate(&s[0])?.mul(self.evaluate(&s[1])?),
            ParseExprNode::Div(s) => self.evaluate(&s[0])?.div(self.evaluate(&s[1])?),
            ParseExprNode::Rem(s) => self.evaluate(&s[0])?.rem(self.evaluate(&s[1])?),
            ParseExprNode::Add(s) => self.evaluate(&s[0])?.add(self.evaluate(&s[1])?),
            ParseExprNode::Sub(s) => self.evaluate(&s[0])?.sub(self.evaluate(&s[1])?),
            ParseExprNode::Eq(s) => Ok(Val::Bool(self.evaluate(&s[0])?.eq(&self.evaluate(&s[1])?))),
            ParseExprNode::NotEq(s) => {
                Ok(Val::Bool(self.evaluate(&s[0])?.ne(&self.evaluate(&s[1])?)))
            }
            ParseExprNode::And(s) => self.evaluate(&s[0])?.and(self.evaluate(&s[1])?),
            ParseExprNode::Or(s) => self.evaluate(&s[0])?.or(self.evaluate(&s[1])?),
            ParseExprNode::Not(b) => Ok(self.evaluate(&b)?.not()?),
            ParseExprNode::Gt(s) => Ok(Val::Bool(self.evaluate(&s[0])? > self.evaluate(&s[1])?)),
            ParseExprNode::Lt(s) => Ok(Val::Bool(self.evaluate(&s[0])? < self.evaluate(&s[1])?)),
            ParseExprNode::Gtoe(s) => Ok(Val::Bool(self.evaluate(&s[0])? >= self.evaluate(&s[1])?)),
            ParseExprNode::Ltoe(s) => Ok(Val::Bool(self.evaluate(&s[0])? <= self.evaluate(&s[1])?)),
        }
    }
}
