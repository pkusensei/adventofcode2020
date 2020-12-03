use std::str::FromStr;

use tools::{self, Error};

fn find_limits(s: &str) -> Result<(u8, u8), Error> {
    let numbers = s
        .split('-')
        .map(u8::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    if numbers.len() != 2 {
        Err("Invalid policy limits".into())
    } else {
        Ok((numbers[0], numbers[1]))
    }
}

fn find_letter(s: &str) -> Result<char, Error> {
    if let Some(res) = s.strip_suffix(':') {
        if res.len() == 1 {
            if let Some(ch) = res.chars().nth(0) {
                return Ok(ch);
            }
        }
    }
    Err("Invalid policy letter".into())
}

fn check_valid(s: &str) -> bool {
    let parts: Vec<_> = s.split(' ').collect();
    if parts.len() != 3 {
        return false;
    }
    if let Ok((lower, upper)) = find_limits(parts[0]) {
        if let Ok(ch) = find_letter(parts[1]) {
            let count = parts[2].matches(ch).count() as u8;
            return lower <= count && count <= upper;
        }
    }
    false
}

fn check_valid_position(s: &str) -> bool {
    let parts: Vec<_> = s.split(' ').collect();
    if parts.len() != 3 {
        return false;
    }
    if let Ok((lower, upper)) = find_limits(parts[0]) {
        if let Ok(ch) = find_letter(parts[1]) {
            let count = parts[2]
                .char_indices()
                .filter(|pair| {
                    pair.1 == ch && (pair.0 as u8 == lower - 1 || pair.0 as u8 == upper - 1)
                })
                .count();
            return count == 1;
        }
    }
    false
}

fn main() -> Result<(), Error> {
    let input = tools::read_input("input.txt")?;

    let count = input.iter().filter(|s| check_valid(s)).count();
    println!("Count: {}", count);
    let count = input.iter().filter(|s| check_valid_position(s)).count();
    println!("Count positions: {}", count);

    Ok(())
}
