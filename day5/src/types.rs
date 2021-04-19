use std::str::FromStr;

#[derive(Debug, Default, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use test_case::test_case;

    #[test_case("BFFFBBFRRR", Ok(Seat {row: 70,  column: 7}))]
    #[test_case("FFFBBBFRRR", Ok(Seat {row: 14,  column: 7}))]
    #[test_case("BBFFBBFRLL", Ok(Seat {row: 102, column: 4}))]
    #[test_case("FBFBBFFRLR", Ok(Seat {row: 44,  column: 5}))]
    fn seat_from_str(x: &str, seat: Result<Seat, String>) {
        pretty_assertions::assert_eq!(x.parse(), seat)
    }

    #[test_case(Seat {row: 70,  column: 7} => 567)]
    #[test_case(Seat {row: 14,  column: 7} => 119)]
    #[test_case(Seat {row: 102, column: 4} => 820)]
    #[test_case(Seat {row: 44,  column: 5} => 357)]
    fn id_of_seat(seat: Seat) -> u32 {
        seat.id()
    }
}
