extern crate btoi;

use btoi::btoi;
use std::io::BufRead;

fn p1_validate(line: &[u8]) -> bool {
    let mut iter = &mut line.iter();

    let mut buffer = [0; 32];
    let mut buffer_len = 0;

    for c in &mut iter {
        if c == &b'-' {
            break;
        }
        buffer[buffer_len] = *c;
        buffer_len += 1;
    }

    let min = btoi(&buffer[..buffer_len]).expect("minimum number of occurences");

    buffer_len = 0;
    for c in &mut iter {
        if c == &b' ' {
            break;
        }
        buffer[buffer_len] = *c;
        buffer_len += 1;
    }

    let max = btoi(&buffer[..buffer_len]).expect("maximum number of occurences");

    let expected_byte = iter.next().expect("expected character");

    iter.next(); // ':'
    iter.next(); // ' '

    let mut occurences = 0;

    for c in &mut iter {
        if c == expected_byte {
            occurences += 1;
        }
    }

    occurences >= min && occurences <= max
}

pub fn p1_parse_and_solve<R>(mut reader: R) -> usize
where
    R: BufRead,
{
    let mut page = [0; 4096];
    let mut buffer = [0; 32];
    let mut buffer_len = 0;

    let mut valid_count = 0;

    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }

        for c in &page[..page_len] {
            if c == &b'\n' {
                valid_count += p1_validate(&buffer[..buffer_len]) as usize;
                buffer_len = 0;
            } else {
                buffer[buffer_len] = *c;
                buffer_len += 1;
            }
        }
    }

    if buffer_len > 0 {
        valid_count += p1_validate(&buffer[..buffer_len]) as usize;
    }

    valid_count
}

fn p2_validate(line: &[u8]) -> bool {
    let mut iter = &mut line.iter();

    let mut buffer = [0; 32];
    let mut buffer_len = 0;

    for c in &mut iter {
        if c == &b'-' {
            break;
        }
        buffer[buffer_len] = *c;
        buffer_len += 1;
    }

    let pos_first = btoi::<usize>(&buffer[..buffer_len]).expect("minimum number of occurences");

    buffer_len = 0;
    for c in &mut iter {
        if c == &b' ' {
            break;
        }
        buffer[buffer_len] = *c;
        buffer_len += 1;
    }

    let pos_second = btoi::<usize>(&buffer[..buffer_len]).expect("maximum number of occurences");

    let expected_byte = iter.next().expect("expected character");

    iter.next(); // ':'
    iter.next(); // ' '

    let mut valid = false;
    let mut pos = 1;
    for c in iter {
        if (pos == pos_first) || (pos == pos_second) {
            valid ^= c == expected_byte;
        }
        pos += 1;
    }

    valid
}

pub fn p2_parse_and_solve<R>(mut reader: R) -> usize
    where
        R: BufRead,
{
    let mut page = [0; 4096];
    let mut buffer = [0; 32];
    let mut buffer_len = 0;

    let mut valid_count = 0;

    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }

        for c in &page[..page_len] {
            if c == &b'\n' {
                valid_count += p2_validate(&buffer[..buffer_len]) as usize;
                buffer_len = 0;
            } else {
                buffer[buffer_len] = *c;
                buffer_len += 1;
            }
        }
    }

    if buffer_len > 0 {
        valid_count += p2_validate(&buffer[..buffer_len]) as usize;
    }

    valid_count
}