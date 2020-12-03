extern crate d03;
use d03::{p1_solve, p2_solve, parse};
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = args
        .get(1)
        .expect("Missing argument 'part'")
        .trim()
        .parse()
        .expect("Part must be a number");

    if let Some(topology) = parse(io::stdin().lock()) {
        let result = match part {
            1 => Some(p1_solve(&topology, 3)),
            2 => Some(p2_solve(&topology)),
            _ => None,
        };

        if let Some(result) = result {
            println!("{}", result);
        }
    }
}
