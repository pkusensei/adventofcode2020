use std::collections::HashMap;

fn main() {
    assert_eq!(1618, solve(&[0, 13, 1, 8, 6, 15], 2020));
    assert_eq!(548531, solve(&[0, 13, 1, 8, 6, 15], 30000000));
    println!("All done")
}

fn solve(input: &[usize], target_length: usize) -> usize {
    let mut nums = Vec::with_capacity(target_length);
    let mut positions = HashMap::new();
    for (idx, num) in input.iter().enumerate() {
        nums.push(*num);
        positions.insert(*num, idx);
    }
    nums.push(0);
    for i in input.len()..target_length {
        let v = match positions.get(&nums[i]) {
            Some(idx) => i - idx,
            None => 0,
        };
        nums.push(v);
        positions.insert(nums[i], i);
    }
    nums[target_length - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(436, solve(&[0, 3, 6], 2020));
        assert_eq!(1, solve(&[1, 3, 2], 2020));
        assert_eq!(10, solve(&[2, 1, 3], 2020));
        assert_eq!(27, solve(&[1, 2, 3], 2020));
        assert_eq!(78, solve(&[2, 3, 1], 2020));
        assert_eq!(438, solve(&[3, 2, 1], 2020));
        assert_eq!(1836, solve(&[3, 1, 2], 2020));
    }

    #[test]
    fn test_p2() {
        assert_eq!(175594, solve(&[0, 3, 6], 30000000));
        assert_eq!(2578, solve(&[1, 3, 2], 30000000));
        assert_eq!(3544142, solve(&[2, 1, 3], 30000000));
        assert_eq!(261214, solve(&[1, 2, 3], 30000000));
        assert_eq!(6895259, solve(&[2, 3, 1], 30000000));
        assert_eq!(18, solve(&[3, 2, 1], 30000000));
        assert_eq!(362, solve(&[3, 1, 2], 30000000));
    }
}
