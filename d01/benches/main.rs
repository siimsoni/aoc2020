use criterion::{criterion_group, criterion_main, Criterion};
use d01::{p1_solve, p2_solve, parse};
use std::io::{stdin, Read};

fn main_benchmark(c: &mut Criterion) {
    let mut buffer = String::new();
    if let Ok(_) = stdin().lock().read_to_string(&mut buffer) {
        c.bench_function("d01 parse", |b| {
            b.iter(|| parse(buffer.as_bytes()));
        });
        let parsed = parse(buffer.as_bytes());
        c.bench_function("d01 p1 (545379)", |b| {
            b.iter(|| p1_solve(&parsed, 2020));
        });
        c.bench_function("d01 p2 (257778836)", |b| b.iter(|| p2_solve(&parsed, 2020)));
    }
}

criterion_group!(benches, main_benchmark);
criterion_main!(benches);
