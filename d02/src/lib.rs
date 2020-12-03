use std::io::BufRead;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
pub struct P1InputLine {
    valid: bool,
}

impl FromStr for P1InputLine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
        let mut buffer = String::new();
        for c in &mut iter {
            if c == '-' {
                break;
            }
            buffer.push(c);
        }
        let min: usize = buffer.parse().expect("Min characters");
        buffer.clear();
        for c in &mut iter {
            if c == ' ' {
                break;
            }
            buffer.push(c);
        }
        let max: usize = buffer.parse().expect("Max characters");
        buffer.clear();
        for c in &mut iter {
            if c == ':' {
                iter.next();
                break;
            }
            buffer.push(c);
        }
        let expected_character: char = buffer.parse().expect("Character");
        buffer.clear();

        let mut occurences = 0;
        for c in &mut iter {
            if c == expected_character {
                occurences += 1;
            }
        }

        Ok(P1InputLine {
            valid: (occurences >= min) && (occurences <= max),
        })
    }
}

pub struct P2InputLine {
    valid: bool,
}

impl FromStr for P2InputLine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
        let mut buffer = String::new();
        for c in &mut iter {
            if c == '-' {
                break;
            }
            buffer.push(c);
        }
        let pos1: usize = buffer.parse().expect("Min characters");
        buffer.clear();
        for c in &mut iter {
            if c == ' ' {
                break;
            }
            buffer.push(c);
        }
        let pos2: usize = buffer.parse().expect("Max characters");
        buffer.clear();
        for c in &mut iter {
            if c == ':' {
                iter.next();
                break;
            }
            buffer.push(c);
        }
        let expected_character: char = buffer.parse().expect("Character");
        buffer.clear();

        let mut pos = 1;
        let mut valid = false;
        for c in &mut iter {
            if (pos == pos1) || (pos == pos2) {
                valid ^= c == expected_character;
            }
            pos += 1;
        }

        Ok(P2InputLine { valid })
    }
}

pub fn p1_parse_and_solve<R>(mut reader: R) -> usize
where
    R: BufRead,
{
    let mut input = String::new();
    let mut c: usize = 0;
    while let Ok(n) = reader.read_line(&mut input) {
        if n == 0 {
            break;
        }
        if let Ok(line) = input.trim().parse::<P1InputLine>() {
            if line.valid {
                c += 1;
            }
        }
        input.clear();
    }
    c
}

pub fn p2_parse_and_solve<R>(mut reader: R) -> usize
where
    R: BufRead,
{
    let mut input = String::new();
    let mut c: usize = 0;
    while let Ok(n) = reader.read_line(&mut input) {
        if n == 0 {
            break;
        }
        if let Ok(line) = input.trim().parse::<P2InputLine>() {
            if line.valid {
                c += 1;
            }
        }
        input.clear();
    }
    c
}
