extern crate btoi;
#[macro_use]
extern crate nom;
extern crate rustc_hash;

use btoi::btoi;
use nom::character::{is_alphanumeric, is_digit};
use rustc_hash::FxHashMap;
use std::fmt;
use std::io::BufRead;
#[derive(Debug)]
pub enum Instruction {
    UpdateBitmask(Bitmask),
    WriteValue(AddressValue),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UpdateBitmask(bitmask) => bitmask.fmt(f),
            Self::WriteValue(value) => value.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct Bitmask {
    ones: u64,
    zeros: u64,
}

impl From<&[u8]> for Bitmask {
    fn from(buffer: &[u8]) -> Self {
        let mut ones = 0;
        let mut zeros = u64::max_value();
        let mut pos = buffer.len();
        for c in buffer.iter() {
            pos -= 1;
            match c {
                b'1' => {
                    ones |= 1 << pos;
                }
                b'0' => {
                    zeros &= !(1 << pos);
                }
                _ => (),
            }
        }

        Self { ones, zeros }
    }
}

impl fmt::Display for Bitmask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::from("mask = ");
        for i in (0..36).rev() {
            let zero = (self.zeros >> i) & 1;
            let one = (self.ones >> i) & 1;
            if one == 1 {
                result.push('1');
            } else if zero != 1 {
                result.push('0');
            } else {
                result.push('X');
            }
        }
        f.write_str(&result)
    }
}

fn get_floating_recursive(value: u64, bit: usize, map: u64, mut result: &mut Vec<u64>) {
    let is_floating = (map >> bit) & 1 == 0;
    if bit == 0 {
        if is_floating {
            result.push(value | 1 << bit);
            result.push(value & !(1 << bit));
        } else {
            result.push(value);
        }
        return;
    }
    if is_floating {
        get_floating_recursive(value | 1 << bit, bit - 1, map, &mut result);
        get_floating_recursive(value & !(1 << bit), bit - 1, map, &mut result);
        return;
    }
    get_floating_recursive(value, bit - 1, map, &mut result);
}

impl Bitmask {
    fn mask(&self, value: u64) -> u64 {
        (value | self.ones) & self.zeros
    }

    fn get_floating(&self, value: u64) -> Box<[u64]> {
        let value = value | self.ones;
        let float_map = !self.zeros | self.ones;

        let masked = float_map | u64::max_value() << 36;
        let mut result = Vec::with_capacity(usize::pow(2, masked.count_zeros()));
        get_floating_recursive(value, 35, float_map, &mut result);
        result.into_boxed_slice()
    }
}

#[derive(Debug)]
pub struct AddressValue {
    address: usize,
    value: u64,
}

impl fmt::Display for AddressValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mem[{}] = {}", self.address, self.value)
    }
}

named!(
    update_bitmask<Instruction>,
    do_parse!(
        tag!("mask = ")
            >> bitmask: take_while!(is_alphanumeric)
            >> char!('\n')
            >> (Instruction::UpdateBitmask(Bitmask::from(bitmask)))
    )
);

named!(
    write_value<Instruction>,
    do_parse!(
        tag!("mem")
            >> address: delimited!(char!('['), take_while!(is_digit), char!(']'))
            >> tag!(" = ")
            >> value: take_while!(is_digit)
            >> char!('\n')
            >> (Instruction::WriteValue(AddressValue {
                address: btoi(address).unwrap_or(0),
                value: btoi(value).unwrap_or(0),
            }))
    )
);

named!(
    read_instruction<Instruction>,
    alt!(write_value | update_bitmask)
);

pub fn parse<R>(mut reader: R) -> Vec<Instruction>
where
    R: BufRead,
{
    let mut result = Vec::new();
    let mut page: [u8; 4096] = [0; 4096];
    let mut line_buf: [u8; 1024] = [0; 1024];
    let mut line_len = 0;
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        for c in page[..page_len].iter() {
            match c {
                b'\n' => {
                    line_buf[line_len] = *c;
                    line_len += 1;
                    if let Ok((_, instruction)) = read_instruction(&line_buf[..line_len]) {
                        result.push(instruction);
                    }
                    line_len = 0;
                }
                _ => {
                    line_buf[line_len] = *c;
                    line_len += 1;
                }
            }
        }
    }
    if line_len > 0 {
        line_buf[line_len] = b'\n';
        line_len += 1;
        if let Ok((_, instruction)) = read_instruction(&line_buf[..line_len]) {
            result.push(instruction);
        }
    }
    result
}

pub fn p1_solve(instructions: &[Instruction]) -> Option<u64> {
    let mut values = FxHashMap::default();
    let mut active_bitmask = None;
    for inst in instructions {
        match inst {
            Instruction::UpdateBitmask(bitmask) => {
                active_bitmask = Some(bitmask);
            }
            Instruction::WriteValue(value) => {
                match active_bitmask {
                    Some(bitmask) => {
                        values.insert(value.address, bitmask.mask(value.value));
                    }
                    _ => {
                        values.insert(value.address, value.value);
                    }
                };
            }
        }
    }
    Some(values.iter().map(|(_, v)| v).sum())
}

pub fn p2_solve(instructions: &[Instruction]) -> Option<u64> {
    let mut values = FxHashMap::default();
    values.reserve(instructions.len());
    let mut active_bitmask = None;
    for inst in instructions {
        match inst {
            Instruction::UpdateBitmask(bitmask) => {
                active_bitmask = Some(bitmask);
            }
            Instruction::WriteValue(value) => {
                if let Some(bitmask) = active_bitmask {
                    for addr in bitmask.get_floating(value.address as u64).iter() {
                        values.insert(*addr, value.value);
                    }
                } else {
                    values.insert(value.address as u64, value.value);
                }
            }
        }
    }
    Some(values.iter().map(|(_, v)| v).sum())
}
