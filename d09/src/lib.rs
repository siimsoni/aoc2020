extern crate btoi;

use btoi::btoi;
use std::collections::VecDeque;
use std::io::BufRead;

pub fn parse<R>(mut reader: R) -> Vec<u64>
where
    R: BufRead,
{
    let mut result = Vec::new();
    let mut page: [u8; 4096] = [0; 4096];
    let mut line_buf: [u8; 32] = [0; 32];
    let mut line_len = 0;
    let mut pages_read: usize = 0;
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        result.reserve(result.len().checked_div(pages_read).unwrap_or(page_len / 2));
        for c in page[..page_len].iter() {
            if c == &b'\n' {
                if let Ok(int) = btoi(&line_buf[..line_len]) {
                    result.push(int);
                }
                line_buf = [0; 32];
                line_len = 0;
            } else if line_len < 32 {
                line_buf[line_len] = *c;
                line_len += 1;
            }
        }
        pages_read += 1;
    }
    if line_len > 0 {
        if let Ok(int) = btoi(&line_buf[..line_len]) {
            result.push(int);
        }
    }
    result.shrink_to_fit();
    result
}

pub fn p1_solve(input: &[u64], preamble_len: usize) -> Option<u64> {
    let mut preamble = VecDeque::with_capacity(preamble_len);
    let mut iter = &mut input.iter();
    for (i, val) in &mut iter.enumerate() {
        preamble.push_back(*val);
        if i == preamble_len - 1 {
            break;
        }
    }
    for val in &mut iter {
        let mut valid = false;
        let contiguous = preamble.make_contiguous();
        let preamble_contiguous = contiguous.iter().enumerate();
        'validate: for (i, a) in preamble_contiguous {
            for b in contiguous[i + 1..].iter() {
                if &(a + b) == val {
                    valid = true;
                    break 'validate;
                }
            }
        }
        if !valid {
            return Some(*val);
        }
        // one might think it's faster to clear space first, and then push,
        // but this is over 25% faster... hmm
        preamble.push_back(*val);
        preamble.pop_front();
    }
    None
}

pub fn p2_solve(input: &[u64], target: u64) -> Option<u64> {
    let mut iter = input.iter().rev().enumerate();
    let start_at = target / 2;
    for (i, val) in &mut iter {
        if val < &start_at {
            if let Some(next) = input.get(input.len() - i - 2) {
                if next + val == target {
                    return Some(val + next);
                }
            }
            break;
        }
    }
    let mut sum;
    let mut min;
    let mut max;
    for (i, val_a) in &mut iter {
        sum = *val_a;
        min = *val_a;
        max = *val_a;
        for val_b in input[..input.len() - i - 1].iter().rev() {
            sum += *val_b;
            min = min.min(*val_b);
            max = max.max(*val_b);
            if sum > target {
                break;
            }
            if sum == target {
                return Some(min + max);
            }
        }
    }
    None
}
