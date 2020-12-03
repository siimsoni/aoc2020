use criterion::{criterion_group, criterion_main, Criterion};
use d03::{p1_solve, p2_solve, parse};
use std::io::{stdin, Read};

fn main_benchmark(c: &mut Criterion) {
    let mut buffer = String::new();
    if stdin().lock().read_to_string(&mut buffer).is_ok() {
        c.bench_function("d03 parse", |b| b.iter(|| parse(buffer.as_bytes())));
        if let Some(parsed) = parse(buffer.as_bytes()) {
            c.bench_function("d03 p1 (274)", |b| b.iter(|| p1_solve(&parsed, 3)));
            c.bench_function("d03 p2 (6050183040)", |b| b.iter(|| p2_solve(&parsed)));
        }
    }
}

criterion_group!(benches, main_benchmark);
criterion_main!(benches);
