pub mod parser;
pub mod runtime;
pub mod tokenizer;

#[cfg(test)]
mod tests {
    use expr_eval::{evaluator::Environment, val::Val};

    use crate::{parser::parse, runtime, tokenizer::tokenize};

    #[test]
    fn while_loop() {
        let mut env = Environment::new();
        let code = 
        "a=0
        while a <10 {
            a = a + 1
        }";
        let instructions = tokenize(&code);
        let ast = parse(&instructions).unwrap();
        runtime::execute(&ast, &mut env).unwrap();
        assert_eq!(env.get_ref("a"), Ok(&Val::Number(10.0)));
    }

    #[test]
    fn vector_declaration() {
        let mut env = Environment::new();
        let code = r#"a = [1+2,3*4,true,[1,2,3],"hello"]"#;
        let instructions = tokenize(&code);
        let ast = parse(&instructions).unwrap();
        runtime::execute(&ast, &mut env).unwrap();
        assert_eq!(
            env.get_ref("a"),
            Ok(&Val::Vec(vec![
                Val::Number(3.0),
                Val::Number(12.0),
                Val::Bool(true),
                Val::Vec(vec![Val::Number(1.0), Val::Number(2.0), Val::Number(3.0)]),
                Val::Str("hello".to_owned())
            ]))
        );
    }
    #[test]
    fn vector_read() {
        let mut env = Environment::new();
        let code = 
        "a = [1,2,3]
        b = a[1]";
        let instructions = tokenize(&code);
        let ast = parse(&instructions).unwrap();
        runtime::execute(&ast, &mut env).unwrap();
        assert_eq!(env.get_ref("b"), Ok(&Val::Number(2.0)));
    }
    #[test]
    fn vector_write() {
        let mut env = Environment::new();
        let code = 
        "a=0
        b=[0,0,0]
        while a < 3 {
           b[a]=a
           a = a + 1
        }";
        let instructions = tokenize(&code);
        let ast = parse(&instructions).unwrap();
        runtime::execute(&ast, &mut env).unwrap();
        assert_eq!(
            env.get_ref("b"),
            Ok(&Val::Vec(vec![
                Val::Number(0.0),
                Val::Number(1.0),
                Val::Number(2.0)
            ]))
        );
    }
    #[test]
    fn vector_2d_read() {
        let mut env = Environment::new();
        let code = 
        "a=[[1,2,3],[4,5,6],[7,8,9]]
        b=a[1][1]";
        let instructions = tokenize(&code);
        let ast = parse(&instructions).unwrap();
        runtime::execute(&ast, &mut env).unwrap();
        assert_eq!(env.get_ref("b"), Ok(&Val::Number(5.0)));
    }

    #[test]
    fn vector_3d_read() {
        let mut env = Environment::new();
        let code = 
        "a=[[[5]]]
        b=a[0][0][0]";
        let instructions = tokenize(&code);
        let ast = parse(&instructions).unwrap();
        runtime::execute(&ast, &mut env).unwrap();
        assert_eq!(env.get_ref("b"), Ok(&Val::Number(5.0)));
    }

    #[test]
    fn vector_2d_write() {
        let mut env = Environment::new();
        let code = 
        "a=[[1,2,3],[4,5,6],[7,8,9]]
        a[1][1]=0";
        let instructions = tokenize(&code);
        // dbg!(&instructions);
        let ast = parse(&instructions).unwrap();
        // dbg!(&ast);
        runtime::execute(&ast, &mut env).unwrap();
        assert_eq!(
            env.get_ref("a"),
            Ok(&Val::Vec(vec![
                Val::Vec(vec![Val::Number(1.0), Val::Number(2.0), Val::Number(3.0),]),
                Val::Vec(vec![Val::Number(4.0), Val::Number(0.0), Val::Number(6.0),]),
                Val::Vec(vec![Val::Number(7.0), Val::Number(8.0), Val::Number(9.0),])
            ]))
        );
    }
    #[test]
    fn vector_copy() {
        let mut env = Environment::new();
        let code = 
        "a=0
        b=[1,2,3]
        c=[0,0,0]
        while a < 3 {
            c[a]=b[a]
            a = a + 1
        }";
        let instructions = tokenize(&code);
        let ast = parse(&instructions).unwrap();
        runtime::execute(&ast, &mut env).unwrap();
        assert_eq!(
            env.get_ref("c"),
            Ok(&Val::Vec(vec![
                Val::Number(1.0),
                Val::Number(2.0),
                Val::Number(3.0)
            ]))
        );
    }

    #[test]
    fn empty_vector() {
        let mut env = Environment::new();
        let code = "a=[]";
        let instructions = tokenize(&code);
        let ast = parse(&instructions).unwrap();
        runtime::execute(&ast, &mut env).unwrap();
        assert_eq!(env.get_ref("a"), Ok(&Val::Vec(vec![])));
    }
}
