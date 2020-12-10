extern crate btoi;

use btoi::btoi;
use std::io::BufRead;

pub fn parse<R>(mut reader: R) -> Vec<bool>
where
    R: BufRead,
{
    let mut result = Vec::new();
    let mut page: [u8; 4096] = [0; 4096];
    let mut line_buf: [u8; 32] = [0; 32];
    let mut line_len = 0;
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        for c in page[..page_len].iter() {
            if c == &b'\n' {
                if let Ok(int) = btoi::<usize>(&line_buf[..line_len]) {
                    if int > result.len() {
                        result.resize(int, false);
                    }
                    result[int - 1] = true;
                }
                line_buf = [0; 32];
                line_len = 0;
            } else if line_len < 32 {
                line_buf[line_len] = *c;
                line_len += 1;
            }
        }
    }
    if line_len > 0 {
        if let Ok(int) = btoi::<usize>(&line_buf[..line_len]) {
            if int > result.len() {
                result.resize(int, false);
            }
            result[int - 1] = true;
        }
    }
    result
}

pub fn p1_solve(bag_of_adapters: &[bool]) -> Option<u64> {
    let mut ones = 0;
    let mut threes = 1;
    let mut i = 0;

    if bag_of_adapters[0] {
        ones += 1;
    } else if bag_of_adapters[2] {
        threes += 1;
    }

    while i < bag_of_adapters.len() - 3 {
        i += 1;
        if bag_of_adapters[i] {
            ones += 1;
            continue;
        }
        i += 1;
        if bag_of_adapters[i] {
            continue;
        }
        i += 1;
        if bag_of_adapters[i] {
            threes += 1;
            continue;
        }
        return None;
    }

    if i < bag_of_adapters.len() - 2 {
        if bag_of_adapters[i + 1] {
            ones += 1;
            i += 1;
        } else if bag_of_adapters[i + 2] {
            i += 2;
        } else {
            return None;
        }
    }

    if i < bag_of_adapters.len() - 1 {
        if bag_of_adapters[i + 1] {
            ones += 1;
        } else {
            return None;
        }
    }

    Some(ones * threes)
}

pub fn p2_solve(bag_of_adapters: &[bool]) -> Option<u64> {
    let mut last3: u64 = 1;
    let mut last2: u64 = 0;
    let mut last1: u64 = 0;
    for i in bag_of_adapters.iter().rev() {
        if *i {
            let last = last1 + last2 + last3;
            last3 = last2;
            last2 = last1;
            last1 = last;
        } else {
            last3 = last2;
            last2 = last1;
            last1 = 0;
        }
    }
    Some(last1 + last2 + last3)
}
