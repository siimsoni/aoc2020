mod tokenizer;
use btoi::btoi;
use rustc_hash::{FxHashSet};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io::BufRead;
use tokenizer::{LiteralKind, TokenKind, Tokenizer};

pub fn parse<R>(mut reader: R) -> (VecDeque<u8>, VecDeque<u8>)
where
    R: BufRead,
{
    let mut input = Vec::new();
    let mut tokenizer = Tokenizer::new();

    let mut page: [u8; 4096] = [0; 4096];
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        tokenizer.tokenize(&mut (page[..page_len].iter()));
        input.extend_from_slice(&page[..page_len]);
    }
    tokenizer.flush();

    let mut token_iter = tokenizer.tokens.iter();
    let mut pos = 0;

    for token in &mut token_iter {
        pos += token.len;
        if token.kind == TokenKind::EndOfLine {
            break;
        }
    }

    let mut p1 = VecDeque::new();

    for token in &mut token_iter {
        match token.kind {
            TokenKind::Literal(LiteralKind::Integer) => {
                p1.push_back(btoi(&input[pos..pos + token.len]).expect("integer"));
                pos += token.len;
            }
            TokenKind::Literal(LiteralKind::Char) => {
                pos += token.len;
                break;
            }
            _ => {
                pos += token.len;
            }
        }
    }

    for token in &mut token_iter {
        pos += token.len;
        if token.kind == TokenKind::EndOfLine {
            break;
        }
    }

    let mut p2 = VecDeque::new();

    for token in &mut token_iter {
        match token.kind {
            TokenKind::Literal(LiteralKind::Integer) => {
                p2.push_back(btoi(&input[pos..pos + token.len]).expect("integer"));
                pos += token.len;
            }
            TokenKind::Literal(LiteralKind::Char) => {
                break;
            }
            _ => {
                pos += token.len;
            }
        }
    }

    (p1, p2)
}

pub fn p1_solve((p1, p2): &(VecDeque<u8>, VecDeque<u8>)) -> Option<usize> {
    let mut p1 = p1.clone();
    let mut p2 = p2.clone();

    while !p1.is_empty() && !p2.is_empty() {
        let p1_draw = p1.pop_front().unwrap();
        let p2_draw = p2.pop_front().unwrap();

        match p1_draw.cmp(&p2_draw) {
            Ordering::Greater => {
                p1.push_back(p1_draw);
                p1.push_back(p2_draw);
            }
            Ordering::Less => {
                p2.push_back(p2_draw);
                p2.push_back(p1_draw);
            }
            _ => (),
        }
    }

    let winner = if p1.is_empty() { p2 } else { p1 };

    Some(
        winner
            .iter()
            .rev()
            .enumerate()
            .map(|(i, v)| *v as usize * (i + 1))
            .sum(),
    )
}

pub fn recursive(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>) -> bool {
    let mut history = FxHashSet::default();

    while !p1.is_empty() && !p2.is_empty() {
        let p1_draw = p1.pop_front().unwrap();
        let p2_draw = p2.pop_front().unwrap();

        let round = (p1.clone(), p2.clone());
        if !history.insert(round) {
            return true;
        }

        let p1_won = if (p1_draw as usize) <= p1.len() && (p2_draw as usize) <= p2.len() {
            let mut p1_rec = p1.clone();
            if (p1_draw as usize) < p1_rec.len() {
                p1_rec.resize(p1_draw as usize, 0);
            }
            let mut p2_rec = p2.clone();
            if (p2_draw as usize) < p2_rec.len() {
                p2_rec.resize(p2_draw as usize, 0);
            }
            recursive(&mut p1_rec, &mut p2_rec)
        } else {
            p1_draw > p2_draw
        };

        if p1_won {
            p1.push_back(p1_draw);
            p1.push_back(p2_draw);
        } else {
            p2.push_back(p2_draw);
            p2.push_back(p1_draw);
        }
    }

    !p1.is_empty()
}

pub fn p2_solve((p1, p2): &(VecDeque<u8>, VecDeque<u8>)) -> Option<usize> {
    let mut p1 = p1.clone();
    let mut p2 = p2.clone();
    recursive(&mut p1, &mut p2);
    let winner = if p1.is_empty() { p2 } else { p1 };

    Some(
        winner
            .iter()
            .rev()
            .enumerate()
            .map(|(i, v)| *v as usize * (i + 1))
            .sum(),
    )
}
