use std::collections::HashSet;

mod direction;
mod position;

use direction::Direction;
use position::Position;

fn main() {
    let stdin = util::Stdin::new();
    let lines = stdin.cleaned_lines();

    let mut rope1 = KnottedRope::<2>::default();
    let mut part_1_positions: HashSet<Position> = HashSet::new();
    let mut rope2 = KnottedRope::<10>::default();
    let mut part_2_positions: HashSet<Position> = HashSet::new();

    part_1_positions.insert(rope1.knots[1]);
    part_2_positions.insert(rope2.knots[9]);

    for line in lines {
        let (dir, count) = line.split_once(" ").unwrap();
        let dir = Direction::from(dir);
        let count = count.parse::<usize>().unwrap();

        for _ in 0..count {
            rope1.move_head(&dir);
            part_1_positions.insert(rope1.knots[1]);

            rope2.move_head(&dir);
            part_2_positions.insert(rope2.knots[9]);
        }
    }
    println!(
        " 2 knotted tail has visited {} unique positions",
        part_1_positions.len()
    );
    println!(
        "10 knotted tail has visited {} unique positions",
        part_2_positions.len()
    );
}

#[derive(Debug)]
struct KnottedRope<const N: usize> {
    knots: [Position; N],
}

impl<const N: usize> Default for KnottedRope<N> {
    fn default() -> Self {
        Self {
            knots: [Position::default(); N],
        }
    }
}

impl<const N: usize> KnottedRope<N> {
    pub fn move_head(&mut self, dir: &Direction) {
        self.knots[0].apply_offset(dir.offset());
        for i in 1..N {
            let (headward, current) = {
                let (start, end) = self.knots.split_at_mut(i);
                (&start[i - 1], &mut end[0])
            };
            if !current.is_adjacent(headward) {
                *current = Self::tail_chase(&headward, &current)
            }
        }
    }

    pub fn tail_chase(head: &Position, tail: &Position) -> Position {
        let new_row = match tail.row.abs_diff(head.row) {
            0 => tail.row,
            1 => head.row,
            2 => isize::max(tail.row, head.row) - 1,
            _ => unreachable!(),
        };
        let new_col = match tail.col.abs_diff(head.col) {
            0 => tail.col,
            1 => head.col,
            2 => isize::max(tail.col, head.col) - 1,
            _ => unreachable!(),
        };
        Position::from((new_row, new_col))
    }
}
