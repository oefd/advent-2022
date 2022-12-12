use std::collections::VecDeque;

#[derive(Clone)]
pub struct Monkey {
    pub id: usize,
    pub items: VecDeque<u64>,
    pub inspections: u64,
    op_is_add: bool,
    operand: Option<u64>,
    pub test_factor: u64,
    targets: (usize, usize),
}

impl Monkey {
    pub fn inspect_item(&mut self, mod_by: Option<u64>) -> Option<(u64, usize)> {
        let mut item = self.items.pop_front()?;
        self.inspections += 1;

        item = match (self.op_is_add, self.operand) {
            (true, None) => item + item,
            (false, None) => item * item,
            (true, Some(value)) => item + value,
            (false, Some(value)) => item * value,
        };
        if let Some(mod_by) = mod_by {
            item %= mod_by;
        } else {
            item /= 3;
        }

        let target = if item % self.test_factor == 0 {
            self.targets.0
        } else {
            self.targets.1
        };

        Some((item, target))
    }
}

impl<S: AsRef<str>> From<&[S; 6]> for Monkey {
    fn from(lines: &[S; 6]) -> Self {
        let mut lines = lines.iter();

        let id = lines.next().unwrap().as_ref();
        let id: usize = id[7..8].parse().unwrap();

        let items = lines.next().unwrap().as_ref();
        let (_, list) = items.split_at(18); // right after `items: `
        let items: VecDeque<u64> = list
            .split(", ")
            .map(|chars| chars.parse::<u64>().unwrap())
            .collect();

        let op = lines.next().unwrap().as_ref();
        let op = &op[23..]; // right after `= old `
        let (operator, operand) = op.split_once(' ').unwrap();
        let op_is_add = operator == "+";
        let operand = if operand == "old" {
            None
        } else {
            Some(operand.parse::<u64>().unwrap())
        };

        let test = lines.next().unwrap().as_ref();
        let test = &test[21..]; // right after `divisible by `
        let test_factor = test.parse::<u64>().unwrap();

        let targets = (
            lines.next().unwrap().as_ref(),
            lines.next().unwrap().as_ref(),
        );
        let targets = (
            targets.0[29..].parse::<usize>().unwrap(),
            targets.1[30..].parse::<usize>().unwrap(),
        );

        assert!(lines.next().is_none());
        Self {
            id,
            items,
            inspections: 0,
            op_is_add,
            operand,
            test_factor,
            targets,
        }
    }
}
