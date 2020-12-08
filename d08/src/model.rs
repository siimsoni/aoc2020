use std::convert::TryFrom;
use std::str::from_utf8;

#[derive(Debug, Clone)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub operation: Operation,
    value: i32,
    executed: bool,
}

impl Instruction {
    fn execute(&self, cursor: &mut usize, accumulator: &mut i32) {
        match self.operation {
            Operation::Acc => {
                *accumulator += self.value;
                *cursor += 1;
            }
            Operation::Jmp => {
                *cursor = (*cursor as i32).checked_add(self.value).unwrap_or(0) as usize;
            }
            Operation::Nop => {
                *cursor += 1;
            }
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Instruction {
    type Error = &'static str;

    fn try_from(v: &'a [u8]) -> Result<Self, &'static str> {
        if v.len() < 6 {
            return Err("line too short");
        }
        let operation = match &v[..3] {
            b"nop" => Some(Operation::Nop),
            b"jmp" => Some(Operation::Jmp),
            b"acc" => Some(Operation::Acc),
            _ => None,
        };
        let multiplier = match &v[4] {
            b'-' => -1,
            _ => 1,
        };
        let value = from_utf8(&v[5..])
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
            * multiplier;

        if operation.is_none() {
            return Err("unsupported operation");
        }
        Ok(Instruction {
            operation: operation.unwrap(),
            value,
            executed: false,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub cursor: usize,
    pub accumulator: i32,
}

impl Program {
    pub fn run(mut self) -> Self {
        while let Some(mut instruction) = self.instructions.get_mut(self.cursor) {
            if instruction.executed {
                break;
            }
            instruction.executed = true;
            instruction.execute(&mut self.cursor, &mut self.accumulator);
        }
        self
    }
    pub fn run_tracked(mut self, steps: &mut Vec<usize>) -> Self {
        while let Some(mut instruction) = self.instructions.get_mut(self.cursor) {
            if instruction.executed {
                break;
            }
            instruction.executed = true;
            steps.push(self.cursor);
            instruction.execute(&mut self.cursor, &mut self.accumulator);
        }
        self
    }
}
