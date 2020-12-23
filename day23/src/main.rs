type Error = Box<dyn std::error::Error>;

fn main() {
    let input = "389547612";
    let nums = read(input).unwrap();
    assert_eq!("45286397", collect_label(play(nums, 100).unwrap()).unwrap());
    assert_eq!(836763710, p2(input).unwrap());
    println!("All done");
}

fn p2(input: &str) -> Result<usize, Error> {
    let input_nums = read(input)?;
    let mut nums = vec![0; 1_000_001];

    for i in 0..input_nums.len() - 1 {
        nums[input_nums[i] as usize] = input_nums[i + 1]
    }
    nums[*input_nums.last().unwrap() as usize] = 10;
    for i in input_nums.len() + 1..1_000_000 {
        nums[i] = (i + 1) as u32;
    }
    nums[1_000_000] = input_nums[0];
    nums = play_p2(nums, 10_000_000, input_nums[0]);
    Ok(nums[1] as usize * nums[nums[1] as usize] as usize)
}

// Totally stole this
fn play_p2(mut nums: Vec<u32>, moves: u32, mut current: u32) -> Vec<u32> {
    for _ in 0..moves {
        let a = nums[current as usize];
        let b = nums[a as usize];
        let c = nums[b as usize];
        let mut dest = if current.saturating_sub(1) <= 0 {
            nums.len() as u32 - 1
        } else {
            current - 1
        };

        while dest == a || dest == b || dest == c {
            dest = dest.saturating_sub(1);
            if dest <= 0 {
                dest = nums.len() as u32 - 1
            }
        }

        nums[current as usize] = nums[c as usize];
        let tmp = nums[dest as usize];
        nums[dest as usize] = a;
        nums[c as usize] = tmp;
        current = nums[current as usize]
    }
    nums
}

fn play(mut nums: Vec<u32>, moves: u32) -> Result<Vec<u32>, Error> {
    let mut current = nums[0];
    for _ in 0..moves {
        let (left, picked) = pick_up(nums, current)?;
        let (dest_idx, _) = find_dest(&left, current)?;
        nums = insert_back(left, dest_idx, picked);
        let current_idx = index_of(&nums, current)?;
        current = nums[(current_idx + 1) % nums.len()];
    }
    Ok(nums)
}

fn read(input: &str) -> Result<Vec<u32>, Error> {
    Ok(input
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| format!("Invalid input: {}", input))?)
}

fn pick_up(nums: Vec<u32>, current: u32) -> Result<(Vec<u32>, Vec<u32>), Error> {
    let mut picked = vec![];
    let current_idx = index_of(&nums, current)?;
    for i in 0..=2 {
        let idx = (current_idx + i + 1) % nums.len();
        picked.push(nums[idx]);
    }
    let left: Vec<_> = nums
        .into_iter()
        .filter(|num| !picked.contains(num))
        .collect();

    Ok((left, picked))
}

fn find_dest(nums: &[u32], current: u32) -> Result<(usize, u32), Error> {
    let min = nums
        .iter()
        .cloned()
        .min()
        .ok_or_else(|| format!("Duplicate values in {:?}", nums))?;
    let max = nums
        .iter()
        .cloned()
        .max()
        .ok_or_else(|| format!("Duplicate values in {:?}", nums))?;

    let mut res = current - 1;
    while !nums.contains(&res) {
        if res < min {
            res = max
        } else {
            res -= 1
        }
    }
    let idx = nums
        .into_iter()
        .position(|x| *x == res)
        .ok_or_else(|| format!("Cannot find {} in {:?}", res, nums))?;
    Ok((idx, res))
}

fn insert_back(mut nums: Vec<u32>, dest_idx: usize, picked: Vec<u32>) -> Vec<u32> {
    for num in picked.into_iter().rev() {
        nums.insert(dest_idx + 1, num)
    }
    nums
}

fn collect_label(mut nums: Vec<u32>) -> Result<String, Error> {
    let idx = nums
        .iter()
        .position(|x| *x == 1)
        .ok_or_else(|| format!("Cannot find \"1\" in {:?}", nums))?;
    nums.rotate_left(idx);
    nums.remove(0);
    let res = nums
        .into_iter()
        .map(|num| std::char::from_digit(num, 10))
        .collect::<Option<Vec<_>>>()
        .and_then(|v| Some(v.into_iter().collect::<String>()))
        .ok_or_else(|| "Cannot build string")?;
    Ok(res)
}

fn index_of(nums: &[u32], num: u32) -> Result<usize, Error> {
    Ok(nums
        .iter()
        .position(|x| *x == num)
        .ok_or_else(|| format!("Cannot find {} in {:?}", num, nums))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "389125467";

    #[test]
    fn test_p1() {
        let nums = read(SAMPLE).unwrap();
        let move_10 = play(nums.clone(), 10).unwrap();
        assert_eq!("92658374", collect_label(move_10).unwrap());
        let move_100 = play(nums, 100).unwrap();
        assert_eq!("67384529", collect_label(move_100).unwrap());
    }

    #[test]
    fn test_p2() {
        assert_eq!(149245887792, p2(SAMPLE).unwrap())
    }
}
