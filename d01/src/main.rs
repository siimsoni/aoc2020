extern crate d01;
use d01::{p1_solve, p2_solve, parse};
use std::env;
use std::io::stdin;

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = args
        .get(1)
        .expect("required [part]")
        .trim()
        .parse()
        .expect("invalid [part]");

    let expected_value = args
        .get(2)
        .expect("required [value]")
        .trim()
        .parse()
        .expect("invalid [value]");

    let parsed = parse(stdin().lock());

    let result = match part {
        1 => p1_solve(&parsed, expected_value),
        2 => p2_solve(&parsed, expected_value),
        _ => {
            println!("Invalid part: {}", part);
            None
        }
    };

    if let Some(value) = result {
        println!("{}", value);
    }
}
