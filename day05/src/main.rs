use std::collections::BTreeSet;

fn find_num(s: &str, lower: u32, upper: u32) -> u32 {
    match s {
        "F" | "L" => lower,
        "B" | "R" => upper,

        _ => match s.chars().next() {
            Some('F') | Some('L') => find_num(&s[1..], lower, (upper + lower) / 2),
            Some('B') | Some('R') => find_num(&s[1..], (upper + lower) / 2 + 1, upper),
            _ => panic!("Invalid input"),
        },
    }
}

fn get_seat_id(s: &str) -> u32 {
    let row = find_num(&s[..7], 0, 127);
    let col = find_num(&s[7..], 0, 7);
    row * 8 + col
}

fn find_seat_id(ids: &BTreeSet<u32>) -> u32 {
    let all_ids: BTreeSet<u32> = (1..127)
        .into_iter()
        .map(|row| {
            (0..=7)
                .into_iter()
                .map(|col| row * 8 + col)
                .collect::<BTreeSet<_>>()
        })
        .flatten()
        .collect();
    all_ids
        .difference(&ids)
        .copied()
        .find(|&id| ids.contains(&(id + 1)) && ids.contains(&(id - 1)))
        .unwrap()
        .clone()
}

fn main() {
    let input = tools::read_input("input.txt").unwrap();
    let ids: BTreeSet<_> = input.into_iter().map(|s| get_seat_id(&s)).collect();

    println!("Highest seat id: {}", ids.iter().max().unwrap());
    println!("Seat id: {}", find_seat_id(&ids));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_num() {
        assert_eq!(find_num("FBFBBFF", 0, 127), 44);
        assert_eq!(find_num("RLR", 0, 7), 5)
    }

    #[test]
    fn test_find_seat_id() {
        assert_eq!(get_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
    }
}
