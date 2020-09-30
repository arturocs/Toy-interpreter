use crate::{
    expr_eval::{self, evaluator, val::Val},
    parser::ParseNode,
};

fn execute_if(
    expr: &expr_eval::parser::ParseNode,
    if_block: &[ParseNode],
    else_block: &Option<Vec<ParseNode>>,
    env: evaluator::Environment,
) -> Result<evaluator::Environment, &'static str> {
    match env.execute(&expr)? {
        Val::Bool(true) => execute(if_block, env),
        Val::Bool(false) => match else_block {
            Some(e) => execute(e, env),
            None => Ok(env),
        },
        _ => Err("if statement only works with booleans"),
    }
}

fn execute_while(
    expr: &expr_eval::parser::ParseNode,
    block: &[ParseNode],
    mut env: evaluator::Environment,
) -> Result<evaluator::Environment, &'static str> {
    while env.execute(&expr)? == Val::Bool(true) {
        env = execute(block, env)?
    }
    Ok(env)
}

fn execute_assignation(
    variable: &str,
    value: &expr_eval::parser::ParseNode,
    mut env: evaluator::Environment,
) -> Result<evaluator::Environment, &'static str> {
    let b = env.execute(&value)?;
    let v = variable.to_owned();
    env.insert(v, b);
    Ok(env)
}

fn execute_print(
    expression: &ParseNode,
    env: evaluator::Environment,
) -> Result<evaluator::Environment, &'static str> {
    match expression {
        ParseNode::Expression(expr) => {
            println!("{}", env.execute(expr)?);
            Ok(env)
        }
        _ => Err("Only expressions can be printed"),
    }
}

fn execute_expression(
    expr: &expr_eval::parser::ParseNode,
    env: evaluator::Environment,
) -> Result<evaluator::Environment, &'static str> {
    env.execute(&expr)?;
    Ok(env)
}

pub(crate) fn execute(
    ast: &[ParseNode],
    mut env: evaluator::Environment,
) -> Result<evaluator::Environment, &'static str> {
    let mut i: usize = 0;
    while i < ast.len() {
        //dbg!(&ast[i]);
        match &ast[i] {
            ParseNode::If(expr, if_block, else_block) => {
                match expr.as_ref() {
                    ParseNode::Expression(e) => env = execute_if(e, if_block, else_block, env)?,
                    _ => panic!("bug1"),
                }
                // env = execute_if(*expr.as_ref(), if_block, else_block, env)?
            }
            ParseNode::While(expr, block) => match expr.as_ref() {
                ParseNode::Expression(e) => env = execute_while(e, block, env)?,
                _ => panic!("bug2"),
            },
            // env = execute_while(*expr.as_ref(), block, env)?},
            ParseNode::Assignation(variable, value) => {
                match value.as_ref() {
                    ParseNode::Expression(e) => env = execute_assignation(variable, e, env)?,
                    _ => panic!("bug3"),
                }
                // env = execute_assignation(variable, *value.as_ref(), env)?;
            }
            ParseNode::Expression(expr) => env = execute_expression(expr, env)?,
            ParseNode::Print(expression) => env = execute_print(expression, env)?,
        }
        i += 1;
    }
    Ok(env)
}
