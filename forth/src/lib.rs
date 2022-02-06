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
//
// TODO:
// - Make a real parser.

/// A toy Forth stack machine with a Virtual machine
use std::{collections::HashMap, fmt::Display};

pub type Value = i16;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<i16>,
    call_table: HashMap<String, usize>,
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
        let over = *stack.iter().rev().nth(1).ok_or(Error::StackUnderflow)?;
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
        let mut call_pc = Vec::with_capacity(128);
        loop {
            let op = if let Some(op) = actual_memory.get(pc) {
                op
            } else {
                return Ok(());
            };
            // println!(
            //     "OPCODE: {:?}, STACK_TOP: {:?}, PC: {:?}, ACTUAL_MEMORY: {:p}",
            //     op,
            //     self.stack.last(),
            //     pc,
            //     actual_memory
            // );
            match *op {
                Bytecode::Return => {
                    pc = call_pc.pop().ok_or(Error::StackUnderflow)?;
                    actual_memory = call_stack.pop().ok_or(Error::StackUnderflow)?;
                }
                Bytecode::Call(addr) => {
                    call_stack.push(actual_memory);
                    call_pc.push(pc);
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

    fn compile_single(&self, instr: &str) -> std::result::Result<Bytecode, Error> {
        use Bytecode as B;
        // println!("compile_single: {:?}", instr);
        // println!("call_table {:?}", self.call_table);
        let possible_fun = self.call_table.get(instr);
        let ret = match instr {
            _ if possible_fun.is_some() => {
                if let Some(&fun_addr) = possible_fun {
                    Bytecode::Call(fun_addr as u16)
                } else {
                    return Err(Error::UnknownWord)
                }
            },
            "+" => Bytecode::Add,
            "-" => Bytecode::Sub,
            "/" => Bytecode::Div,
            "*" => Bytecode::Mul,
            "dup" => Bytecode::Dup,
            "drop" => Bytecode::Pop,
            "swap" => Bytecode::Swap,
            "over" => Bytecode::Over,
            _ if instr.chars().all(|b| b.is_digit(10)) => {
                let num = instr.parse().map_err(|_| Error::InvalidWord)?;
                B::Push(num)
            },
            _ => return Err(Error::UnknownWord),
        };
        Ok(ret)
    }

    pub fn compile(&mut self, input: &str) -> Result {
        let mut instructions = input.split_whitespace();
        while let Some(instruction) = instructions.next() {
            if instruction == ":" {
                let instructions = &mut instructions;
                let name = instructions.next().ok_or(Error::InvalidWord)?;
                let name = name.to_ascii_lowercase();

                if name.chars().all(|b| b.is_digit(10)) {
                    return Err(Error::InvalidWord);
                }
                let mut not_found_semi = false;
                let iter = instructions.take_while(|&c| {
                    not_found_semi = c != ";";
                    not_found_semi
                });

                assert!(self.functions_mem.len() < 2_usize.pow(16));
                let callsite = self.functions_mem.len();

                let mut count = 0;
                for instruction in iter {
                    count += 1;
                    let instruction = &*instruction.to_ascii_lowercase();
                    self.functions_mem.push(self.compile_single(instruction)?);
                }
                // Not optimal
                if not_found_semi || count < 1 {
                    return Err(Error::InvalidWord);
                }

                self.functions_mem.push(Bytecode::Return);
                self.call_table.insert(name, callsite);
                continue;
            }

            self.main_mem
                .push(self.compile_single(&*instruction.to_ascii_lowercase())?);
        }

        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result {
        // println!("PROGRAM {}", input);
        self.compile(input)?;
        // println!("main  memory:{:?}", self.main_mem);
        // println!("funct memory:{:?}", self.functions_mem);
        let result = self.exec();
        if result.is_err() {
            self.stack.clear();
        }
        result
    }
}

#[derive(Debug)]
enum Bytecode {
    Return,
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
