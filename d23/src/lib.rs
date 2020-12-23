use std::cmp::Ordering;
use std::io::BufRead;

pub fn parse<R>(mut reader: R) -> Box<[u8]>
where
    R: BufRead,
{
    let mut result = Vec::new();
    let mut page: [u8; 10] = [0; 10];
    if let Ok(page_len) = reader.read(&mut page) {
        result.reserve(page_len);
        for c in page[..page_len].iter() {
            match c {
                b'0' => result.push(0),
                b'1' => result.push(1),
                b'2' => result.push(2),
                b'3' => result.push(3),
                b'4' => result.push(4),
                b'5' => result.push(5),
                b'6' => result.push(6),
                b'7' => result.push(7),
                b'8' => result.push(8),
                b'9' => result.push(9),
                _ => (),
            }
        }
    }
    result.into_boxed_slice()
}

fn round(parsed: &[u8], result: &mut [u8]) {
    let current = &parsed[0];
    let pick_up = &parsed[1..=3];
    let remainder = &parsed[4..];

    let target = current - 1;
    let (mut result_i, mut result_v) = (0, 0);

    for (i, v) in remainder.iter().enumerate() {
        match v.cmp(&target) {
            Ordering::Equal => {
                result_i = i;
                result_v = *v;
                break;
            },
            Ordering::Less => {
                if v > &result_v || (result_v > target) {
                    result_i = i;
                    result_v = *v;
                }
            },
            Ordering::Greater => {
                if result_v == 0 || (result_v > target && v > &result_v) {
                    result_i = i;
                    result_v = *v;
                }
            }
        }
    }

    let mut pos = 1;
    for n in &remainder[..result_i] {
        result[pos] = *n;
        pos += 1;
    }
    result[pos] = result_v;
    pos += 1;
    for n in pick_up {
        result[pos] = *n;
        pos += 1;
    }
    for n in &remainder[result_i + 1..] {
        result[pos] = *n;
        pos += 1;
    }

    result.rotate_left(1);
}

pub fn p1_solve(parsed: &Box<[u8]>) -> Option<String> {

    let mut parsed = parsed.clone();

    for _ in 0..100 {
        let mut result = parsed.clone();
        round(&parsed, &mut result);
        parsed = result.clone();
    }

    while parsed[0] != 1 {
        parsed.rotate_left(1);
    }

    Some(parsed[1..].iter().map(|i| i.to_string()).collect::<String>())
}

pub fn p2_solve(parsed: &Box<[u8]>) -> Option<String> {
    None
}
