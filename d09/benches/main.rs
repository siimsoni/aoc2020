use criterion::{criterion_group, criterion_main, Criterion};
use d09::{p1_solve, p2_solve, parse};
use std::io::{stdin, Read};

fn main_benchmark(c: &mut Criterion) {
    let mut buffer = String::new();
    if stdin().lock().read_to_string(&mut buffer).is_ok() {
        c.bench_function("d09 parse", |b| b.iter(|| parse(buffer.as_bytes())));
        let parsed = parse(buffer.as_bytes());
        c.bench_function("d09 p1 (248131121)", |b| b.iter(|| p1_solve(&parsed, 25)));
        c.bench_function("d09 p2 (31580383)", |b| {
            b.iter(|| p2_solve(&parsed, 248131121))
        });
    }
}

criterion_group!(benches, main_benchmark);
criterion_main!(benches);
