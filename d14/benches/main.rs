use criterion::{criterion_group, criterion_main, Criterion};
use d14::{p1_solve, p2_solve, parse};
use std::io::{stdin, Read};

fn main_benchmark(c: &mut Criterion) {
    let mut buffer = String::new();
    if stdin().lock().read_to_string(&mut buffer).is_ok() {
        c.bench_function("d14 parse", |b| b.iter(|| parse(buffer.as_bytes())));
        let parsed = parse(buffer.as_bytes());
        c.bench_function("d14 p1 (17934269678453)", |b| b.iter(|| p1_solve(&parsed)));
        c.bench_function("d14 p2 (3440662844064)", |b| b.iter(|| p2_solve(&parsed)));
    }
}

criterion_group!(benches, main_benchmark);
criterion_main!(benches);
