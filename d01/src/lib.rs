use std::io::BufRead;
use btoi::btoi;

pub fn parse<R>(mut reader: R) -> Vec<i32>
where
    R: BufRead,
{
    let mut result = Vec::new();
    let mut page = [0; 2048];
    let mut item: [u8; 16] = [0; 16];
    let mut item_len = 0;
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        for c in &page[..page_len] {
            if c == &b'\n' {
                result.push(btoi(&item[..item_len]).expect("valid integer"));
                item_len = 0;
            } else {
                item[item_len] = *c;
                item_len += 1;
            }
        }
    }
    if item_len > 0 {
        result.push(btoi(&item[..item_len]).expect("valid integer"));
    }
    result
}

pub fn p1_solve(a: &[i32], expected_value: usize) -> Option<i32> {
    let mut map: Vec<bool> = Vec::with_capacity(expected_value);
    map.resize(expected_value, false);
    for value in a {
        map[*value as usize] = true;
    }
    let mut diff;
    for value in a {
        diff = expected_value as i32 - value;
        if map[diff as usize] {
            return Some(value * diff);
        }
    }
    None
}

pub fn p2_solve(a: &[i32], expected_value: usize) -> Option<i32> {
    let mut map: Vec<bool> = Vec::with_capacity(expected_value);
    map.resize(expected_value, false);
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
            diff = expected_value as i32 - b_value - a_value;
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
