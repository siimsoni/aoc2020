extern crate btoi;

use btoi::btoi;
use std::collections::{BTreeSet};
use std::io::BufRead;

pub fn parse<R>(mut reader: R) -> BTreeSet<u64>
where
    R: BufRead,
{
    let mut result = BTreeSet::new();
    let mut page: [u8; 4096] = [0; 4096];
    let mut line_buf: [u8; 32] = [0; 32];
    let mut line_len = 0;
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        for c in page[..page_len].iter() {
            if c == &b'\n' {
                if let Ok(int) = btoi(&line_buf[..line_len]) {
                    result.insert(int);
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
        if let Ok(int) = btoi(&line_buf[..line_len]) {
            result.insert(int);
        }
    }
    result
}

pub fn p1_solve(bag_of_adapters: &BTreeSet<u64>) -> Option<u64> {
    let mut count_ones = 0;
    let mut count_threes = 1;
    let mut prev = 0;
    for adapter in bag_of_adapters {
        match adapter - &prev {
            1 => count_ones += 1,
            2 => (),
            3 => count_threes += 1,
            _ => return None,
        }
        prev = *adapter
    }
    Some(count_ones * count_threes)
}

pub fn p2_solve(bag_of_adapters: &BTreeSet<u64>) -> Option<u64> {
    let highest = bag_of_adapters.iter().rev().next().unwrap_or(&0);
    let mut prev = highest;
    let mut last3: u64 = 1;
    let mut last2: u64 = 0;
    let mut last1: u64 = 0;
    for adapter in bag_of_adapters.iter().rev() {
        for _ in 1..prev-adapter {
            last3 = last2;
            last2 = last1;
            last1 = 0;
        }
        prev = adapter;
        let last = last1 + last2 + last3;
        last3 = last2;
        last2 = last1;
        last1 = last;
    }
    Some(last1 + last2 + last3)
}

