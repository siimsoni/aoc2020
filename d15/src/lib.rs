extern crate btoi;
extern crate rustc_hash;

use btoi::btoi;
use rustc_hash::FxHashMap;
use std::io::BufRead;

pub fn parse<R>(mut reader: R) -> (FxHashMap<usize, usize>, usize)
where
    R: BufRead,
{
    let mut result = FxHashMap::default();
    let mut buffer: [u8; 4096] = [0; 4096];
    let mut num_buf: [u8; 16] = [0; 16];
    let mut num_len = 0;
    let mut turn = 0;
    let mut last = 0;
    if let Ok(page_len) = reader.read(&mut buffer) {
        for c in buffer[..page_len].iter() {
            if c == &b',' {
                if let Ok(int) = btoi::<usize>(&num_buf[..num_len]) {
                    result.insert(int, turn);
                    last = int;
                    num_len = 0;
                    turn += 1;
                }
            } else {
                num_buf[num_len] = *c;
                num_len += 1;
            }
        }
        if let Ok(int) = btoi::<usize>(&num_buf[..num_len]) {
            result.insert(int, turn);
            last = int;
        }
    }
    (result, last)
}

pub fn p1_solve((history, last): &(FxHashMap<usize, usize>, usize)) -> Option<usize> {
    let mut history = history.clone();
    let mut last = *last;
    for turn in (history.len() - 1)..(2020 - 1) {
        let turns_ago = history
            .get(&last)
            .and_then(|prev_turn| Some(turn - prev_turn))
            .unwrap_or(0);
        history.insert(last, turn);
        last = turns_ago;
    }
    Some(last)
}

pub fn p2_solve((history, last): &(FxHashMap<usize, usize>, usize)) -> Option<usize> {
    let mut history = history.clone();
    let mut last = *last;
    for turn in (history.len() - 1)..(30000000 - 1) {
        let turns_ago = history
            .get(&last)
            .and_then(|prev_turn| Some(turn - prev_turn))
            .unwrap_or(0);
        history.insert(last, turn);
        last = turns_ago;
    }
    Some(last)
}
