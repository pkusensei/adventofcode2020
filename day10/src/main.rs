use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!(r#"..\input.txt"#);
    let nums = get_nums(&input);
    let ones = find_diff(&nums, 1);
    let threes = find_diff(&nums, 3);
    debug_assert_eq!(2376, ones * threes);
    debug_assert_eq!(129586085429248, count_arrangements(&nums));
    println!("All done.");
}

fn find_diff(nums: &[u32], diff: u32) -> usize {
    nums[0..nums.len() - 1]
        .into_iter()
        .zip(nums[1..].into_iter())
        .filter_map(
            |(left, right)| {
                if right - left == diff {
                    Some(())
                } else {
                    None
                }
            },
        )
        .count()
}

fn count_arrangements(nums: &[u32]) -> usize {
    fn inner(nums: &[u32], result: &mut HashMap<usize, usize>, idx: usize) -> usize {
        if idx >= nums.len() - 1 {
            1
        } else if let Some(&r) = result.get(&idx) {
            r
        } else {
            let count = nums
                .iter()
                .skip(idx + 1)
                .enumerate()
                .take(3)
                .filter_map(|(i, v)| {
                    if v - nums[idx] <= 3 {
                        Some(inner(nums, result, idx + i + 1))
                    } else {
                        None
                    }
                })
                .sum();
            result.insert(idx, count);
            count
        }
    }

    let mut result = HashMap::new();
    inner(nums, &mut result, 0)
}

fn get_nums(input: &str) -> Vec<u32> {
    let mut nums: Vec<_> = input
        .split_ascii_whitespace()
        .map(|s| u32::from_str(s.trim()).unwrap())
        .collect();
    nums.sort();
    nums.insert(0, 0);
    nums.push(nums.last().cloned().unwrap() + 3);
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = r#"16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4"#;

    const SAMPLE2: &str = r#"28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3"#;

    #[test]
    fn test_find_diff() {
        {
            let nums = get_nums(SAMPLE1);
            let ones = find_diff(&nums, 1);
            let threes = find_diff(&nums, 3);
            assert_eq!(7, ones);
            assert_eq!(5, threes);
        }
        {
            let nums = get_nums(SAMPLE2);
            let ones = find_diff(&nums, 1);
            let threes = find_diff(&nums, 3);
            assert_eq!(22, ones);
            assert_eq!(10, threes);
        }
    }

    #[test]
    fn test_find_gaps() {
        {
            let nums = get_nums(SAMPLE1);
            assert_eq!(8, count_arrangements(&nums))
        }
        {
            let nums = get_nums(SAMPLE2);
            assert_eq!(19208, count_arrangements(&nums))
        }
    }
}
