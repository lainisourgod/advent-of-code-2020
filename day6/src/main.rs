use std::collections::HashSet;
use std::fs::read_to_string;

fn read_input(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap_or_else(|_| panic!("No file named {} found", filename))
        .split("\n\n")
        .map(str::to_owned)
        .collect()
}

fn find_what_anyone_answered(answers: &[String]) -> Vec<HashSet<char>> {
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

fn find_what_everyone_answered(answers: &[String]) -> Vec<HashSet<char>> {
    answers
        .iter()
        .map(|group_answers| {
            let mut group_answers: Vec<HashSet<char>> = group_answers
                .split_whitespace()
                .map(|person_answers| person_answers.chars().collect::<HashSet<char>>())
                .collect();

            let mut result = group_answers.pop().unwrap();
            result.retain(|item| {
                group_answers
                    .iter()
                    .all(|person_answers| person_answers.contains(item))
            });
            result
        })
        .collect()
}

fn main() {
    let answers = read_input("input.txt");

    let answers_anyone = find_what_anyone_answered(&answers);

    let total_answers_anyone: usize = answers_anyone
        .iter()
        .map(|group_answers| group_answers.len())
        .sum();
    println!("Total anyone answer count {}", &total_answers_anyone);

    let answers_everyone = find_what_everyone_answered(&answers);

    let total_answers_everyone: usize = answers_everyone
        .iter()
        .map(|group_answers| group_answers.len())
        .sum();
    println!("Total everyone answer count {}", &total_answers_everyone);
}
