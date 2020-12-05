extern crate d05;
use d05::{p1_solve, p2_solve, parse};
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = args
        .get(1)
        .expect("Missing argument 'part'")
        .trim()
        .parse()
        .expect("Part must be a number");

    let flight_passes = parse(io::stdin().lock());

    let result = match part {
        1 => Some(p1_solve(&flight_passes)),
        2 => p2_solve(&flight_passes),
        _ => None,
    };

    if let Some(result) = result {
        println!("{}", result);
    }
}
