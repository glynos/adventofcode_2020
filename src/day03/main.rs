use std::ops::Index;
use std::fs;

struct Row {
    values: Vec<char>,
}

impl Row {
    fn count(&self) -> usize { self.values.len() }
}

impl Index<usize> for Row {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output { &self.values[index % self.count()] }
}

struct RollingMap {
    rows: Vec<Row>,
}

impl RollingMap {
    fn load(input: &str) -> RollingMap {
        let mut rows = Vec::<Row>::new();
        for row in input.lines() {
            rows.push(Row { values: row.chars().collect::<Vec<char>>() });
        }
        RollingMap { rows }
    }

    fn row_count(&self) -> usize {
        self.rows.len()
    }

    fn count_trees(&self, row_step: usize, column_step: usize) -> usize {
        let mut tree_count = 0;
        for i in 0..(self.row_count() / row_step) {
            if self[i * row_step][i * column_step] == '#' {
                tree_count += 1;
            }
        }
        tree_count
    }
}

impl Index<usize> for RollingMap {
    type Output = Row;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Unable to read input.txt");

    let map = RollingMap::load(&contents);

    println!("{}", map.count_trees(1, 3));

    let steps = vec!(
        (1 as usize, 1 as usize),
        (1 as usize, 3 as usize),
        (1 as usize, 5 as usize),
        (1 as usize, 7 as usize),
        (2 as usize, 1 as usize),
    );
    let tree_count_product: usize = steps
        .into_iter()
        .map(|(row_step, column_step)| map.count_trees(row_step, column_step))
        .product()
    ;
    println!("{}", tree_count_product);
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_load_map() {
        let map = RollingMap::load(INPUT);
        assert_eq!(map.row_count(), 11);
        assert_eq!(map[0][0], '.');
        assert_eq!(map[0][3], '#');
        assert_eq!(map[1][3], '.');
    }

    #[test]
    fn test_index_wraps_around() {
        let map = RollingMap::load(INPUT);
        assert_eq!(map.row_count(), 11);
        assert_eq!(map[0][11], '.');
        assert_eq!(map[0][13], '#');
    }
}
