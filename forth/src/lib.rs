// Cursed subset of forth with a single-pass-compiler-vm
// Author: Axel Viala <axel.viala@darnuria.eu>
// Licence MIT
// To the dear reader: It's my first virtual machine ever.
// it's cursed to the bones but I'am proud of it.
// Did with with some inspiration from craftingInterpreters book
// Based on the exercism eponym rust exercise.
//
// Some constrain like Eval retaining call_site is needed by exercice.
//
// Maybe will expand it.

/// A toy Forth stack machine with a Virtual machine

use std::{collections::HashMap, fmt::Display};

pub type Value = i16;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<i16>,
    code: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

fn copy_bytes(src: &[u8], dst: &mut [u8]) {
    for (s, d) in src.iter().zip(dst) {
        *d = *s;
    }
}

impl Default for Forth {
    fn default() -> Self {
        Self::new()
    }
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::with_capacity(128),
            code: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }


    fn dup(&mut self) -> Result {
        let d = *self.stack.last().ok_or(Error::StackUnderflow)?;
        self.stack.push(d);
        Ok(())
    }

    fn over(&mut self) -> Result {
        let over = *self
            .stack
            .iter()
            .rev()
            .skip(1)
            .next()
            .ok_or(Error::StackUnderflow)?;
        assert!(self.stack.len() >= 2);
        self.stack.push(over);
        Ok(())
    }

    fn swap(&mut self) -> Result {
        let len = self.stack.len();
        let penult_index = len.checked_sub(2).ok_or(Error::StackUnderflow)?;
        let last_index = len.checked_sub(1).ok_or(Error::StackUnderflow)?;
        // Awkard overflow check
        {
            self.stack.get(penult_index).ok_or(Error::StackUnderflow)?;
            self.stack.last().ok_or(Error::StackUnderflow)?;
        }
        self.stack.swap(last_index, penult_index);
        Ok(())
    }
    fn pop(&mut self) -> Result {
        self.stack.pop().ok_or(Error::StackUnderflow)?;
        Ok(())
    }

    fn bin_operator(&mut self, op: &[u8]) -> Result {
        let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let result = match op {
            b"+" => a + b,
            b"-" => b - a,
            b"/" => b.checked_div(a).ok_or(Error::DivisionByZero)?,
            b"*" => a * b,
            _ => unreachable!(),
        };
        self.stack.push(result);
        Ok(())
    }

    // pub parse_function(&mut self, input: &str) -> Result {

    // }

    fn parse(&mut self, instr: &[u8]) -> Result {
        match instr {
            b"+" | b"-" | b"/" | b"*" => self.bin_operator(instr),
            b"dup" => self.dup(),
            b"drop" => self.pop(),
            b"swap" => self.swap(),
            b"over" => self.over(),
            word if word.iter().all(|&b| (b as char).is_digit(10)) => {
                let utf8ed_number = unsafe { std::str::from_utf8_unchecked(word) };
                self.stack
                    .push(utf8ed_number.parse().map_err(|_| Error::InvalidWord)?);
                    Ok(())
            }
            _ => return Err(Error::UnknownWord),
        }
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut instructions = input.split_whitespace();       
        while let Some(instruction) = instructions.next() {
            // Happy-path
            // Get an array of four bytes, copy the instruction
            // Then match.
            let mut instr_buff = [0_u8; 4];
            copy_bytes(instruction.as_bytes(), &mut instr_buff);
            instr_buff.make_ascii_lowercase();
            let instr = &instr_buff[..instruction.len()];



            if instr == b":" {
                let instructions = &mut instructions;
                let name = instructions.next().ok_or(Error::InvalidWord)?;
                if name.chars().all(|b| b.is_digit(10)) {
                    return Err(Error::InvalidWord);
                }
                let sep = " ";
                let mut count = 0;
                let instructions = instructions
                    .take_while(|&c| { count += 1; c != ";"})
                    .flat_map(|s| s.chars().chain(sep.chars()))
                    //.map(|s| s.to_string())
                    .collect::<_>();
                if count < 2 {
                    return Err(Error::InvalidWord);
                }
                self.code.insert(name.to_ascii_lowercase(), instructions);
                continue;
            }

            if let Some(code) = self.code.get(instruction).cloned() {
                println!("{}", code);
                self.eval(&code)?;
            }
            self.parse(instr)?
        }
        Ok(())
    }
}
