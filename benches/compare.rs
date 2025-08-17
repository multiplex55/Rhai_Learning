use criterion::{Criterion, criterion_group, criterion_main};
use rhai::{AST, Engine};

const SCRIPT: &str = r#"let sum = 0; for n in 0..1000 { sum += n; } sum"#;

fn pure_rust_sum() -> i64 {
    (0..1000).sum()
}

fn rhai_sum(engine: &Engine, ast: &AST) -> i64 {
    engine.eval_ast::<i64>(ast).expect("script evaluation")
}

fn benchmark(c: &mut Criterion) {
    let engine = Engine::new();
    let ast = engine.compile(SCRIPT).expect("compile script");

    c.bench_function("pure_rust_sum", |b| b.iter(|| pure_rust_sum()));
    c.bench_function("rhai_script_sum", |b| b.iter(|| rhai_sum(&engine, &ast)));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
