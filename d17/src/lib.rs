use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::BufRead;

#[derive(Eq, Hash, PartialEq)]
pub struct Coordinate<T> {
    value: T,
}

impl<T> Coordinate<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Coordinate<T>
where
    Coordinate<T>: Copy,
{
    fn neighbors(&self) -> Neighbors<T> {
        Neighbors {
            center: *self,
            i: 0,
        }
    }
}

impl fmt::Debug for Coordinate<[i32; 3]> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        str.push('{');
        str.push_str(&self.value[0].to_string());
        str.push(',');
        str.push_str(&self.value[1].to_string());
        str.push(',');
        str.push_str(&self.value[2].to_string());
        str.push('}');
        f.write_str(&str)
    }
}

impl<T> Clone for Coordinate<T>
where
    T: Copy,
{
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}

impl<T> Copy for Coordinate<T> where T: Copy {}

pub struct Neighbors<T> {
    center: Coordinate<T>,
    i: i32,
}

impl Iterator for Neighbors<[i32; 3]> {
    type Item = Coordinate<[i32; 3]>;

    fn next(&mut self) -> Option<Coordinate<[i32; 3]>> {
        let mut i = self.i;
        if i == 26 {
            return None;
        }
        if i >= 13 {
            i += 1;
        }

        let x_offset = i / 9;
        i -= 9 * x_offset;
        let y_offset = i / 3;
        i -= 3 * y_offset;
        let z_offset = i;

        self.i += 1;

        let x = self.center.value[0] + x_offset - 1;
        let y = self.center.value[1] + y_offset - 1;
        let z = self.center.value[2] + z_offset - 1;

        Some(Coordinate::new([x, y, z]))
    }
}

impl fmt::Debug for Coordinate<[i32; 4]> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        str.push('{');
        str.push_str(&self.value[0].to_string());
        str.push(',');
        str.push_str(&self.value[1].to_string());
        str.push(',');
        str.push_str(&self.value[2].to_string());
        str.push(',');
        str.push_str(&self.value[3].to_string());
        str.push('}');
        f.write_str(&str)
    }
}

impl Iterator for Neighbors<[i32; 4]> {
    type Item = Coordinate<[i32; 4]>;

    fn next(&mut self) -> Option<Coordinate<[i32; 4]>> {
        let mut index = self.i;
        if index == 80 {
            return None;
        }
        if index >= 40 {
            index += 1;
        }

        let a_offset = index / 27;
        index -= 27 * a_offset;
        let b_offset = index / 9;
        index -= 9 * b_offset;
        let c_offset = index / 3;
        index -= 3 * c_offset;
        let d_offset = index;

        self.i += 1;

        let a = self.center.value[0] + a_offset - 1;
        let b = self.center.value[1] + b_offset - 1;
        let c = self.center.value[2] + c_offset - 1;
        let d = self.center.value[3] + d_offset - 1;

        Some(Coordinate::new([a, b, c, d]))
    }
}

pub fn parse<R>(mut reader: R) -> (Box<[bool]>, usize)
where
    R: BufRead,
{
    let mut result = Vec::new();
    let mut width = None;
    let mut page: [u8; 4096] = [0; 4096];
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        result.reserve(page_len);
        for c in page[..page_len].iter() {
            match *c {
                b'\n' => {
                    if width.is_none() {
                        width = Some(result.len());
                    }
                }
                b'.' => result.push(false),
                b'#' => result.push(true),
                _ => (),
            }
        }
    }
    (result.into_boxed_slice(), width.expect("width"))
}

pub fn p1_solve((result, width): &(Box<[bool]>, usize)) -> Option<usize> {
    let mut active = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    let z = 1;
    for is_active in result.iter() {
        if *is_active {
            active.insert(Coordinate::new([x, y, z]));
        }
        x += 1;
        if x == *width as i32 {
            x = 0;
            y += 1;
        }
    }
    for _ in 0..6 {
        let mut neighbors = HashMap::new();
        for coordinate in active.iter() {
            for neighbor in coordinate.neighbors() {
                *neighbors.entry(neighbor).or_insert(0) += 1;
            }
        }
        for (coordinate, count) in &neighbors {
            match count {
                2 => (),
                3 => {
                    if !active.contains(&coordinate) {
                        active.insert(*coordinate);
                    }
                }
                _ => {
                    if active.contains(&coordinate) {
                        active.remove(&coordinate);
                    }
                }
            }
        }
        active.retain(|coordinate| neighbors.contains_key(coordinate));
    }
    Some(active.len())
}

pub fn p2_solve((result, width): &(Box<[bool]>, usize)) -> Option<usize> {
    let mut active = HashSet::new();
    let mut a = 0;
    let mut b = 0;
    let c = 1;
    let d = 1;
    for is_active in result.iter() {
        if *is_active {
            active.insert(Coordinate::new([a, b, c, d]));
        }
        a += 1;
        if a == *width as i32 {
            a = 0;
            b += 1;
        }
    }
    for _ in 0..6 {
        let mut neighbors = HashMap::new();
        for coordinate in active.iter() {
            for neighbor in coordinate.neighbors() {
                *neighbors.entry(neighbor).or_insert(0) += 1;
            }
        }
        for (coordinate, count) in &neighbors {
            match count {
                2 => (),
                3 => {
                    if !active.contains(&coordinate) {
                        active.insert(*coordinate);
                    }
                }
                _ => {
                    if active.contains(&coordinate) {
                        active.remove(&coordinate);
                    }
                }
            }
        }
        active.retain(|coordinate| neighbors.contains_key(coordinate));
    }
    Some(active.len())
}
