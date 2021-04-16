mod types;

use crate::types::Passport;
use std::fs::read_to_string;

fn count_valid_passports_1(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.is_valid_1())
        .count()
}

fn count_valid_passports_2(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.is_valid_2())
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
