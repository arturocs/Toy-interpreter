use crate::parser::{check_result, ParseNode};
use v_eval::{Eval, Value};

fn execute_if(
    expr: &str,
    if_block: &[ParseNode],
    else_block: &Option<Vec<ParseNode>>,
    mut env: Eval,
) -> Result<Eval, &'static str> {
    let mut error = "";
    match env.eval(expr).ok_or("Error evaluating if expression")? {
        Value::Bool(true) => {
            env = execute(if_block, env)?;
        }
        Value::Bool(false) => match else_block {
            Some(e) => {
                env = execute(e, env)?;
            }
            None => {}
        },
        _ => error = "if statement only works with booleans",
    }
    check_result(error == "", env, error)
}

fn execute_while(expr: &str, block: &[ParseNode], mut env: Eval) -> Result<Eval, &'static str> {
    while env.eval(expr).ok_or("Error evaluating while expression")? == Value::Bool(true) {
        env = execute(block, env)?
    }
    Ok(env)
}

fn value_vec_to_string(vector: Vec<Value>) -> String {
    "[".to_string()
        + &vector
            .into_iter()
            .map(value_to_string)
            .collect::<Vec<_>>()
            .join(", ")
        + "]"
}

fn value_to_string(val: Value) -> String {
    match val {
        Value::Bool(b) => b.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Str(s) => s,
        Value::Range(r) => format!("{:?}", r),
        Value::Vec(v) => value_vec_to_string(v),
        Value::None => "None".to_string(),
    }
}

fn execute_assignation(variable: &str, value: &str, env: Eval) -> Result<Eval, &'static str> {
    let b = value_to_string(env.eval(value).ok_or("Error evaluating expression")?);
    Ok(env
        .insert(variable, &b)
        .map_err(|_| "Error assigning variable")?)
}

fn execute_print(expression: &ParseNode, env: Eval) -> Result<Eval, &'static str> {
    let mut error = "";
    match expression {
        ParseNode::Expression(expr) => {
            println!(
                "{}",
                value_to_string(env.eval(expr).ok_or("Error evaluating print expression")?)
            );
        }
        _ => error = "Only expressions can be printed",
    }
    check_result(error == "", env, error)
}

fn execute_expression(expr: &str, env: Eval) -> Result<Eval, &'static str> {
    env.eval(expr).ok_or("Error evaluating expression")?;
    Ok(env)
}

pub(crate) fn execute(ast: &[ParseNode], mut env: Eval) -> Result<Eval, &'static str> {
    let mut i: usize = 0;
    while i < ast.len() {
        //dbg!(&ast[i]);
        match &ast[i] {
            ParseNode::If(expr, if_block, else_block) => {
                env = execute_if(expr, if_block, else_block, env)?
            }
            ParseNode::While(expr, block) => env = execute_while(expr, block, env)?,
            ParseNode::Assignation(variable, value) => {
                env = execute_assignation(variable, value, env)?;
            }
            ParseNode::Expression(expr) => env = execute_expression(expr, env)?,
            ParseNode::Print(expression) => env = execute_print(expression, env)?,
        }
        i += 1;
    }
    Ok(env)
}
