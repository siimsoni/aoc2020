extern crate d02;
use d02::{p1_parse_and_solve, p2_parse_and_solve};
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
        1 => Some(p1_parse_and_solve(io::stdin().lock())),
        2 => Some(p2_parse_and_solve(io::stdin().lock())),
        _ => None,
    };

    if let Some(value) = result {
        println!("{}", value);
    }
}
