use crate::parser::ParseNode;

use expr_eval::{self, evaluator::Environment, parser::ParseExprNode, val::Val};

fn execute_if(
    expr: &expr_eval::parser::ParseExprNode,
    if_block: &[ParseNode],
    else_block: &Option<Vec<ParseNode>>,
    env: &mut Environment,
) -> Result<(), &'static str> {
    match env.evaluate(&expr) {
        Ok(Val::Bool(true)) => execute(if_block, env),
        Ok(Val::Bool(false)) => match else_block {
            Some(e) => execute(e, env),
            None => Ok(()),
        },
        _ => Err("if statement only works with booleans"),
    }
}

fn execute_while(
    expr: &expr_eval::parser::ParseExprNode,
    block: &[ParseNode],
    env: &mut Environment,
) -> Result<(), &'static str> {
    while env.evaluate(&expr)? == Val::Bool(true) {
        execute(block, env)?
    }
    Ok(())
}

fn execute_assignation(
    variable: &str,
    value: &expr_eval::parser::ParseExprNode,
    env: &mut Environment,
) -> Result<(), &'static str> {
    let computed_value = env.evaluate(&value)?;
    let varname = variable.to_owned();
    env.insert(varname, computed_value);
    Ok(())
}

fn execute_print(expression: &ParseNode, env: &mut Environment) -> Result<(), &'static str> {
    match expression {
        ParseNode::Expression(expr) => {
            println!("{}", env.evaluate(expr)?);
            Ok(())
        }
        _ => Err("Only expressions can be printed"),
    }
}

fn execute_expression(expr: &ParseExprNode, env: &mut Environment) -> Result<(), &'static str> {
    env.evaluate(&expr)?;
    Ok(())
}

fn execute_vector_write(
    name: &str,
    index: &[ParseExprNode],
    value: ParseExprNode,
    env: &mut Environment,
) -> Result<(), &'static str> {
    let computed_value = env.evaluate(&value)?;
    let mut computed_indexes = index
        .iter()
        .map(|n| env.evaluate(n))
        .collect::<Result<Vec<_>, _>>()?;
    let mut a = env.get_mut_ref(name)?;
    let last_index = computed_indexes.pop().ok_or("Empty index")?;
    for i in computed_indexes {
        a = a.index(i)?
    }
    a.write_to_vec(last_index, computed_value)?;
    Ok(())
}

pub fn execute(ast: &[ParseNode], mut env: &mut Environment) -> Result<(), &'static str> {
    let mut i: usize = 0;

    while i < ast.len() {
        match &ast[i] {
            ParseNode::If(expr, if_block, else_block) => match expr.as_ref() {
                ParseNode::Expression(e) => execute_if(e, if_block, else_block, env)?,
                _ => return Err("Error parsing if expression"),
            },
            ParseNode::While(expr, block) => match expr.as_ref() {
                ParseNode::Expression(e) => execute_while(e, block, env)?,
                _ => return Err("Error parsing while expression"),
            },
            ParseNode::Assignation(variable, value) => {
                execute_assignation(variable, &value, &mut env)?
            }
            ParseNode::Expression(expr) => execute_expression(expr, env)?,
            ParseNode::Print(expression) => execute_print(expression, env)?,
            ParseNode::VecWrite(name, index, value) => {
                execute_vector_write(name, index, *value.clone(), env)?
            }
        }
        i += 1;
    }

    Ok(())
}
