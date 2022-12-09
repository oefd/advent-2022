pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            other => panic!("invalid direction `{}`", other),
        }
    }
}

impl Direction {
    pub fn offset(&self) -> (isize, isize) {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }
}
