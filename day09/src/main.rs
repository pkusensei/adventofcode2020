use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let nums: Vec<_> = include_str!(r#"..\input.txt"#)
        .lines()
        .map(|s| usize::from_str(s.trim()).unwrap())
        .collect();
    let (idx, num) = find_invalid(&nums, 25).unwrap();
    debug_assert_eq!(num, 1038347917);
    let window = try_find_window(&nums[0..idx], num).unwrap();
    debug_assert_eq!(137394018, find_sum(window).unwrap());
    println!("All done!")
}

fn read_window(nums: &[usize], start_idx: usize, length: usize) -> &[usize] {
    &nums[start_idx..start_idx + length]
}

fn find_invalid(nums: &[usize], length: usize) -> Option<(usize, usize)> {
    for idx in length..nums.len() {
        let window = read_window(nums, idx - length, length);
        let sums: Vec<_> = window
            .into_iter()
            .combinations(2)
            .map(|com| com.iter().cloned().sum::<usize>())
            .collect();
        let num = nums[idx];
        if sums.contains(&num) {
            continue;
        } else {
            return Some((idx, num));
        }
    }
    None
}

fn try_find_window(nums: &[usize], num: usize) -> Option<&[usize]> {
    for start in 0..nums.len() - 1 {
        for end in start + 2..nums.len() {
            let window = &nums[start..end];
            let sum = window.into_iter().cloned().sum::<usize>();
            if sum == num {
                return Some(window);
            } else if sum > num {
                break;
            }
        }
    }
    None
}

fn find_sum(nums: &[usize]) -> Option<usize> {
    if let Some(s) = nums.iter().min() {
        if let Some(l) = nums.iter().max() {
            return Some(s + l);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    const sample: &str = r#"35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576"#;

    #[test]
    fn test_find_invalid() {
        let nums: Vec<_> = sample
            .lines()
            .map(|s| usize::from_str(s.trim()).unwrap())
            .collect();
        let (idx, num) = find_invalid(&nums, 5).unwrap();
        assert_eq!(127, num);
        let window = try_find_window(&nums[0..idx], 127).unwrap();
        assert_eq!(62, find_sum(window).unwrap())
    }
}
