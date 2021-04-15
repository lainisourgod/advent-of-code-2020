mod types;

use crate::types::{Field, Passport};
use std::fs::read_to_string;

fn count_valid_passports(passports: Vec<Passport>) -> usize {
    passports
        .into_iter()
        .filter(|passport| passport.is_valid())
        .count()
}

fn read_input(filename: String) -> Vec<Passport> {
    read_to_string(filename)
        .expect("No file named \"input.txt\" found")
        .split("\n\n")
        .map(|passport_as_string| passport_as_string.parse().expect("Passport entry is bad"))
        .collect()
}

fn main() {
    let passports: Vec<Passport> = read_input("input.txt".to_string());
    let answer_1 = count_valid_passports(passports);
    println!("Total valid passports: {}", &answer_1);
}
