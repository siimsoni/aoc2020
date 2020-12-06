mod declaration;
use declaration::Declaration;
use std::io::BufRead;

pub fn parse<R>(mut reader: R) -> Vec<Declaration>
where
    R: BufRead,
{
    let mut buf: Vec<u8> = Vec::new();
    let mut result = Vec::new();
    let mut group = 0;
    while let Ok(n) = reader.read_until(b'\n', &mut buf) {
        match n {
            0 => break,
            1 => {
                if buf == b"\n" {
                    group += 1;
                } else {
                    result.push(Declaration {
                        group,
                        answers: 1 << (buf[0] - b'a'),
                    });
                }
            }
            _ => {
                if buf[buf.len() - 1] == b'\n' {
                    buf.pop();
                }
                let mut answers = 0;
                for c in &buf {
                    answers |= 1 << (c - b'a');
                }
                result.push(Declaration { group, answers });
            }
        }
        buf.clear();
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
