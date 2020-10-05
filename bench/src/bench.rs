use criterion::{black_box, criterion_group, criterion_main, Criterion};
use expr_eval::evaluator::Environment;
use interpreter::{parser::parse, runtime::execute, tokenizer::tokenize};

pub fn criterion_benchmark1(c: &mut Criterion) {
    c.bench_function("tokenize", |b| {
        b.iter(|| {
            tokenize(black_box(
                r#"a=0
        b=3000000
        c=""
        while a < b {
            a = a + 1
            if a % 5 == 0 {
                print a
                print "multiple of 5"
            }
        }"#,
            ))
        })
    });
}

pub fn criterion_benchmark2(c: &mut Criterion) {
    let tokens = tokenize(
        r#"a=0
b=30000
c=""
while a < b {
    a = a + 1
    if a % 5 == 0 {
        print a
        print "multiple of 5"
    }
}"#,
    );
    c.bench_function("parse", |b| b.iter(|| parse(black_box(&tokens))));
}

pub fn criterion_benchmark3(c: &mut Criterion) {
    let tokens = tokenize(
        r#"a=0
b=30000
c=""
while a < b {
    a = a + 1
    if a % 5 == 0 {
        a
        "multiplo de a"
    }
}"#,
    );
    let mut env = Environment::new();
    let ast = parse(&tokens).unwrap();
    c.bench_function("execute", |b| b.iter(|| execute(black_box(&ast), &mut env)));
}

pub fn criterion_benchmark4(c: &mut Criterion) {
    let code = r#"a=0
b=30000
c=""
while a < b {
    a = a + 1
    if a % 5 == 0 {
        a
        "multiplo de a"
    }
}"#;

    c.bench_function("all together", |b| {
        b.iter(|| {
            let mut env = Environment::new();
            let tokens = tokenize(code);
            let ast = parse(&tokens).unwrap();
            execute(black_box(&ast), &mut env)
        })
    });
}

pub fn criterion_benchmark5(c: &mut Criterion) {
    let tokens = tokenize(
        r#"a=0
b=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
while a < 20 {
   b[a]=a
   a = a + 1
}
"#,
    );
    let mut env = Environment::new();
    let ast = parse(&tokens).unwrap();
    c.bench_function("vector_write", |b| {
        b.iter(|| execute(black_box(&ast), &mut env))
    });
}

criterion_group!(
    benches,
    criterion_benchmark1,
    criterion_benchmark2,
    criterion_benchmark3,
    criterion_benchmark4,
    criterion_benchmark5,
);
criterion_main!(benches);
