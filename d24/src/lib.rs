extern crate rustc_hash;
use rustc_hash::{FxHashMap, FxHashSet};
use std::io::BufRead;

enum ParserState {
    None,
    South,
    North,
}

#[derive(Debug, Clone)]
pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthEast,
    NorthWest,
}

pub fn parse<R>(mut reader: R) -> Vec<Vec<Direction>>
where
    R: BufRead,
{
    let mut result = Vec::new();
    let mut page: [u8; 4096] = [0; 4096];
    let mut directions = Vec::new();
    let mut state = ParserState::None;
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        for c in page[..page_len].iter() {
            match *c {
                b'\n' => {
                    result.push(directions.clone());
                    directions.clear();
                }
                b'n' => {
                    state = ParserState::North;
                }
                b's' => {
                    state = ParserState::South;
                }
                b'w' => {
                    match state {
                        ParserState::None => directions.push(Direction::West),
                        ParserState::South => directions.push(Direction::SouthWest),
                        ParserState::North => directions.push(Direction::NorthWest),
                    }
                    state = ParserState::None;
                }
                b'e' => {
                    match state {
                        ParserState::None => directions.push(Direction::East),
                        ParserState::South => directions.push(Direction::SouthEast),
                        ParserState::North => directions.push(Direction::NorthEast),
                    }
                    state = ParserState::None;
                }
                _ => (),
            }
        }
    }
    if !directions.is_empty() {
        result.push(directions);
    }
    result
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
    value: [i32; 2],
}

impl Coordinate {
    fn new() -> Self {
        Self { value: [0, 0] }
    }

    fn move_towards(&mut self, direction: &Direction) {
        match direction {
            Direction::East => {
                self.value[0] += 1;
            }
            Direction::SouthEast => {
                self.value[1] += 1;
            }
            Direction::SouthWest => {
                self.value[0] -= 1;
                self.value[1] += 1;
            }
            Direction::West => {
                self.value[0] -= 1;
            }
            Direction::NorthWest => {
                self.value[1] -= 1;
            }
            Direction::NorthEast => {
                self.value[0] += 1;
                self.value[1] -= 1;
            }
        }
    }

    fn neighbors(&self) -> Neighbors {
        Neighbors {
            center: self.clone(),
            i: 0,
        }
    }
}

pub struct Neighbors {
    center: Coordinate,
    i: i32,
}

impl Iterator for Neighbors {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Coordinate> {
        let result = match self.i {
            0 => {
                let mut result = self.center.clone();
                result.move_towards(&Direction::East);
                Some(result)
            }
            1 => {
                let mut result = self.center.clone();
                result.move_towards(&Direction::SouthEast);
                Some(result)
            }
            2 => {
                let mut result = self.center.clone();
                result.move_towards(&Direction::SouthWest);
                Some(result)
            }
            3 => {
                let mut result = self.center.clone();
                result.move_towards(&Direction::West);
                Some(result)
            }
            4 => {
                let mut result = self.center.clone();
                result.move_towards(&Direction::NorthWest);
                Some(result)
            }
            5 => {
                let mut result = self.center.clone();
                result.move_towards(&Direction::NorthEast);
                Some(result)
            }
            _ => None,
        };
        self.i += 1;
        result
    }
}

pub fn p1_solve(directions: &[Vec<Direction>]) -> Option<usize> {
    let mut coordinate: Coordinate;
    let mut flipped = FxHashSet::default();
    for tile in directions {
        coordinate = Coordinate::new();
        for direction in tile {
            coordinate.move_towards(&direction);
        }
        if flipped.contains(&coordinate) {
            flipped.remove(&coordinate);
        } else {
            flipped.insert(coordinate.clone());
        }
    }
    Some(flipped.len())
}

pub fn p2_solve(directions: &[Vec<Direction>]) -> Option<usize> {
    let mut coordinate: Coordinate;
    let mut black_tiles = FxHashSet::default();
    for tile in directions {
        coordinate = Coordinate::new();
        for direction in tile {
            coordinate.move_towards(&direction);
        }
        if black_tiles.contains(&coordinate) {
            black_tiles.remove(&coordinate);
        } else {
            black_tiles.insert(coordinate.clone());
        }
    }

    let mut neighbors = FxHashMap::default();
    for _ in 0..100 {
        for tile in black_tiles.iter() {
            for neighbor in tile.neighbors() {
                let count = neighbors.entry(neighbor).or_insert(0);
                *count += 1;
            }
        }
        black_tiles.retain(|coordinate| neighbors.contains_key(coordinate));
        for (coordinate, count) in neighbors.iter() {
            let is_black = black_tiles.contains(coordinate);
            if is_black {
                if count > &2 {
                    black_tiles.remove(coordinate);
                }
            } else if count == &2 {
                black_tiles.insert(coordinate.clone());
            }
        }
        neighbors.clear();
    }

    Some(black_tiles.len())
}
