use std::io::BufRead;

const EXPECTED_VALUE: i32 = 2020;

pub fn parse<R>(mut reader: R) -> Vec<i32>
where
    R: BufRead,
{
    let mut buffer = String::new();
    let mut result: Vec<i32> = Vec::new();
    while let Ok(n) = reader.read_line(&mut buffer) {
        if n == 0 {
            break;
        }
        if let Ok(number) = buffer.trim().parse() {
            result.push(number);
        }
        buffer.clear();
    }
    result
}

pub fn p1_solve(a: &[i32]) -> Option<i32> {
    let mut map: Vec<bool> = Vec::with_capacity(EXPECTED_VALUE as usize);
    map.resize(EXPECTED_VALUE as usize, false);
    for value in a {
        map[*value as usize] = true;
    }
    let mut diff;
    for value in a {
        diff = EXPECTED_VALUE - value;
        if map[diff as usize] {
            return Some(value * diff);
        }
    }
    None
}

pub fn p2_solve(a: &[i32]) -> Option<i32> {
    let mut map: Vec<bool> = Vec::with_capacity(EXPECTED_VALUE as usize);
    map.resize(EXPECTED_VALUE as usize, false);
    for value in a {
        map[*value as usize] = true;
    }

    let sorted = map
        .iter()
        .enumerate()
        .filter_map(|(v, s)| match s {
            true => Some(v as i32),
            false => None,
        })
        .collect::<Vec<_>>();

    let mut diff;

    for a_value in &sorted {
        for b_value in &sorted {
            diff = EXPECTED_VALUE - b_value - a_value;
            if diff < 1 {
                break;
            }
            if map[diff as usize] {
                return Some(a_value * b_value * diff);
            }
        }
    }
    None
}
