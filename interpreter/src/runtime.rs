use crate::parser::ParseNode;

use expr_eval::{self, evaluator, val::Val};

fn execute_if(
    expr: &expr_eval::parser::ParseNode,
    if_block: &[ParseNode],
    else_block: &Option<Vec<ParseNode>>,
    env: &mut evaluator::Environment,
) -> Result<(), &'static str> {
    match env.execute(&expr) {
        Ok(Val::Bool(true)) => execute(if_block, env),
        Ok(Val::Bool(false)) => match else_block {
            Some(e) => execute(e, env),
            None => Ok(()),
        },
        _ => Err("if statement only works with booleans"),
    }
}

fn execute_while(
    expr: &expr_eval::parser::ParseNode,
    block: &[ParseNode],
    env: &mut evaluator::Environment,
) -> Result<(), &'static str> {
    while env.execute(&expr)? == Val::Bool(true) {
        execute(block, env)?
    }
    Ok(())
}

fn execute_assignation(
    variable: &str,
    value: &expr_eval::parser::ParseNode,
    env: &mut evaluator::Environment,
) -> Result<(), &'static str> {
    let computed_value = env.execute(&value)?;
    let varname = variable.to_owned();
    env.insert(varname, computed_value);
    Ok(())
}

fn execute_print(expression: &ParseNode, env: &mut evaluator::Environment) -> Result<(), &'static str> {
    match expression {
        ParseNode::Expression(expr) => {
            println!("{}", env.execute(expr)?);
            Ok(())
        }
        _ => Err("Only expressions can be printed"),
    }
}

fn execute_expression(
    expr: &expr_eval::parser::ParseNode,
    env: &mut evaluator::Environment,
) -> Result<(), &'static str> {
    env.execute(&expr)?;
    Ok(())
}

pub fn execute(
    ast: &[ParseNode],
    mut env: &mut evaluator::Environment,
) -> Result<(), &'static str> {
    let mut i: usize = 0;
    let mut error = "";
    while i < ast.len() {
        match &ast[i] {
            ParseNode::If(expr, if_block, else_block) => match expr.as_ref() {
                ParseNode::Expression(e) => execute_if(e, if_block, else_block, env)?,
                _ => {
                    error = "Error parsing if expression";
                    break;
                }
            },
            ParseNode::While(expr, block) => match expr.as_ref() {
                ParseNode::Expression(e) => execute_while(e, block, env)?,
                _ => {
                    error = "Error parsing while expression";
                    break;
                }
            },
            ParseNode::Assignation(variable, value) => match value.as_ref() {
                ParseNode::Expression(e) => execute_assignation(variable, e, &mut env)?,
                _ => {
                    error = "Error parsing assignation expression";
                    break;
                }
            },
            ParseNode::Expression(expr) => execute_expression(expr, env)?,
            ParseNode::Print(expression) => execute_print(expression, env)?,
        }
        i += 1;
    }
    if error == "" {
        Ok(())
    } else {
        Err(error)
    }
}
