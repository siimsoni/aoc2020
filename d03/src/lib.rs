use std::convert::From;
use std::io::BufRead;

#[derive(Debug)]
pub struct Topology {
    grid: Vec<bool>,
    width: usize,
}

impl From<&[u8]> for Topology {
    fn from(buffer: &[u8]) -> Topology {
        let mut width = None;
        let mut grid = Vec::with_capacity(buffer.len());
        let mut iter = buffer.iter();
        for c in &mut iter {
            match c {
                35 => grid.push(true),
                46 => grid.push(false),
                _ => {
                    width = Some(grid.len());
                    break;
                }
            }
        }
        for c in &mut iter {
            match c {
                35 => grid.push(true),
                46 => grid.push(false),
                _ => (),
            }
        }
        Topology {
            width: width.unwrap_or(grid.len()),
            grid,
        }
    }
}

impl Topology {
    fn get_row(&self, y: usize) -> Option<&[bool]> {
        let end = self.width * (y + 1);
        if self.grid.len() >= end {
            Some(&self.grid[end - self.width..end])
        } else {
            None
        }
    }
}

struct Slope {
    shift_by: usize,
    pos: usize,
    count: usize,
}

impl Slope {
    fn new(right: usize) -> Slope {
        Slope {
            shift_by: right,
            pos: right,
            count: 0,
        }
    }
    fn visit(&mut self, row: &[bool]) {
        self.count += row[self.pos] as usize;
        self.pos += self.shift_by;
        if self.pos >= row.len() {
            self.pos -= row.len();
        }
    }
}

pub fn parse<R>(mut reader: R) -> Option<Topology>
where
    R: BufRead,
{
    let mut buffer: Vec<u8> = Vec::new();
    if reader.read_to_end(&mut buffer).is_ok() {
        Some(Topology::from(buffer.as_slice()))
    } else {
        None
    }
}

pub fn p1_solve(topology: &Topology, right: usize) -> usize {
    let mut s1 = Slope::new(right);

    let mut y = 1;

    while let Some(row) = topology.get_row(y) {
        s1.visit(row);
        y += 1;
    }

    s1.count
}

pub fn p2_solve(topology: &Topology) -> usize {
    let mut s1 = Slope::new(1);
    let mut s2 = Slope::new(3);
    let mut s3 = Slope::new(5);
    let mut s4 = Slope::new(7);
    let mut s5 = Slope::new(1);

    let mut y = 1;

    while let Some(row) = topology.get_row(y) {
        s1.visit(row);
        s2.visit(row);
        s3.visit(row);
        s4.visit(row);

        if (y & 1) == 0 {
            s5.visit(row);
        }

        y += 1;
    }

    s1.count * s2.count * s3.count * s4.count * s5.count
}
