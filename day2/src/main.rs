use std::num::ParseIntError;
use std::str::FromStr;
use std::{fs::read_to_string, ops::RangeInclusive};

const INPUT_FILENANME: &str = "input.txt";

#[derive(Debug, PartialEq)]
struct PasswordEntry {
    letter: char,
    range: RangeInclusive<usize>,
    password: String,
}

impl PasswordEntry {
    fn is_valid(&self) -> bool {
        match self.password.chars().map(|x| x == self.letter).count() {
            x if self.range.contains(&x) => true,
            _ => false,
        }
    }
}

#[test]
fn test_pass_is_valid() {
    let cases = vec![
        (
            PasswordEntry {
                letter: 'g',
                range: 5..=9,
                password: "ggccggmgn".to_owned(),
            },
            true,
        ),
        (
            PasswordEntry {
                letter: 'l',
                range: 11..=16,
                password: "llllqllllllllflq".to_owned(),
            },
            true,
        ),
        (
            PasswordEntry {
                letter: 'r',
                range: 1..=2,
                password: "rrrr".to_owned(),
            },
            false,
        ),
    ];

    for case in cases {
        assert_eq!(case.0.is_valid(), case.1);
    }
}

impl FromStr for PasswordEntry {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        // First part
        let range_borders: Vec<usize> = tokens[0]
            .split('-')
            .map(|num| num.parse().unwrap())
            .collect();
        let range = RangeInclusive::new(range_borders[0], range_borders[1]);

        // Second part
        let letter = tokens[1].chars().nth(0).unwrap();

        // Third part
        let password = tokens[2].to_string();

        Ok(PasswordEntry {
            range,
            letter,
            password,
        })
    }
}

#[test]
fn test_pass_from_str() {
    let cases = vec![
        (
            "5-9 g: ggccggmgn",
            PasswordEntry {
                letter: 'g',
                range: 5..=9,
                password: "ggccggmgn".to_owned(),
            },
        ),
        (
            "11-16 l: llllqllllllllflq",
            PasswordEntry {
                letter: 'l',
                range: 11..=16,
                password: "llllqllllllllflq".to_owned(),
            },
        ),
        (
            "1-2 r: rrrr",
            PasswordEntry {
                letter: 'r',
                range: 1..=2,
                password: "rrrr".to_owned(),
            },
        ),
    ];

    for case in cases {
        assert_eq!(PasswordEntry::from_str(case.0).unwrap(), case.1);
    }
}

fn read_input() -> Vec<PasswordEntry> {
    read_to_string(INPUT_FILENANME)
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn main() {
    let input = read_input();
    println!(
        "Valid passwords count: {}",
        input.iter().map(|entry| entry.is_valid()).count()
    )
}
