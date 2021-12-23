// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

enum Instr {
    Forward,
    Left,
    Right,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        use Direction::*;
        let d = match self.d {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        Robot { d, ..self }
    }

    pub fn turn_left(self) -> Self {
        use Direction::*;
        let d = match self.d {
            North => West,
            West => South,
            South => East,
            East => North,
        };
        Robot { d, ..self }
    }

    pub fn advance(self) -> Self {
        let (x, y) = match self.d {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        };
        Robot {
            x: self.x + x,
            y: self.y + y,
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        use Instr::*;
        let instructions = instructions.as_bytes();
        instructions
            .iter()
            .filter_map(|instr| match instr {
                b'A' => Some(Forward),
                b'R' => Some(Right),
                b'L' => Some(Left),
                _ => None,
            })
            .fold(self, |acc, i| match i {
                Forward => acc.advance(),
                Left => acc.turn_left(),
                Right => acc.turn_right(),
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
