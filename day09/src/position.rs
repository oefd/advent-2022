#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub row: isize,
    pub col: isize,
}

impl From<(isize, isize)> for Position {
    fn from(pos: (isize, isize)) -> Self {
        Position {
            row: pos.0,
            col: pos.1,
        }
    }
}

impl Position {
    pub fn apply_offset(&mut self, offset: (isize, isize)) {
        self.row += offset.0;
        self.col += offset.1;
    }

    pub fn is_adjacent(&self, other: &Self) -> bool {
        let adjacent_rows = other.row - 1..=other.row + 1;
        let adjacent_cols = other.col - 1..=other.col + 1;
        adjacent_rows.contains(&self.row) && adjacent_cols.contains(&self.col)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adjacency() {
        let pos = Position { row: 0, col: 0 };
        assert!(pos.is_adjacent(&(0, 0).into()));
        assert!(pos.is_adjacent(&(1, 0).into()));
        assert!(pos.is_adjacent(&(1, 1).into()));
        assert!(pos.is_adjacent(&(0, 1).into()));
        assert!(pos.is_adjacent(&(-1, 1).into()));
        assert!(pos.is_adjacent(&(-1, 0).into()));
        assert!(pos.is_adjacent(&(-1, -1).into()));
        assert!(pos.is_adjacent(&(0, -1).into()));
        assert!(!pos.is_adjacent(&(0, 2).into()));
        assert!(!pos.is_adjacent(&(1, 2).into()));
        assert!(!pos.is_adjacent(&(-2, 0).into()));
    }
}
