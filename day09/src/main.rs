use std::collections::HashSet;

mod direction;
mod position;

use direction::Direction;
use position::Position;

fn main() {
    let stdin = util::Stdin::new();
    let lines = stdin.cleaned_lines();

    let mut tail_positions: HashSet<Position> = HashSet::new();
    let mut rope = Rope::default();
    tail_positions.insert(rope.tail);
    for line in lines {
        let (dir, count) = line.split_once(" ").unwrap();
        let dir = Direction::from(dir);
        let count = count.parse::<usize>().unwrap();

        for _ in 0..count {
            rope.move_head(&dir);
            tail_positions.insert(rope.tail);
        }
    }

    println!("tail has visited {} unique positions", tail_positions.len());
}

#[derive(Debug, Default)]
struct Rope {
    pub head: Position,
    pub tail: Position,
}

impl Rope {
    pub fn move_head(&mut self, dir: &Direction) {
        let old_head = self.head;
        self.head.apply_offset(dir.offset());
        if !self.tail.is_adjacent(&self.head) {
            self.tail = old_head;
        }
    }
}
