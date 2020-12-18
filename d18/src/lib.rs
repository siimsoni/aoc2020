extern crate btoi;

use btoi::btoi;
use std::io::BufRead;

#[derive(Debug)]
pub enum Token {
    Add,
    Multiply,
    ParenthesesOpen,
    ParenthesesClose,
    Number(i64),
    EndOfLine,
}

pub fn parse<R>(mut reader: R) -> Box<[Token]>
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
            match &c {
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    line_buf[line_len] = *c;
                    line_len += 1;
                }
                _ => {
                    if line_len > 0 {
                        result.push(Token::Number(btoi(&line_buf[..line_len]).unwrap_or(0)));
                        line_len = 0;
                    }
                }
            }

            match &c {
                b'\n' => result.push(Token::EndOfLine),
                b'+' => result.push(Token::Add),
                b'*' => result.push(Token::Multiply),
                b'(' => result.push(Token::ParenthesesOpen),
                b')' => result.push(Token::ParenthesesClose),
                _ => (),
            }
        }
    }
    if line_len > 0 {
        result.push(Token::Number(btoi(&line_buf[..line_len]).unwrap_or(0)));
    }
    result.push(Token::EndOfLine);
    result.into_boxed_slice()
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Expression {
    operation: Operation,
    value: i64,
}

impl Expression {
    fn execute(&mut self, value: &i64) {
        match self.operation {
            Operation::Add => {
                self.value += value;
            }
            Operation::Multiply => {
                self.value *= value;
            }
        }
    }
}

pub fn p1_solve(tokens: &[Token]) -> Option<i64> {
    let mut result = Vec::new();
    result.push(Expression {
        operation: Operation::Add,
        value: 0,
    });
    let mut sum = 0;
    for token in tokens {
        match token {
            Token::Add => {
                result.last_mut().unwrap().operation = Operation::Add;
            }
            Token::Multiply => {
                result.last_mut().unwrap().operation = Operation::Multiply;
            }
            Token::ParenthesesOpen => {
                result.push({
                    Expression {
                        operation: Operation::Add,
                        value: 0,
                    }
                });
            }
            Token::ParenthesesClose => {
                let last_value = result.pop().unwrap().value;
                result.last_mut().unwrap().execute(&last_value);
            }
            Token::Number(value) => {
                result.last_mut().unwrap().execute(value);
            }
            Token::EndOfLine => {
                sum += result.last().unwrap().value;
                result.clear();
                result.push(Expression {
                    operation: Operation::Add,
                    value: 0,
                })
            }
        }
    }

    Some(sum)
}

pub fn p2_solve(tokens: &[Token]) -> Option<i64> {
    let mut tokens_w_precedence = Vec::new();

    tokens_w_precedence.push(Token::ParenthesesOpen);
    tokens_w_precedence.push(Token::ParenthesesOpen);

    // there must be a special place in hell for this... credit to FORTRAN

    for token in tokens {
        match token {
            Token::Add => {
                tokens_w_precedence.push(Token::ParenthesesClose);
                tokens_w_precedence.push(Token::Add);
                tokens_w_precedence.push(Token::ParenthesesOpen);
            }
            Token::Multiply => {
                tokens_w_precedence.push(Token::ParenthesesClose);
                tokens_w_precedence.push(Token::ParenthesesClose);
                tokens_w_precedence.push(Token::Multiply);
                tokens_w_precedence.push(Token::ParenthesesOpen);
                tokens_w_precedence.push(Token::ParenthesesOpen);
            }
            Token::ParenthesesOpen => {
                tokens_w_precedence.push(Token::ParenthesesOpen);
                tokens_w_precedence.push(Token::ParenthesesOpen);
                tokens_w_precedence.push(Token::ParenthesesOpen);
            }
            Token::ParenthesesClose => {
                tokens_w_precedence.push(Token::ParenthesesClose);
                tokens_w_precedence.push(Token::ParenthesesClose);
                tokens_w_precedence.push(Token::ParenthesesClose);
            }
            Token::EndOfLine => {
                tokens_w_precedence.push(Token::ParenthesesClose);
                tokens_w_precedence.push(Token::ParenthesesClose);
                tokens_w_precedence.push(Token::EndOfLine);
                tokens_w_precedence.push(Token::ParenthesesOpen);
                tokens_w_precedence.push(Token::ParenthesesOpen);
            }
            Token::Number(number) => tokens_w_precedence.push(Token::Number(*number)),
        }
    }

    let mut result = Vec::new();
    result.push(Expression {
        operation: Operation::Add,
        value: 0,
    });
    let mut sum = 0;
    for token in tokens_w_precedence {
        match token {
            Token::Add => {
                result.last_mut().unwrap().operation = Operation::Add;
            }
            Token::Multiply => {
                result.last_mut().unwrap().operation = Operation::Multiply;
            }
            Token::ParenthesesOpen => {
                result.push({
                    Expression {
                        operation: Operation::Add,
                        value: 0,
                    }
                });
            }
            Token::ParenthesesClose => {
                let last_value = result.pop().unwrap().value;
                result.last_mut().unwrap().execute(&last_value);
            }
            Token::Number(value) => {
                result.last_mut().unwrap().execute(&value);
            }
            Token::EndOfLine => {
                sum += result.last().unwrap().value;
                result.clear();
                result.push(Expression {
                    operation: Operation::Add,
                    value: 0,
                })
            }
        }
    }

    Some(sum)
}
