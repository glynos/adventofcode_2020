use std::ops::Index;
use std::fs;
use reduce::Reduce;

struct Row {
    values: Vec<u8>,
}

impl Row {
    fn count(&self) -> usize {
        self.values.len()
    }
}

impl Index<usize> for Row {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index % self.count()]
    }
}

struct RollingMap {
    rows: Vec<Row>,
}

impl RollingMap {
    fn load(input: &str) -> Option<RollingMap> {
        let mut rows = Vec::<Row>::new();
        for row in input.lines() {
            rows.push(Row { values: row.chars().map(|c| c as u8).collect::<Vec<u8>>() });
        }
        Some(RollingMap { rows })
    }

    fn row_count(&self) -> usize {
        self.rows.len()
    }

    fn count_trees(&self, step: (usize, usize)) -> usize {
        let mut tree_count = 0;
        for i in 0..(self.row_count() / step.0) {
            let char = self[i * step.0][i * step.1];
            if char == '#' as u8 {
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

    let map = RollingMap::load(&contents).unwrap();

    println!("{}", map.count_trees((1, 3)));

    let steps = vec!(
        (1 as usize, 1 as usize),
        (1 as usize, 3 as usize),
        (1 as usize, 5 as usize),
        (1 as usize, 7 as usize),
        (2 as usize, 1 as usize),
    );
    let tree_count_product = steps
        .into_iter()
        .map(|step| map.count_trees(step))
        .reduce(|x, y| x * y);
    println!("{}", tree_count_product.unwrap());
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
        assert!(map.is_some());
        let map = map.unwrap();
        assert_eq!(map.row_count(), 11);
        assert_eq!(map[0][0], '.' as u8);
        assert_eq!(map[0][3], '#' as u8);
        assert_eq!(map[1][3], '.' as u8);
    }

    #[test]
    fn test_index_wraps_around() {
        let map = RollingMap::load(INPUT);
        assert!(map.is_some());
        let map = map.unwrap();
        assert_eq!(map.row_count(), 11);
        assert_eq!(map[0][11], '.' as u8);
        assert_eq!(map[0][13], '#' as u8);
    }
}
