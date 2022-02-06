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
    call_table: HashMap<[u8; 4], (usize, usize)>,
    main_mem: Vec<Bytecode>,
    functions_mem: Vec<Bytecode>,
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
            call_table: HashMap::with_capacity(8),
            stack: Vec::with_capacity(128),
            main_mem: Vec::with_capacity(64),
            functions_mem: Vec::with_capacity(64),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn dup(stack: &mut Vec<i16>) -> Result {
        let d = *stack.last().ok_or(Error::StackUnderflow)?;
        stack.push(d);
        Ok(())
    }

    fn over(stack: &mut Vec<i16>) -> Result {
        let over = *stack
            .iter()
            .rev()
            .skip(1)
            .next()
            .ok_or(Error::StackUnderflow)?;
        assert!(stack.len() >= 2);
        stack.push(over);
        Ok(())
    }

    fn swap(stack: &mut Vec<i16>) -> Result {
        let len = stack.len();
        let penult_index = len.checked_sub(2).ok_or(Error::StackUnderflow)?;
        let last_index = len.checked_sub(1).ok_or(Error::StackUnderflow)?;
        // Awkard overflow check
        {
            stack.get(penult_index).ok_or(Error::StackUnderflow)?;
            stack.last().ok_or(Error::StackUnderflow)?;
        }
        stack.swap(last_index, penult_index);
        Ok(())
    }

    fn exec(&mut self) -> Result {
        let mut pc = 0_usize;
        let mut actual_memory = &self.main_mem;
        let mut call_stack = Vec::with_capacity(128);
        loop {
            let op = if let Some(op) = actual_memory.get(pc) {
                op
            } else {
                return Ok(());
            };
            println!(
                "OPCODE: {:?}, STACK_TOP: {:?}, PC: {:?}, ACTUAL_MEMORY: {:p}",
                op,
                self.stack.last(),
                pc,
                actual_memory
            );
            match *op {
                Bytecode::Noop => continue,
                Bytecode::Return(addr) => {
                    pc = addr as usize;
                    actual_memory = call_stack.pop().ok_or(Error::StackUnderflow)?;
                    continue;
                }
                Bytecode::Call(addr) => {
                    call_stack.push(actual_memory);
                    pc = addr as usize;
                    actual_memory = &self.functions_mem;
                    continue;
                }
                Bytecode::Pop => {
                    self.stack.pop().ok_or(Error::StackUnderflow)?;
                }
                Bytecode::Dup => {
                    Forth::dup(&mut self.stack)?;
                }
                Bytecode::Swap => {
                    Forth::swap(&mut self.stack)?;
                }
                Bytecode::Over => {
                    Forth::over(&mut self.stack)?;
                }
                Bytecode::Push(val) => self.stack.push(val),
                Bytecode::Add | Bytecode::Sub | Bytecode::Div | Bytecode::Mul => {
                    Forth::exec_binop(&mut self.stack, op)?;
                }
            }
            pc += 1;
        }
    }

    fn exec_binop(stack: &mut Vec<i16>, op: &Bytecode) -> Result {
        let a = stack.pop().ok_or(Error::StackUnderflow)?;
        let b = stack.pop().ok_or(Error::StackUnderflow)?;
        let result = match op {
            Bytecode::Add => a + b,
            Bytecode::Sub => b - a,
            Bytecode::Div => b.checked_div(a).ok_or(Error::DivisionByZero)?,
            Bytecode::Mul => a * b,
            _ => unreachable!(),
        };
        stack.push(result);
        Ok(())
    }

    fn compile_single(
        &self,
        instr: &str,
        instr_buff: &[u8],
    ) -> std::result::Result<Bytecode, Error> {
        use Bytecode as B;
        let ret = match &instr_buff[..instr.len()] {
            b"+" => Bytecode::Add,
            b"-" => Bytecode::Sub,
            b"/" => Bytecode::Div,
            b"*" => Bytecode::Mul,
            b"dup" => Bytecode::Dup,
            b"drop" => Bytecode::Pop,
            b"swap" => Bytecode::Swap,
            b"over" => Bytecode::Over,
            _ if instr.chars().all(|b| b.is_digit(10)) => {
                let num = instr.parse().map_err(|_| Error::InvalidWord)?;
                B::Push(num)
            }
            _ => return Err(Error::UnknownWord),
        };
        Ok(ret)
    }

    pub fn compile(&mut self, input: &str) -> Result {
        let mut instructions = input.split_whitespace();
        while let Some(instruction) = instructions.next() {
            // Happy-path
            // Get an array of four bytes, copy the instruction
            // Then match.
            let instruction = &*instruction.to_ascii_lowercase();
            println!("{}", instruction);
            if instruction == ":" {
                let instructions = &mut instructions;
                let name = instructions.next().ok_or(Error::InvalidWord)?;

                if name.chars().all(|b| b.is_digit(10)) {
                    return Err(Error::InvalidWord);
                }
                let iter = instructions.take_while(|&c| c != ";");

                assert!(self.functions_mem.len() <= 2_usize.pow(16) - 1);
                let callsite = self.functions_mem.len();

                let mut count = 0;
                for instruction in iter {
                    count += 1;
                    let mut instr_buff = [0_u8; 4];
                    copy_bytes(instruction.as_bytes(), &mut instr_buff);
                    instr_buff.make_ascii_lowercase();
                    self.functions_mem
                        .push(self.compile_single(instruction, &instr_buff)?);
                }
                // Shall be + 1?

                // placeholder changed with a TRICK
                let mut name_buff = [0_u8; 4];
                copy_bytes(name.as_bytes(), &mut name_buff);
                name_buff.make_ascii_lowercase();
                self.functions_mem.push(Bytecode::Noop);
                self.call_table
                    .insert(name_buff, (callsite, self.functions_mem.len() - 1));

                if count < 1 {
                    return Err(Error::InvalidWord);
                }
                continue;
            }

            let mut instr_buff = [0_u8; 4];
            copy_bytes(instruction.as_bytes(), &mut instr_buff);
            instr_buff.make_ascii_lowercase();
            if let Some(&(fun_addr, end_funaddr)) = self.call_table.get(&instr_buff) {
                self.main_mem.push(Bytecode::Call(fun_addr as u16));
                let main_addr = self.main_mem.len() as u16;
                self.functions_mem[end_funaddr] = Bytecode::Return(main_addr);
            } else {
                let mut instr_buff = [0_u8; 4];
                copy_bytes(instruction.as_bytes(), &mut instr_buff);
                instr_buff.make_ascii_lowercase();
                self.main_mem
                    .push(self.compile_single(instruction, &instr_buff)?);
            }
        }

        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result {
        println!("PROGRAM {}", input);
        self.compile(input)?;
        println!("MHHH COMPILED THAT DELICOUS PROGRAM");
        println!("main  memory:{:?}", self.main_mem);
        println!("funct memory:{:?}", self.functions_mem);
        self.exec()
    }
}

#[derive(Debug)]
enum Bytecode {
    Noop,
    Return(u16),
    Call(u16),
    Pop,
    Dup,
    Swap,
    Over,
    Push(i16),
    Add,
    Sub,
    Div,
    Mul,
}

impl Display for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
