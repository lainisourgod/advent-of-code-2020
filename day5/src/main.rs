use crate::types::Seat;
use std::fs::read_to_string;

mod types;

fn read_input(filename: &str) -> Vec<Seat> {
    read_to_string(filename)
        .expect("No file named \"input.txt\" found")
        .split_whitespace()
        .enumerate()
        .map(|(line, seat_as_string)| {
            seat_as_string.parse().unwrap_or_else(|err| {
                panic!("Seat {} at line {} is bad: {}", seat_as_string, line, err)
            })
        })
        .collect()
}

fn main() {
    let seats = read_input("input.txt");

    let answer_1 = seats.iter().map(Seat::id).max();
    println!("The highest ID on a boarding pass is: {:?}", answer_1);
}
