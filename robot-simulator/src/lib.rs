// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next_clockwise(self) -> Self {
        (self as u8 + 1).rem_euclid(4).into()
    }

    fn next_counterclockwise(self) -> Self {
        (self as u8).wrapping_sub(1).rem_euclid(4).into()
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => panic!("Unknown direction: {}.", value),
        }
    }
}

impl From<Direction> for (i32, i32) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    pub fn turn_right(mut self) -> Self {
        self.d = self.d.next_clockwise();
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.d = self.d.next_counterclockwise();
        self
    }

    pub fn advance(mut self) -> Self {
        let (dx, dy) = self.d.into();
        self.x += dx;
        self.y += dy;
        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for chr in instructions.chars() {
            match chr {
                'L' => self = self.turn_left(),
                'R' => self = self.turn_right(),
                'A' => self = self.advance(),
                _ => panic!("Unknown instruction: '{}'.", chr),
            }
        }

        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
