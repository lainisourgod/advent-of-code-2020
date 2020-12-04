use std::fs::read_to_string;

const INPUT_FILENANME: &str = "input.txt";

fn read_input() -> Vec<u32> {
    read_to_string(INPUT_FILENANME)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn find_two_that_sum_up_to(to: u32, input: &[u32]) -> Result<(u32, u32), String> {
    for first in input.iter().take(input.len() - 1) {
        for second in input.iter().skip(1) {
            if first + second == to {
                return Ok((first.clone(), second.clone()));
            }
        }
    }

    return Err(format!("No two numbers sum up to {}", to).to_owned());
}

#[test]
fn test_find_two_that_sum_up_to() {
    #[rustfmt::skip]
    let cases = vec![
        (
            (3, &[1, 2, 3]),
            (1, 2)
        )
    ];

    for case in cases {
        assert_eq!(find_two_that_sum_up_to(case.0.0, case.0.1).unwrap(), case.1)
    }
}

fn main() {
    let input = read_input();
    match find_two_that_sum_up_to(2020, &input) {
        Ok((first, second)) => println!(
            "First: {}, Second: {}, Answer: {}",
            first,
            second,
            first * second
        ),
        Err(err) => eprintln!("{}", err),
    };
}
