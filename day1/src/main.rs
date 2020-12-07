use itertools::Itertools;
use std::fs::read_to_string;

const INPUT_FILENANME: &str = "input.txt";

fn read_input() -> Vec<u32> {
    read_to_string(INPUT_FILENANME)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

#[test]
fn test_find_n_that_sum_up_to() {
    #[rustfmt::skip]
    let cases = vec![
        (
            (3, vec![1, 2, 3], 2),
            vec![1, 2]
        ),
        (
            (6, vec![1, 5, 7, 2, 3], 3),
            vec![1, 2, 3]
        ),
    ];

    for case in cases {
        assert_eq!(find_n_that_sum_up_to(case.0.0, &case.0.1, case.0.2).unwrap(), case.1)
    }
}

fn find_n_that_sum_up_to(to: u32, input: &[u32], n: u32) -> Result<Vec<u32>, String> {
    input
        .iter()
        .combinations(n as usize)
        .find(|x| x.iter().map(|&&x| x).sum::<u32>() == to)
        .map(|x| x.into_iter().cloned().collect())
        .ok_or(format!("No two numbers sum up to {}", to).to_owned())
}

fn main() {
    let input = read_input();
    match find_n_that_sum_up_to(2020, &input, 2) {
        Ok(numbers) => println!(
            "First: {}, Second: {}, Answer: {}",
            numbers[0],
            numbers[1],
            numbers[0] * numbers[1]
        ),
        Err(err) => eprintln!("{}", err),
    };

    match find_n_that_sum_up_to(2020, &input, 3) {
        Ok(numbers) => println!(
            "First: {}, Second: {}, Third: {}, Answer: {}",
            numbers[0],
            numbers[1],
            numbers[2],
            numbers[0] * numbers[1] * numbers[2]
        ),
        Err(err) => eprintln!("{}", err),
    };
}
