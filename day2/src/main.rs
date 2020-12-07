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
        match self.password.chars().filter(|&x| x == self.letter).count() {
            count if self.range.contains(&count) => true,
            _ => false,
        }
    }

    /// Check that exactly one of boundary letters is equal to self.letter
    fn is_valid_2(&self) -> bool {
        self.password
            .chars()
            .enumerate()
            .filter(|(pos, letter)| {
                *letter == self.letter
                    && (*pos == (self.range.start() - 1) || *pos == (self.range.end() - 1))
            })
            .count()
            == 1
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
                letter: 'a',
                range: 1..=3,
                password: "abcde".to_owned(),
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
        assert_eq!(case.0.is_valid(), case.1, "entry is {:?}", case.0);
    }
}

#[test]
fn test_pass_is_valid_2() {
    let cases = vec![
        (
            PasswordEntry {
                letter: 'a',
                range: 1..=3,
                password: "abcde".to_owned(),
            },
            true,
        ),
        (
            PasswordEntry {
                letter: 'b',
                range: 1..=3,
                password: "cdefg".to_owned(),
            },
            false,
        ),
        (
            PasswordEntry {
                letter: 'c',
                range: 2..=9,
                password: "ccccccccc".to_owned(),
            },
            false,
        ),
    ];

    for case in cases {
        assert_eq!(case.0.is_valid_2(), case.1, "entry is {:?}", case.0);
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
        "Valid passwords count:\n\tBy 1st rule {}\n\tBy 2nd rule {}",
        input.iter().filter(|entry| entry.is_valid()).count(),
        input.iter().filter(|entry| entry.is_valid_2()).count()
    )
}
