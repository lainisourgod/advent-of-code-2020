use std::fs::read_to_string;
use std::str::FromStr;

const INPUT_FILENANME: &str = "input.txt";

#[derive(Debug, Clone, Eq, PartialEq)]
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

        Ok(Forest { pattern })
    }
}

#[test]
fn test_pass_from_str() {
    use Cell::*;

    let cases = vec![
        (
            "####",
            Forest {
                pattern: vec![vec![Tree, Tree, Tree, Tree]],
            },
        ),
        (
            "..##.#",
            Forest {
                pattern: vec![vec![Free, Free, Tree, Tree, Free, Tree]],
            },
        ),
        (
            "###.\n##.#",
            Forest {
                pattern: vec![vec![Tree, Tree, Tree, Free], vec![Tree, Tree, Free, Tree]],
            },
        ),
        (
            "###.",
            Forest {
                pattern: vec![vec![Tree, Tree, Tree, Free]],
            },
        ),
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

fn read_input() -> Forest {
    read_to_string(INPUT_FILENANME).unwrap().parse().unwrap()
}

fn main() {
    let input = read_input();
    println!("{:?}", input);
}
