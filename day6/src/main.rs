use std::collections::HashSet;
use std::fs::read_to_string;

fn read_input(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap_or_else(|_| panic!("No file named {} found", filename))
        .split("\n\n")
        .map(str::to_owned)
        .collect()
}

fn find_what_everyone_answered(answers: Vec<String>) -> Vec<HashSet<char>> {
    answers
        .iter()
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

    let answers_anyone = find_what_everyone_answered(answers);

    let total_answers_anyone: usize = answers_anyone
        .iter()
        .map(|group_answers| group_answers.len())
        .sum();

    println!("Total answer count {}", &total_answers_anyone);
}
