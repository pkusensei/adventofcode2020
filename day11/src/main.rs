use std::collections::HashMap;

use itertools::Itertools;

use tools::Error;

fn main() {
    let input = include_str!(r#"..\input.txt"#);
    let seats = get_seat_map(input).unwrap();
    debug_assert_eq!(2406, reach_stable(seats.clone()));
    let (col_count, row_count) = get_dimensions(input).unwrap();
    debug_assert_eq!(
        2149,
        reach_stable_visible(seats, col_count - 1, row_count - 1)
    );

    println!("All done");
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Status {
    Floor,
    Empty,
    Occupied,
}

impl Status {
    fn new(status: char) -> Result<Self, Error> {
        match status {
            '.' => Ok(Self::Floor),
            'L' => Ok(Self::Empty),
            '#' => Ok(Self::Occupied),
            _ => Err(format!("Invalid status {}", status).into()),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Floor | Self::Empty => true,
            _ => false,
        }
    }
}

fn reach_stable_visible(mut seats: HashMap<(u8, u8), Status>, max_col: u8, max_row: u8) -> usize {
    let new_seats = get_changed_seats_visible(&seats, max_col, max_row);
    if new_seats.is_empty() {
        seats
            .iter()
            .filter(|(_, status)| **status == Status::Occupied)
            .count()
    } else {
        seats.extend(new_seats.into_iter());
        reach_stable_visible(seats, max_col, max_row)
    }
}

fn get_changed_seats_visible(
    seats: &HashMap<(u8, u8), Status>,
    max_col: u8,
    max_row: u8,
) -> HashMap<(u8, u8), Status> {
    seats
        .iter()
        .filter_map(|((col, row), _)| {
            match change_seat_status_visible(*col, *row, max_col, max_row, seats) {
                Some(status) => Some(((*col, *row), status)),
                None => None,
            }
        })
        .collect()
}

fn change_seat_status_visible(
    col: u8,
    row: u8,
    max_col: u8,
    max_row: u8,
    seats: &HashMap<(u8, u8), Status>,
) -> Option<Status> {
    let occupied = count_visible_occupied(col, row, max_col, max_row, seats);
    match seats.get(&(col, row)).cloned().unwrap_or(Status::Floor) {
        Status::Floor => None,
        Status::Empty => match occupied {
            0 => Some(Status::Occupied),
            _ => None,
        },
        Status::Occupied => {
            if occupied >= 5 {
                Some(Status::Empty)
            } else {
                None
            }
        }
    }
}

fn count_visible_occupied(
    col: u8,
    row: u8,
    max_col: u8,
    max_row: u8,
    seats: &HashMap<(u8, u8), Status>,
) -> u8 {
    fn is_seat(col: u8, row: u8, seats: &HashMap<(u8, u8), Status>) -> bool {
        if let Some(status) = seats.get(&(col, row)) {
            *status == Status::Occupied || *status == Status::Empty
        } else {
            false
        }
    }
    fn is_occupied(col: u8, row: u8, seats: &HashMap<(u8, u8), Status>) -> bool {
        if let Some(status) = seats.get(&(col, row)) {
            *status == Status::Occupied
        } else {
            false
        }
    }

    let mut occupied = 0;

    // left
    if (0..col)
        .rev()
        .find(|i| is_seat(*i, row, seats))
        .filter(|i| is_occupied(*i, row, seats))
        .is_some()
    {
        occupied += 1;
    }

    //right
    if (col + 1..=max_col)
        .find(|i| is_seat(*i, row, seats))
        .filter(|i| is_occupied(*i, row, seats))
        .is_some()
    {
        occupied += 1;
    }

    // up
    if (0..row)
        .rev()
        .find(|i| is_seat(col, *i, seats))
        .filter(|i| is_occupied(col, *i, seats))
        .is_some()
    {
        occupied += 1;
    }

    // down
    if (row + 1..=max_row)
        .find(|i| is_seat(col, *i, seats))
        .filter(|i| is_occupied(col, *i, seats))
        .is_some()
    {
        occupied += 1;
    }

    // top-left
    if (0..col)
        .rev()
        .zip((0..row).rev())
        .find(|(x, y)| is_seat(*x, *y, seats))
        .filter(|(x, y)| is_occupied(*x, *y, seats))
        .is_some()
    {
        occupied += 1;
    }

    // top-right
    if (col + 1..=max_col)
        .zip((0..row).rev())
        .find(|(x, y)| is_seat(*x, *y, seats))
        .filter(|(x, y)| is_occupied(*x, *y, seats))
        .is_some()
    {
        occupied += 1;
    }

    // bottom-left
    if (0..col)
        .rev()
        .zip(row + 1..=max_row)
        .find(|(x, y)| is_seat(*x, *y, seats))
        .filter(|(x, y)| is_occupied(*x, *y, seats))
        .is_some()
    {
        occupied += 1;
    }

    // bottom-right
    if (col + 1..=max_col)
        .zip(row + 1..=max_row)
        .find(|(x, y)| is_seat(*x, *y, seats))
        .filter(|(x, y)| is_occupied(*x, *y, seats))
        .is_some()
    {
        occupied += 1;
    }

    occupied
}

fn get_dimensions(input: &str) -> Result<(u8, u8), Error> {
    let row_count = input.split_ascii_whitespace().count();
    let col_count = input
        .split_ascii_whitespace()
        .next()
        .ok_or_else(|| -> Error { "Empty input".into() })?
        .len();
    Ok((col_count as u8, row_count as u8))
}

fn reach_stable(mut seats: HashMap<(u8, u8), Status>) -> usize {
    let new_seats = get_changed_seats(&seats);
    if new_seats.is_empty() {
        seats
            .iter()
            .filter(|(_, status)| **status == Status::Occupied)
            .count()
    } else {
        seats.extend(new_seats.into_iter());
        reach_stable(seats)
    }
}

fn get_changed_seats(seats: &HashMap<(u8, u8), Status>) -> HashMap<(u8, u8), Status> {
    seats
        .iter()
        .filter_map(
            |((col, row), _)| match change_seat_status(*col, *row, seats) {
                Some(status) => Some(((*col, *row), status)),
                None => None,
            },
        )
        .collect()
}

fn change_seat_status(col: u8, row: u8, seats: &HashMap<(u8, u8), Status>) -> Option<Status> {
    let surrouding = get_surrouding(col, row);
    match seats.get(&(col, row)).cloned().unwrap_or(Status::Floor) {
        Status::Floor => None,
        Status::Empty => {
            let empty_count = surrouding
                .iter()
                .filter(|(col, row)| match seats.get(&(*col, *row)) {
                    Some(status) => status.is_empty(),
                    None => true,
                })
                .count();
            if empty_count == surrouding.len() {
                Some(Status::Occupied)
            } else {
                None
            }
        }
        Status::Occupied => {
            let occupied_count = surrouding
                .iter()
                .filter(|(col, row)| match seats.get(&(*col, *row)) {
                    Some(status) => *status == Status::Occupied,
                    None => false,
                })
                .count();
            if occupied_count >= 4 {
                Some(Status::Empty)
            } else {
                None
            }
        }
    }
}

fn get_surrouding(col: u8, row: u8) -> Vec<(u8, u8)> {
    (col.saturating_sub(1)..=col.saturating_add(1))
        .into_iter()
        .cartesian_product((row.saturating_sub(1)..=row.saturating_add(1)).into_iter())
        .filter(|(x, y)| *x != col || *y != row)
        .collect()
}

fn get_seat_map(input: &str) -> Result<HashMap<(u8, u8), Status>, Error> {
    let seats = input
        .split_ascii_whitespace()
        .enumerate()
        .map(|(row, line)| {
            line.trim()
                .char_indices()
                .map(move |(col, ch)| -> Result<((u8, u8), Status), Error> {
                    Ok(((col as u8, row as u8), Status::new(ch)?))
                })
        })
        .flatten()
        .collect::<Result<HashMap<_, _>, Error>>()?;
    Ok(seats)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL"#;

    #[test]
    fn test_reach_stable() {
        let seats = get_seat_map(SAMPLE).unwrap();
        assert_eq!(37, reach_stable(seats));
    }

    #[test]
    fn test_reach_stable_visible() {
        let (col_count, row_count) = get_dimensions(SAMPLE).unwrap();
        assert_eq!(10, col_count);
        assert_eq!(10, row_count);
        let seats = get_seat_map(SAMPLE).unwrap();
        assert_eq!(
            26,
            reach_stable_visible(seats, col_count - 1, row_count - 1)
        )
    }
}
