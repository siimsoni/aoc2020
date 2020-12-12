extern crate btoi;

use btoi::btoi;
use std::convert::TryFrom;
use std::fmt;
use std::io::BufRead;

#[derive(Debug)]
pub enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Orientation::South => "S",
            Orientation::East => "E",
            Orientation::West => "W",
            Orientation::North => "N",
        })
    }
}

#[derive(Debug)]
pub struct Instruction {
    action: Action,
    value: usize,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Action::South => "S",
            Action::Forward => "F",
            Action::East => "E",
            Action::West => "W",
            Action::North => "N",
            Action::Left => "L",
            Action::Right => "R",
        })
    }
}

impl<'a> TryFrom<&'a [u8]> for Instruction {
    type Error = &'static str;

    fn try_from(v: &'a [u8]) -> Result<Self, &'static str> {
        if let Some(action) = v.get(0).and_then(|first_byte| match first_byte {
            b'N' => Some(Action::North),
            b'S' => Some(Action::South),
            b'E' => Some(Action::East),
            b'W' => Some(Action::West),
            b'L' => Some(Action::Left),
            b'R' => Some(Action::Right),
            b'F' => Some(Action::Forward),
            _ => None,
        }) {
            if let Ok(value) = btoi(&v[1..]) {
                Ok(Instruction { action, value })
            } else {
                Err("value expected")
            }
        } else {
            Err("instruction expected")
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.action, self.value)
    }
}

pub fn parse<R>(mut reader: R) -> Vec<Instruction>
where
    R: BufRead,
{
    let mut result = Vec::new();
    let mut page: [u8; 4096] = [0; 4096];
    let mut line_buf: [u8; 8] = [0; 8];
    let mut line_len = 0;
    let mut pages_read: usize = 0;
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        result.reserve(result.len().checked_div(pages_read).unwrap_or(page_len / 2));
        for c in page[..page_len].iter() {
            if c == &b'\n' {
                if let Ok(instruction) = Instruction::try_from(&line_buf[..line_len]) {
                    result.push(instruction);
                }
                line_buf = [0; 8];
                line_len = 0;
            } else if line_len < 8 {
                line_buf[line_len] = *c;
                line_len += 1;
            }
        }
        pages_read += 1;
    }
    if line_len > 0 {
        if let Ok(instruction) = Instruction::try_from(&line_buf[..line_len]) {
            result.push(instruction);
        }
    }
    result
}

pub fn rotate_left(orientation: Orientation) -> Orientation {
    match orientation {
        Orientation::East => Orientation::North,
        Orientation::North => Orientation::West,
        Orientation::West => Orientation::South,
        Orientation::South => Orientation::East,
    }
}

pub fn rotate_right(orientation: Orientation) -> Orientation {
    match orientation {
        Orientation::East => Orientation::South,
        Orientation::South => Orientation::West,
        Orientation::West => Orientation::North,
        Orientation::North => Orientation::East,
    }
}

pub fn p1_solve(instructions: &[Instruction]) -> Option<usize> {
    let mut orientation = Orientation::East;
    let (mut east, mut north) = (0, 0);
    for i in instructions {
        let (move_to, move_by) = match i.action {
            Action::Left => {
                let mut rotation = i.value;
                while rotation > 0 {
                    orientation = rotate_left(orientation);
                    if let Some(value) = rotation.checked_sub(90) {
                        rotation = value;
                    } else {
                        break;
                    }
                }
                (&orientation, 0)
            }
            Action::Right => {
                let mut rotation = i.value;
                while rotation > 0 {
                    orientation = rotate_right(orientation);
                    if let Some(value) = rotation.checked_sub(90) {
                        rotation = value;
                    } else {
                        break;
                    }
                }
                (&orientation, 0)
            }
            Action::North => (&Orientation::North, i.value),
            Action::West => (&Orientation::West, i.value),
            Action::South => (&Orientation::South, i.value),
            Action::East => (&Orientation::East, i.value),
            Action::Forward => (&orientation, i.value),
        };

        match move_to {
            Orientation::East => east += move_by as i32,
            Orientation::North => north += move_by as i32,
            Orientation::West => east -= move_by as i32,
            Orientation::South => north -= move_by as i32,
        }
        // println!("{}\t{}\tN{}\tS{}\tE{}\tW{}", i, orientation, north, south, east, west);
    }

    Some((north.abs() + east.abs()) as usize)
}

struct Waypoint {
    north: i32,
    east: i32,
}

impl Waypoint {
    fn move_by(&mut self, orientation: Orientation, value: usize) {
        match orientation {
            Orientation::North => self.north += value as i32,
            Orientation::East => self.east += value as i32,
            Orientation::South => self.north -= value as i32,
            Orientation::West => self.east -= value as i32,
        }
    }

    fn rotate_left(&mut self) {
        let east = 0 - self.north;
        self.north = self.east;
        self.east = east;
    }

    fn rotate_right(&mut self) {
        let north = 0 - self.east;
        self.east = self.north;
        self.north = north;
    }
}

pub fn p2_solve(instructions: &[Instruction]) -> Option<usize> {
    let (mut east, mut north) = (0, 0);
    let mut waypoint = Waypoint { north: 1, east: 10 };
    for i in instructions {
        match i.action {
            Action::Left => {
                let mut rotation = i.value;
                while rotation > 0 {
                    waypoint.rotate_left();
                    if let Some(value) = rotation.checked_sub(90) {
                        rotation = value;
                    } else {
                        break;
                    }
                }
            }
            Action::Right => {
                let mut rotation = i.value;
                while rotation > 0 {
                    waypoint.rotate_right();
                    if let Some(value) = rotation.checked_sub(90) {
                        rotation = value;
                    } else {
                        break;
                    }
                }
            }
            Action::North => waypoint.move_by(Orientation::North, i.value),
            Action::West => waypoint.move_by(Orientation::West, i.value),
            Action::South => waypoint.move_by(Orientation::South, i.value),
            Action::East => waypoint.move_by(Orientation::East, i.value),
            Action::Forward => {
                for _ in 0..i.value {
                    east += waypoint.east;
                    north += waypoint.north;
                }
            }
        };
        // println!("{}\tN{}\tE{}", i, north, east);
    }

    Some((north.abs() + east.abs()) as usize)
}
