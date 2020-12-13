extern crate d13;
use d13::{p1_solve, p2_solve, parse};
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = args
        .get(1)
        .expect("Missing argument 'part'")
        .trim()
        .parse()
        .expect("Part must be a number");

    let some_parsed = parse(io::stdin().lock());
    let mut result = None;

    if let Some(parsed) = some_parsed {
        result = match part {
            1 => p1_solve(&parsed),
            2 => p2_solve(&parsed),
            _ => None,
        }
    }

    if let Some(result) = result {
        println!("{}", result);
    }
}
