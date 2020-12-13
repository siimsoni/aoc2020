extern crate btoi;

use btoi::{btoi, ParseIntegerError};
use std::collections::HashMap;
use std::io::BufRead;

fn parse_part(result: &mut Vec<Option<i64>>, buffer: &[u8]) {
    if let Ok(int) = btoi::<i64>(buffer) {
        result.push(Some(int));
    }
}

fn parse_page(
    iter: std::slice::Iter<u8>,
    mut result: &mut Vec<Option<i64>>,
    buffer: &mut [u8; 16],
    len: &mut usize,
) {
    for c in iter {
        match c {
            &b'\n' | &b',' => {
                parse_part(&mut result, &buffer[..*len]);
                *len = 0;
            }
            &b'x' => {
                result.push(None);
                *len = 0;
            }
            _ => {
                buffer[*len] = *c;
                *len += 1;
            }
        }
    }
}

pub fn parse<R>(mut reader: R) -> Option<(i64, Vec<Option<i64>>)>
where
    R: BufRead,
{
    let mut result = Vec::new();
    let mut page: [u8; 4096] = [0; 4096];
    let mut part_buffer: [u8; 16] = [0; 16];
    let mut part_len = 0;

    let mut some_earliest_departure: Option<i64> = None;
    if let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            return None;
        }
        let mut shared_iter = page[..page_len].iter();
        // part 1: read the beginning timestamp
        for c in &mut shared_iter {
            if c == &b'\n' {
                if let Ok(earliest_departure) = btoi::<i64>(&part_buffer[..part_len]) {
                    some_earliest_departure = Some(earliest_departure);
                }
                part_len = 0;
                break;
            }
            part_buffer[part_len] = *c;
            part_len += 1;
        }
        parse_page(shared_iter, &mut result, &mut part_buffer, &mut part_len);
    }
    if let Some(earliest_departure) = some_earliest_departure {
        while let Ok(page_len) = reader.read(&mut page) {
            if page_len == 0 {
                break;
            }
            parse_page(
                page[..page_len].iter(),
                &mut result,
                &mut part_buffer,
                &mut part_len,
            );
        }
        if part_len != 0 {
            parse_part(&mut result, &part_buffer[..part_len]);
        }
        if result.len() != 0 {
            return Some((earliest_departure, result));
        }
    }

    None
}

pub fn p1_solve((start_at, bus_numbers): &(i64, Vec<Option<i64>>)) -> Option<i64> {
    let mut counters = HashMap::new();
    for some_number in bus_numbers {
        if let Some(number) = some_number {
            counters.insert(*number, start_at % number);
        }
    }
    let mut minute = 0;
    loop {
        for (bus_number, minutes) in counters.iter_mut() {
            if minutes == bus_number {
                return Some(minute * (*bus_number) as i64);
            }
            *minutes += 1;
        }
        minute += 1;
    }
}

// start copy-pasta (seems it's a common implementation in many libraries)
fn extended_euclidian(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_euclidian(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn inverse_modulo(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = extended_euclidian(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
// end copy-pasta

pub fn p2_solve((_, bus_numbers): &(i64, Vec<Option<i64>>)) -> Option<i64> {
    let mut first_number = None;
    let mut expected = HashMap::new();
    for (i, some_number) in bus_numbers.iter().enumerate() {
        if let Some(number) = some_number {
            if first_number.is_none() {
                first_number = Some(number);
            }
            let modulo = *number as i64;
            let mut remainder = modulo - (i as i64 % modulo);
            if remainder == modulo {
                remainder = 0;
            }
            expected.insert(modulo, remainder);
        }
    }
    // linear congruence & chinese remainder
    let n_all: i64 = expected.iter().map(|(v, _)| v).product();
    let mut sum = 0;
    for (modulo, residue) in expected.iter() {
        let n = n_all / modulo;
        if let Some(modulo) = inverse_modulo(n, *modulo) {
            sum += n * residue * modulo;
        }
    }
    Some(sum % n_all)
}
