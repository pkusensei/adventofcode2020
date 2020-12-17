use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("..\\input.txt");
    assert_eq!(237, p1::run(input));
    assert_eq!(2448, p2::run(input));
    println!("All done")
}

mod p1 {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Coord {
        x: i32,
        y: i32,
        z: i32,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum State {
        Active,
        Inactive,
    }

    pub fn run(input: &str) -> usize {
        let init = read(input);
        let last = simulate(init, 0, 6);
        last.values().filter(|s| **s == State::Active).count()
    }

    fn simulate(
        old_state: HashMap<Coord, State>,
        current_cycle: u32,
        target_cycle: u32,
    ) -> HashMap<Coord, State> {
        if current_cycle == target_cycle {
            old_state
        } else {
            let coords: Vec<_> = old_state.keys().collect();
            let new_coords = apply_deltas(&coords);
            let new_state = new_coords
                .into_iter()
                .map(|new_coord| {
                    let active_count = apply_delta(&new_coord)
                        .into_iter()
                        .filter(|c| *c != new_coord)
                        .filter(|c| match old_state.get(&c) {
                            Some(State::Active) => true,
                            _ => false,
                        })
                        .count();
                    match old_state.get(&new_coord) {
                        Some(State::Active) => match active_count {
                            2 | 3 => (new_coord, State::Active),
                            _ => (new_coord, State::Inactive),
                        },
                        _ => match active_count {
                            3 => (new_coord, State::Active),
                            _ => (new_coord, State::Inactive),
                        },
                    }
                })
                .collect();
            simulate(new_state, current_cycle + 1, target_cycle)
        }
    }

    fn read(input: &str) -> HashMap<Coord, State> {
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.trim().char_indices().map(move |(x, ch)| match ch {
                    '#' => (
                        Coord {
                            x: x as i32,
                            y: y as i32,
                            z: 0,
                        },
                        State::Active,
                    ),
                    '.' => (
                        Coord {
                            x: x as i32,
                            y: y as i32,
                            z: 0,
                        },
                        State::Inactive,
                    ),
                    _ => unreachable!(),
                })
            })
            .flatten()
            .collect()
    }

    fn apply_delta(coord: &Coord) -> Vec<Coord> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .map(|((x, y), z)| (x, y, z))
            .map(|(dx, dy, dz)| Coord {
                x: coord.x + dx,
                y: coord.y + dy,
                z: coord.z + dz,
            })
            .collect()
    }

    fn apply_deltas(coords: &[&Coord]) -> Vec<Coord> {
        coords
            .into_iter()
            .map(|c| apply_delta(*c).into_iter())
            .flatten()
            .unique()
            .collect()
    }
}

mod p2 {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Coord {
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum State {
        Active,
        Inactive,
    }

    pub fn run(input: &str) -> usize {
        let init = read(input);
        let last = simulate(init, 0, 6);
        last.values().filter(|s| **s == State::Active).count()
    }

    fn simulate(
        old_state: HashMap<Coord, State>,
        current_cycle: u32,
        target_cycle: u32,
    ) -> HashMap<Coord, State> {
        if current_cycle == target_cycle {
            old_state
        } else {
            let coords: Vec<_> = old_state.keys().collect();
            let new_coords = apply_deltas(&coords);
            let new_state = new_coords
                .into_iter()
                .map(|new_coord| {
                    let active_count = apply_delta(&new_coord)
                        .into_iter()
                        .filter(|c| *c != new_coord)
                        .filter(|c| match old_state.get(&c) {
                            Some(State::Active) => true,
                            _ => false,
                        })
                        .count();
                    match old_state.get(&new_coord) {
                        Some(State::Active) => match active_count {
                            2 | 3 => (new_coord, State::Active),
                            _ => (new_coord, State::Inactive),
                        },
                        _ => match active_count {
                            3 => (new_coord, State::Active),
                            _ => (new_coord, State::Inactive),
                        },
                    }
                })
                .collect();
            simulate(new_state, current_cycle + 1, target_cycle)
        }
    }

    fn read(input: &str) -> HashMap<Coord, State> {
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.trim().char_indices().map(move |(x, ch)| match ch {
                    '#' => (
                        Coord {
                            x: x as i32,
                            y: y as i32,
                            z: 0,
                            w: 0,
                        },
                        State::Active,
                    ),
                    '.' => (
                        Coord {
                            x: x as i32,
                            y: y as i32,
                            z: 0,
                            w: 0,
                        },
                        State::Inactive,
                    ),
                    _ => unreachable!(),
                })
            })
            .flatten()
            .collect()
    }

    fn apply_delta(coord: &Coord) -> Vec<Coord> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .map(|(((x, y), z), w)| (x, y, z, w))
            .map(|(dx, dy, dz, dw)| Coord {
                x: coord.x + dx,
                y: coord.y + dy,
                z: coord.z + dz,
                w: coord.w + dw,
            })
            .collect()
    }

    fn apply_deltas(coords: &[&Coord]) -> Vec<Coord> {
        coords
            .into_iter()
            .map(|c| apply_delta(*c).into_iter())
            .flatten()
            .unique()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#".#.
    ..#
    ###"#;

    #[test]
    fn test_p1() {
        assert_eq!(112, p1::run(SAMPLE))
    }

    #[test]
    fn test_p2() {
        assert_eq!(848, p2::run(SAMPLE))
    }
}
