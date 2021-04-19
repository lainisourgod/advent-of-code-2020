use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct Seat {
    row: u32,
    column: u32,
    id: u32,
}

impl FromStr for Seat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row, column) =
            s.chars()
                .enumerate()
                .try_fold((0, 0), |searched_seat, (line, partition)| {
                    match partition {
                        'B' => Ok((searched_seat.0 + 1, searched_seat.1 + 1)),
                        // 'F' =>
                        // 'L' =>
                        // 'R' =>,
                        _ => Err(format!("Bad seat at line {}", line)),
                    }
                })?;

        Ok(Seat {
            row,
            column,
            id: 8 * row + column,
        })
    }
}
