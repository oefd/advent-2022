fn main() {
    let stdin = util::Stdin::new();
    let lines = stdin.cleaned_lines();
    let forest = Forest::from_strings(lines);
    let total_visible: usize = forest
        .iter()
        .map(|tree| if forest.is_visible(tree) { 1 } else { 0 })
        .sum();
    println!("{} trees are visible from the outside", total_visible);
}

struct Forest {
    height: usize,
    width: usize,
    trees: Vec<u8>,
}

impl Forest {
    pub fn from_strings(strs: impl Iterator<Item = String>) -> Self {
        let mut strs = strs.peekable();
        let width = strs.peek().unwrap().len();
        let mut trees = vec![];
        let mut height = 0;
        strs.for_each(|s| {
            height += 1;
            let tree_heights = s
                .chars()
                .map(|char| char.to_digit(10).unwrap().try_into().unwrap());
            tree_heights.for_each(|height| trees.push(height));
        });

        Self {
            height,
            width,
            trees,
        }
    }

    pub fn get(&self, row_col: (usize, usize)) -> u8 {
        let (row, col) = row_col;
        assert!(row < self.height);
        assert!(col < self.width);
        let inner_idx = (row * self.width) + col;
        self.trees[inner_idx]
    }

    pub fn lines_of_sight(&self, row_col: (usize, usize)) -> LinesOfSight {
        let (row, col) = row_col;
        assert!(row < self.height);
        assert!(col < self.width);

        LinesOfSight {
            north: (0..row)
                .rev()
                .map(|northern_row| (northern_row, col))
                .collect(),
            south: (row + 1..self.height)
                .map(|southern_row| (southern_row, col))
                .collect(),
            east: (col + 1..self.width)
                .map(|eastern_col| (row, eastern_col))
                .collect(),
            west: (0..col)
                .rev()
                .map(|western_col| (row, western_col))
                .collect(),
        }
    }

    pub fn is_visible(&self, row_col: (usize, usize)) -> bool {
        let (row, col) = row_col;
        assert!(row < self.height);
        assert!(col < self.width);

        let is_edge_row = row == 0 || row == self.height - 1;
        let is_edge_col = col == 0 || col == self.width - 1;

        if is_edge_row || is_edge_col {
            true
        } else {
            let height = self.get(row_col);
            let los = self.lines_of_sight(row_col);

            let visible_north = !los.north.iter().any(|tree| self.get(*tree) >= height);
            let visible_south = !los.south.iter().any(|tree| self.get(*tree) >= height);
            let visible_east = !los.east.iter().any(|tree| self.get(*tree) >= height);
            let visible_west = !los.west.iter().any(|tree| self.get(*tree) >= height);

            visible_north || visible_south || visible_east || visible_west
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let mut rows = 1..self.height;
        let mut current_row = 0;
        let cols = 0..self.width;
        let mut current_col = cols.clone();
        std::iter::from_fn(move || {
            if let Some(next_col) = current_col.next() {
                Some((current_row, next_col))
            } else {
                if let Some(next_row) = rows.next() {
                    current_row = next_row;
                    current_col = cols.clone();
                    Some((current_row, current_col.next().unwrap()))
                } else {
                    None
                }
            }
        })
    }
}

struct LinesOfSight {
    pub north: Vec<(usize, usize)>,
    pub south: Vec<(usize, usize)>,
    pub east: Vec<(usize, usize)>,
    pub west: Vec<(usize, usize)>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn indexing() {
        let forest = Forest::from_strings(
            [
                "2311".to_string(),
                "1141".to_string(),
                "1115".to_string(),
                "1111".to_string(),
            ]
            .into_iter(),
        );
        assert!(forest.get((0, 0)) == 2);
        assert!(forest.get((0, 1)) == 3);
        assert!(forest.get((1, 2)) == 4);
        assert!(forest.get((2, 3)) == 5);
    }

    #[test]
    fn lines_of_sight() {
        let forest = Forest::from_strings(
            [
                "1111".to_string(),
                "1111".to_string(),
                "1111".to_string(),
                "1111".to_string(),
            ]
            .into_iter(),
        );

        let los = forest.lines_of_sight((0, 0));
        assert!(los.north.len() == 0);
        assert!(los.south.len() == 3);
        assert!(los.south[0] == (1, 0));
        assert!(los.south[1] == (2, 0));
        assert!(los.south[2] == (3, 0));
        assert!(los.west.len() == 0);
        assert!(los.east.len() == 3);
        assert!(los.east[0] == (0, 1));
        assert!(los.east[1] == (0, 2));
        assert!(los.east[2] == (0, 3));

        let los = forest.lines_of_sight((1, 2));
        assert!(los.north.len() == 1);
        assert!(los.north[0] == (0, 2));
        assert!(los.south.len() == 2);
        assert!(los.south[0] == (2, 2));
        assert!(los.south[1] == (3, 2));
        assert!(los.west.len() == 2);
        assert!(los.west[0] == (1, 1));
        assert!(los.west[1] == (1, 0));
        assert!(los.east.len() == 1);
        assert!(los.east[0] == (1, 3));
    }

    #[test]
    fn is_visible() {
        let forest = Forest::from_strings(
            [
                "1111".to_string(),
                "1121".to_string(),
                "1111".to_string(),
                "1111".to_string(),
            ]
            .into_iter(),
        );

        assert!(forest.is_visible((0, 0)));
        assert!(forest.is_visible((0, 3)));
        assert!(forest.is_visible((1, 0)));
        assert!(forest.is_visible((3, 0)));
        assert!(!forest.is_visible((1, 1)));
        assert!(forest.is_visible((1, 2)));
    }

    #[test]
    fn forest_iter() {
        let forest = Forest::from_strings(["111".to_string(), "111".to_string()].into_iter());

        let mut iter = forest.iter();
        assert!(iter.next().unwrap() == (0, 0));
        assert!(iter.next().unwrap() == (0, 1));
        assert!(iter.next().unwrap() == (0, 2));
        assert!(iter.next().unwrap() == (1, 0));
        assert!(iter.next().unwrap() == (1, 1));
        assert!(iter.next().unwrap() == (1, 2));
        assert!(iter.next().is_none());
    }
}
