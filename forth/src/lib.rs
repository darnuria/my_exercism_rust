pub type Value = i16;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<i16>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::with_capacity(128),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        for instr in input.split_whitespace() {
            let instr = instr.as_bytes();
            match instr {
                b"+" => {
                    let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(a + b);
                }
                b"-" => {
                    let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(b - a);
                }
                b"/" => {
                    let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack
                        .push(b.checked_div(a).ok_or(Error::DivisionByZero)?);
                }
                b"*" => {
                    let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(a * b);
                }
                b"DUP" | b"dup" => {
                    let d = *self.stack.last().ok_or(Error::StackUnderflow)?;
                    self.stack.push(d);
                }
                b"drop" | b"DROP" => {
                    self.stack.pop().ok_or(Error::StackUnderflow)?;
                }
                b"swap" | b"SWAP" => {
                    let len = self.stack.len();
                    let penult_index = len.checked_sub(2).ok_or(Error::StackUnderflow)?;
                    let last_index = len.checked_sub(1).ok_or(Error::StackUnderflow)?;
                    // Awkard overflow check
                    {
                        self.stack.get(penult_index).ok_or(Error::StackUnderflow)?;
                        self.stack.last().ok_or(Error::StackUnderflow)?;
                    }
                    self.stack.swap(last_index, penult_index);
                }
                b"over" | b"Over" => {
                    self.stack.last().ok_or(Error::StackUnderflow)?;
                    let penult_index = self
                        .stack
                        .len()
                        .checked_sub(2)
                        .ok_or(Error::StackUnderflow)?;
                    let penult = *self.stack.get(penult_index).ok_or(Error::StackUnderflow)?;
                    self.stack.push(penult);
                }
                word if word.iter().all(|&b| (b as char).is_digit(10)) => {
                    let utf8ed_number = unsafe { std::str::from_utf8_unchecked(word) };
                    self.stack
                        .push(utf8ed_number.parse().map_err(|_| Error::InvalidWord)?);
                }
                b":" => unimplemented!("custom instr not implemented"),
                _ => return Err(Error::UnknownWord),
            }
        }
        Ok(())
    }
}
