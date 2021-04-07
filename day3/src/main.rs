use std::fs::read_to_string;
use std::str::FromStr;

const INPUT_FILENANME: &str = "input.txt";
const X_STEP: usize = 3;
const Y_STEP: usize = 1;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Tree,
    Free,
}

impl FromStr for Cell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Cell::Tree),
            "." => Ok(Cell::Free),
            _ => Err("not a valid forest map".to_string()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Forest {
    pattern: Vec<Vec<Cell>>,
}

impl Forest {
    fn new(pattern: Vec<Vec<Cell>>) -> Self {
        Forest { pattern }
    }

    fn step_and_count_trees(self) -> u32 {
        self.into_iter().fold(0, |count, x| {
            count
                + match x {
                    (Cell::Tree, _) => 1,
                    _ => 0,
                }
        })
    }
}

impl IntoIterator for Forest {
    type Item = (Cell, (usize, usize));
    type IntoIter = ForestIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        let y_size = self.pattern.len();
        let x_size = self.pattern[0].len();

        let pattern_size = (x_size, y_size);

        ForestIntoIterator {
            forest: self,
            pattern_size,
            x: 0,
            y: 0,
        }
    }
}

struct ForestIntoIterator {
    forest: Forest,
    pattern_size: (usize, usize),
    x: usize,
    y: usize,
}

impl Iterator for ForestIntoIterator {
    type Item = (Cell, (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        self.x += X_STEP;
        self.y += Y_STEP;

        match self
            .forest
            .pattern
            // We can iterate right forever, but not down
            .get(self.y)
            .and_then(|line| line.get(self.x % self.pattern_size.0).cloned())
        {
            Some(cell) => Some((cell, (self.x, self.y))),
            None => None,
        }
    }
}

impl FromStr for Forest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|chr| chr.to_string().parse().unwrap())
                    .collect()
            })
            .collect();

        Ok(Forest::new(pattern))
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;
    use Cell::*;

    #[test]
    fn test_map_iter() {
        let _full_map = indoc! {"
            ..##.........##.........##.........##.........##.........##.......
            #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
            .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
            ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
            .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
            ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....
            .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
            .#........#.#........X.#........#.#........#.#........#.#........#
            #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
            #...##....##...##....##...#X....##...##....##...##....##...##....#
            .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#
        "};

        let correct_answer = vec![
            (Free, (3, 1)),
            (Tree, (6, 2)),
            (Free, (9, 3)),
            (Tree, (12, 4)),
            (Tree, (15, 5)),
            (Free, (18, 6)),
            (Tree, (21, 7)),
            (Tree, (24, 8)),
            (Tree, (27, 9)),
            (Tree, (30, 10)),
        ];

        let map = indoc! {"
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        "};

        let forest = Forest::from_str(map).unwrap();

        let answer: Vec<(Cell, (usize, usize))> = forest.into_iter().collect();

        assert_eq!(answer, correct_answer);

        assert_eq!(
            find_correct_positions_in_full_map(_full_map),
            correct_answer
        );
        assert_eq!(
            find_correct_positions_in_full_map_no_iterators(_full_map),
            correct_answer
        );
    }

    #[test]
    fn test_step_and_count_trees() {
        let _full_map = indoc! {"
            ..##.........##.........##.........##.........##.........##.......
            #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
            .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
            ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
            .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
            ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....
            .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
            .#........#.#........X.#........#.#........#.#........#.#........#
            #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
            #...##....##...##....##...#X....##...##....##...##....##...##....#
            .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#
        "};

        let map = indoc! {"
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        "};

        let forest = Forest::from_str(map).unwrap();

        assert_eq!(forest.step_and_count_trees(), 7);
    }

    #[test]
    fn test_pass_from_str() {
        let cases = vec![
            ("####", Forest::new(vec![vec![Tree, Tree, Tree, Tree]])),
            (
                "..##.#",
                Forest::new(vec![vec![Free, Free, Tree, Tree, Free, Tree]]),
            ),
            (
                indoc! {"
                    ###.
                    ##.#
                "},
                Forest::new(vec![
                    vec![Tree, Tree, Tree, Free],
                    vec![Tree, Tree, Free, Tree],
                ]),
            ),
            ("###.", Forest::new(vec![vec![Tree, Tree, Tree, Free]])),
        ];

        for case in cases {
            assert_eq!(
                Forest::from_str(case.0).unwrap(),
                case.1,
                "Failed to parse {}",
                case.0
            );
        }
    }

    /// I wrote 2 functions to find correct answer for forest iteration
    /// But it was unnecessary as tests should use hardcoded answer...
    /// It was fun to write anyway so here we go
    /// First version of function to find correct positions
    /// Find positions of all `O`s and `X`s in __full__ map
    fn find_correct_positions_in_full_map_no_iterators(
        full_map: &str,
    ) -> Vec<(Cell, (usize, usize))> {
        let mut positions: Vec<(Cell, (usize, usize))> = vec![];

        for (y, line) in full_map.split_whitespace().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                match chr {
                    'O' => positions.push((Free, (x, y))),
                    'X' => positions.push((Tree, (x, y))),
                    _ => {}
                }
            }
        }

        positions
    }

    /// Then I though I may practice in iterators for some time...
    fn find_correct_positions_in_full_map(full_map: &str) -> Vec<(Cell, (usize, usize))> {
        full_map
            .split_whitespace()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, chr)| match chr {
                        'O' => Some((Free, (x, y))),
                        'X' => Some((Tree, (x, y))),
                        _ => None,
                    })
                    .collect::<Vec<(Cell, (usize, usize))>>()
            })
            .collect()
    }
}

fn read_input() -> Forest {
    read_to_string(INPUT_FILENANME).unwrap().parse().unwrap()
}

fn main() {
    let forest = read_input();
    println!("Trees encountered: {}", forest.step_and_count_trees())
}
