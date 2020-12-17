use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::BufRead;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn neighbors(&self) -> Neighbors {
        Neighbors { center: *self, i: 0 }
    }
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        str.push('{');
        str.push_str(&self.x.to_string());
        str.push(',');
        str.push_str(&self.y.to_string());
        str.push(',');
        str.push_str(&self.z.to_string());
        str.push('}');
        f.write_str(&str)
    }
}

pub struct Neighbors {
    center: Coordinate,
    i: usize,
}

impl Iterator for Neighbors {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Coordinate> {
        let result = match self.i {
            0 => Some(Coordinate::new(self.center.x - 1, self.center.y - 1, self.center.z - 1)),
            1 => Some(Coordinate::new(self.center.x - 1, self.center.y - 1, self.center.z )),
            2 => Some(Coordinate::new(self.center.x - 1, self.center.y - 1, self.center.z + 1)),
            
            3 => Some(Coordinate::new(self.center.x - 1, self.center.y, self.center.z - 1)),
            4 => Some(Coordinate::new(self.center.x - 1, self.center.y, self.center.z )),
            5 => Some(Coordinate::new(self.center.x - 1, self.center.y, self.center.z + 1)),
            
            6 => Some(Coordinate::new(self.center.x - 1, self.center.y + 1, self.center.z - 1)),
            7 => Some(Coordinate::new(self.center.x - 1, self.center.y + 1, self.center.z )),
            8 => Some(Coordinate::new(self.center.x - 1, self.center.y + 1, self.center.z + 1)),

            9 => Some(Coordinate::new(self.center.x, self.center.y - 1, self.center.z - 1)),
            10 => Some(Coordinate::new(self.center.x, self.center.y - 1, self.center.z )),
            11 => Some(Coordinate::new(self.center.x, self.center.y - 1, self.center.z + 1)),

            12 => Some(Coordinate::new(self.center.x, self.center.y, self.center.z - 1)),
            13 => Some(Coordinate::new(self.center.x, self.center.y, self.center.z + 1)),

            14 => Some(Coordinate::new(self.center.x, self.center.y + 1, self.center.z - 1)),
            15 => Some(Coordinate::new(self.center.x, self.center.y + 1, self.center.z )),
            16 => Some(Coordinate::new(self.center.x, self.center.y + 1, self.center.z + 1)),
            
            17 => Some(Coordinate::new(self.center.x + 1, self.center.y - 1, self.center.z - 1)),
            18 => Some(Coordinate::new(self.center.x + 1, self.center.y - 1, self.center.z )),
            19 => Some(Coordinate::new(self.center.x + 1, self.center.y - 1, self.center.z + 1)),

            20 => Some(Coordinate::new(self.center.x + 1, self.center.y, self.center.z - 1)),
            21 => Some(Coordinate::new(self.center.x + 1, self.center.y, self.center.z )),
            22 => Some(Coordinate::new(self.center.x + 1, self.center.y, self.center.z + 1)),

            23 => Some(Coordinate::new(self.center.x + 1, self.center.y + 1, self.center.z - 1)),
            24 => Some(Coordinate::new(self.center.x + 1, self.center.y + 1, self.center.z )),
            25 => Some(Coordinate::new(self.center.x + 1, self.center.y + 1, self.center.z + 1)),

            _ => None,
        };
        
        self.i += 1;

        result
    }
}

#[derive(Clone, Debug)]
pub struct Space {
    active: HashSet<Coordinate>,
}

impl Space {
    fn new() -> Self {
        Self {
            active: HashSet::new(),
        }
    }

    pub fn insert(&mut self, coordinate: Coordinate) {
        self.active.insert(coordinate);
    }
}

pub fn parse<R>(mut reader: R) -> Space
where
    R: BufRead,
{
    let mut space = Space::new();

    let mut x = 0;
    let mut y = 0;
    let z = 0;

    let mut page: [u8; 4096] = [0; 4096];
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        for c in page[..page_len].iter() {
            match c {
                &b'\n' => {
                    y += 1;
                    x = 0;
                }
                &b'.' => x += 1,
                &b'#' => {
                    space.insert(Coordinate::new(x, y, z));
                    x += 1;
                }
                _ => (),
            }
        }
    }
    space
}

pub fn debug(active: &HashSet<Coordinate>, z: i32, size: i32) {
    for z in -z..=z {
        println!("z={}", z);
        for y in -size..=size {
            for x in -size..=size {
                if active.contains(&Coordinate::new(x, y, z)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }

}

pub fn p1_solve(space: &Space) -> Option<u64> {
    let mut active = space.active.clone();

    // println!("{:#?}", active);

    for _ in 0..6 {
        // println!("---");
        // debug(&active, 2, 5);


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

        println!("{:#?}", active.len());
    }

    None
}

pub fn p2_solve(space: &Space) -> Option<u64> {
    None
}
