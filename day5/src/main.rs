use crate::types::Seat;
use std::fs::read_to_string;

mod types;

fn read_input(filename: &str) -> Vec<Seat> {
    read_to_string(filename)
        .expect("No file named \"input.txt\" found")
        .split('\n')
        .map(|seat_as_string| seat_as_string.parse().expect("Seat is bad"))
        .collect()
}

fn main() {
    let seats = read_input("input.txt");
    seats.iter().for_each(|seat| println!("{:?}", seat))
}
