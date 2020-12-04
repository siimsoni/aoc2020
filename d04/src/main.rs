extern crate d04;
use d04::{p1_solve, p2_solve, parse};
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = args
        .get(1)
        .expect("Missing argument 'part'")
        .trim()
        .parse()
        .expect("Part must be a number");

    let passports = parse(io::stdin().lock());

    let result = match part {
        1 => Some(p1_solve(&passports)),
        2 => Some(p2_solve(&passports)),
        _ => None,
    };

    if let Some(result) = result {
        println!("{}", result);
    }
}
