use std::str::FromStr;

#[derive(Debug, Default)]
pub(crate) struct Seat {
    row: u32,
    column: u32,
}

impl Seat {
    pub fn id(&self) -> u32 {
        self.row * 8 + self.column
    }

    pub fn calculate_movement(&self, movement: &char) -> Result<Self, ()> {
        match movement {
            'B' => Ok(Seat {
                row: self.row + 1,
                column: self.column + 1,
            }),
            'F' => Ok(Seat {
                row: self.row + 1,
                column: self.column + 1,
            }),
            'L' => Ok(Seat {
                row: self.row + 1,
                column: self.column + 1,
            }),
            'R' => Ok(Seat {
                row: self.row + 1,
                column: self.column + 1,
            }),
            _ => Err(()),
        }
    }
}

impl FromStr for Seat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seat = s.chars().enumerate().try_fold(
            Seat::default(),
            |searched_seat, (index, movement)| {
                searched_seat
                    .calculate_movement(&movement)
                    .map_err(|()| format!("Bad movement {} at index {}", movement, index))
            },
        )?;

        Ok(seat)
    }
}
