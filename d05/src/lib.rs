mod flight_pass;
use flight_pass::FlightPassCollection;
use std::cmp;
use std::io::BufRead;

pub fn parse<R>(mut reader: R) -> FlightPassCollection
where
    R: BufRead,
{
    let mut code: Vec<bool> = Vec::new();
    let mut buf: Vec<u8> = Vec::with_capacity(11);

    'outer: while let Ok(n) = reader.read_until(b'\n', &mut buf) {
        match n {
            0 => break,
            10 | 11 => {
                code.reserve(10);
                for c in &buf {
                    match c {
                        b'B' => code.push(true),
                        b'F' => code.push(false),
                        b'L' => code.push(false),
                        b'R' => code.push(true),
                        b'\n' => (),
                        _ => continue 'outer,
                    }
                }
            }
            _ => (),
        }
        buf.clear();
    }

    FlightPassCollection::new(code)
}

pub fn p1_solve(flight_passes: &FlightPassCollection) -> u16 {
    let mut max_id = 0;
    for flight_pass in flight_passes.iter() {
        max_id = cmp::max(max_id, flight_pass.get_id());
    }
    max_id
}

pub fn p2_solve(flight_passes: &FlightPassCollection) -> Option<u16> {
    let mut map: [u64; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for flight_pass in flight_passes.iter() {
        let id = flight_pass.get_id();
        map[(id as usize) / 64] |= 1 << (id % 64);
    }
    let mut i = 0;
    let mut iter = map.iter();
    for bucket in &mut iter {
        if *bucket == 0 {
            i += 1;
        } else {
            let masked = bucket | (u64::max_value() >> (63 - bucket.trailing_zeros()));
            if masked == u64::max_value() {
                i += 1;
                break;
            } else {
                return Some((i * 64 + bucket.trailing_zeros()) as u16);
            }
        }
    }
    for bucket in &mut iter {
        if *bucket == u64::max_value() {
            i += 1;
        } else {
            return Some((i  * 64 + bucket.trailing_ones()) as u16);
        }
    }
    None
}
