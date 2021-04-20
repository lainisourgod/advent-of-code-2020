use std::{ops::Range, str::FromStr};

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    pub fn id(&self) -> usize {
        self.row * 8 + self.column
    }
}

#[derive(Debug, Clone)]
struct SeatRange {
    rows: Range<usize>,
    columns: Range<usize>,
}

impl Default for SeatRange {
    fn default() -> Self {
        SeatRange {
            rows: 0..128,
            columns: 0..8,
        }
    }
}

impl SeatRange {
    fn calculate_next(&self, movement: &char) -> Self {
        match movement {
            'B' => SeatRange {
                rows: (self.rows.end / 2)..(self.rows.end),
                columns: self.columns.clone(),
            },
            'F' => SeatRange {
                rows: (self.rows.start)..(self.rows.end / 2),
                columns: self.columns.clone(),
            },
            'L' => SeatRange {
                rows: (self.rows.end / 2)..(self.rows.end),
                columns: (self.columns.start)..(self.columns.end / 2),
            },
            'R' => SeatRange {
                rows: (self.rows.end / 2)..(self.rows.end),
                columns: (self.columns.end / 2)..(self.columns.end),
            },
            _ => unreachable!(),
        }
    }
}

impl FromStr for Seat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seat_range = s.chars().enumerate().try_fold(
            SeatRange::default(),
            |seat_range, (index, movement)| {
                if !matches!(movement, 'B' | 'F' | 'L' | 'R') {
                    Err(format!("Bad movement {} at index {}", movement, index))
                } else {
                    dbg!(index, &seat_range, &movement);
                    Ok(seat_range.calculate_next(&movement))
                }
            },
        )?;

        assert!(
            seat_range.rows.is_empty() && seat_range.columns.is_empty(),
            "Read all movements but range is not empty: {:?}",
            seat_range
        );

        Ok(Seat {
            row: seat_range.rows.start,
            column: seat_range.columns.start,
        })
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
    fn id_of_seat(seat: Seat) -> usize {
        seat.id()
    }
}
