extern crate d11;
use d11::{p1_solve, p2_solve, parse};
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = args
        .get(1)
        .expect("Missing argument 'part'")
        .trim()
        .parse()
        .expect("Part must be a number");

    let result = match part {
        1 => p1_solve(&parse(io::stdin().lock())),
        2 => p2_solve(&parse(io::stdin().lock())),
        _ => None,
    };

    if let Some(result) = result {
        println!("{}", result);
    }
}
