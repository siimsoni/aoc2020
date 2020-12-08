mod model;

use model::{Instruction, Operation, Program};
use std::convert::TryFrom;
use std::io::BufRead;

type ParseResult = Program;

pub fn parse<R>(mut reader: R) -> ParseResult
where
    R: BufRead,
{
    let mut result = Program {
        instructions: Vec::new(),
        cursor: 0,
        accumulator: 0,
    };
    let mut buf: [u8; 4096] = [0; 4096];
    let mut line_buf: [u8; 16] = [0; 16];
    let mut line_len = 0;
    while let Ok(len) = reader.read(&mut buf) {
        if len == 0 {
            break;
        }
        if (result.instructions.capacity() - result.instructions.len()) < len / 7 {
            result.instructions.reserve(len / 7);
        }
        for c in buf[..len].iter() {
            if c == &b'\n' {
                if let Ok(instruction) = Instruction::try_from(&line_buf[..line_len]) {
                    result.instructions.push(instruction);
                }
                line_buf = [0; 16];
                line_len = 0;
            } else {
                line_buf[line_len] = *c;
                line_len += 1;
            }
        }
    }
    if line_len > 0 {
        if let Ok(instruction) = Instruction::try_from(&line_buf[..line_len]) {
            result.instructions.push(instruction);
        }
    }
    result.instructions.shrink_to_fit();
    result
}

pub fn p1_solve(mut parsed: ParseResult) -> i32 {
    parsed = parsed.run();
    parsed.accumulator
}

pub fn p2_solve(parsed: ParseResult) -> Option<i32> {
    let mut steps = Vec::new();
    parsed.clone().run_tracked(&mut steps);
    for step in steps {
        if let Some(parsed_instruction) = parsed.instructions.get(step) {
            let swapped;
            match parsed_instruction.operation {
                Operation::Nop => swapped = Operation::Jmp,
                Operation::Jmp => swapped = Operation::Nop,
                _ => continue,
            }
            let mut program = parsed.clone();
            let mut instruction = program.instructions.get_mut(step).unwrap();
            instruction.operation = swapped;
            program = program.run(); // continue the normal run
            if program.instructions.get(program.cursor).is_none() {
                return Some(program.accumulator);
            }
        }
    }
    None
}
