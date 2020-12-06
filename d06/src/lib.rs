mod declaration;
use declaration::Declaration;
use std::io::BufRead;

pub fn parse<R>(mut reader: R) -> Vec<Declaration>
where
    R: BufRead,
{
    let mut result: Vec<Declaration> = Vec::new();
    let mut in_buf: [u8; 4096] = [0; 4096];
    let mut group = 0;
    let mut answers = 0;
    while let Ok(in_len) = reader.read(&mut in_buf) {
        if in_len == 0 {
            break;
        }
        if (result.capacity() - result.len()) < (in_len / 8) {
            result.reserve(in_len / 8);
        }
        for c in in_buf[..in_len].iter() {
            if c == &b'\n' {
                if answers == 0 {
                    group += 1;
                } else {
                    result.push(Declaration { group, answers });
                    answers = 0;
                }
            } else {
                answers |= 1 << (c - b'a');
            }
        }
    }
    if answers != 0 {
        result.push(Declaration { group, answers });
    }

    result
}

pub fn p1_solve(declarations: &[Declaration]) -> usize {
    let mut sum = 0;
    let mut group_answers: u32 = 0;
    let mut group_index = 0;
    for dec in declarations {
        if dec.group != group_index {
            sum += group_answers.count_ones();
            group_index = dec.group;
            group_answers = 0;
        }
        group_answers |= dec.answers;
    }
    sum += group_answers.count_ones();
    sum as usize
}

pub fn p2_solve(declarations: &[Declaration]) -> usize {
    let mut sum = 0;
    let mut group_answers: u32 = 0;
    let mut group_index = 0;
    let mut iter = declarations.iter();
    if let Some(dec) = &mut iter.next() {
        group_answers = dec.answers;
    }
    for dec in &mut iter {
        if dec.group != group_index {
            sum += group_answers.count_ones();
            group_index = dec.group;
            group_answers = dec.answers;
        } else {
            group_answers &= dec.answers;
        }
    }
    sum += group_answers.count_ones();
    sum as usize
}
