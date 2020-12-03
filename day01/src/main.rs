use std::{path::Path, str::FromStr};

use tools::{self, Error};

const SUM: u32 = 2020;

fn read_input<P: AsRef<Path>>(p: P) -> Result<Vec<u32>, Error> {
    let lines = tools::read_input(p)?;
    let mut numbers = lines
        .iter()
        .map(|s| u32::from_str(s))
        .collect::<Result<Vec<_>, _>>()?;
    numbers.sort();
    Ok(numbers)
}

fn find_pair(numbers: &[u32]) -> Option<(u32, u32)> {
    let mut to_skip = 0;
    for small in numbers {
        for big in numbers.iter().rev().skip(to_skip) {
            if small + big > SUM {
                to_skip += 1;
                continue;
            } else if small + big < SUM {
                break;
            } else {
                return Some((*small, *big));
            }
        }
    }
    None
}

/// shameless bruteforce
fn find_three(numbers: &[u32]) -> Option<(u32, u32, u32)> {
    let mut last_idx = numbers.len() - 1;
    while numbers[last_idx] + numbers[0] + numbers[1] > SUM {
        last_idx -= 1;
    }
    for idx1 in 0..(last_idx - 1) {
        for idx2 in (idx1 + 1)..last_idx {
            for idx3 in (idx2 + 1)..=last_idx {
                if numbers[idx1] + numbers[idx2] + numbers[idx3] > SUM {
                    break;
                } else if numbers[idx1] + numbers[idx2] + numbers[idx3] < SUM {
                    continue;
                } else {
                    return Some((numbers[idx1], numbers[idx2], numbers[idx3]));
                }
            }
        }
    }
    None
}

fn main() -> Result<(), Error> {
    let input = read_input("input.txt")?;
    if let Some((small, big)) = find_pair(&input) {
        println!("{} * {} = {}", small, big, small * big)
    } else {
        println!("Error in find_pair")
    }
    if let Some((x, y, z)) = find_three(&input) {
        println!("{} * {} * {} = {}", x, y, z, x * y * z)
    } else {
        println!("Error in find_three")
    }
    Ok(())
}
