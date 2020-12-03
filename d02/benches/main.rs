use criterion::{criterion_group, criterion_main, Criterion};
use d02::{p1_parse_and_solve, p2_parse_and_solve};
use std::io::{stdin, Read};

fn main_benchmark(c: &mut Criterion) {
    let mut buffer = String::new();
    if let Ok(_) = stdin().lock().read_to_string(&mut buffer) {
        c.bench_function("d02 p1 (580)", |b| {
            b.iter(|| p1_parse_and_solve(buffer.as_bytes()))
        });
        c.bench_function("d02 p2 (611)", |b| {
            b.iter(|| p2_parse_and_solve(buffer.as_bytes()))
        });
    }
}

criterion_group!(benches, main_benchmark);
criterion_main!(benches);
