use std::collections::HashSet;
use std::fs::read_to_string;

fn read_input(filename: &str) -> Vec<HashSet<char>> {
    read_to_string(filename)
        .unwrap_or_else(|_| panic!("No file named {} found", filename))
        .split("\n\n")
        .map(|group_answers| {
            group_answers
                .split_whitespace()
                .flat_map(|person_answers| person_answers.chars())
                .collect()
        })
        .collect()
}

fn main() {
    let answers = read_input("input.txt");

    let total_answers: usize = answers
        .iter()
        .map(|group_answers| group_answers.len())
        .sum();

    println!("Total answer count {}", &total_answers);
}
