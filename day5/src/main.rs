use crate::types::Seat;
use std::collections::HashSet;
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

    let highest_seat_id = seats.iter().map(Seat::id).max().unwrap();
    println!(
        "The highest ID on a boarding pass is: {:?}",
        highest_seat_id
    );

    let lowest_seat_id = seats.iter().map(Seat::id).min().unwrap();
    let listed_seat_ids: HashSet<_> = seats.iter().map(Seat::id).collect();

    let my_id = (lowest_seat_id..=highest_seat_id)
        .find(|id| !listed_seat_ids.contains(id))
        .unwrap();

    println!("My id is {}", my_id);
}
