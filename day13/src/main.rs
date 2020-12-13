use std::str::FromStr;

type Error = Box<dyn std::error::Error>;

fn main() {
    let input = include_str!("..\\input.txt");
    let (timestamp, ids) = read(input).unwrap();
    let (id, time) = find_lowest(timestamp, &ids).unwrap();
    debug_assert_eq!(3035, id * (time - timestamp));
    let stamps = read_stamps(input).unwrap();
    debug_assert_eq!(725169163285238, find_repeat(&stamps).unwrap());
    println!("All done")
}

fn find_repeat(stamps: &[(usize, usize)]) -> Option<usize> {
    let (rhs, modulii): (Vec<_>, Vec<_>) = stamps
        .iter()
        .map(|&(offset, stamp)| (-(offset as isize), stamp as isize))
        .unzip();
    match ring_algorithm::chinese_remainder_theorem(&rhs, &modulii) {
        None => None,
        Some(r) => {
            if r >= 0 {
                Some(r as usize)
            } else {
                let res = modulii.into_iter().fold(1, |acc, x| lcm(acc, x)) + r;
                Some(res as usize)
            }
        }
    }
}

fn read_stamps(input: &str) -> Result<Vec<(usize, usize)>, Error> {
    let line = input
        .split_terminator('\n')
        .skip(1)
        .next()
        .ok_or_else(|| -> Error { "Insufficient input".into() })?;
    let mut stamps: Vec<_> = line
        .split(',')
        .enumerate()
        .filter_map(|(idx, s)| match usize::from_str(s.trim()) {
            Ok(num) => Some((idx, num)),
            _ => None,
        })
        .collect();
    stamps.sort_unstable_by_key(|(idx, _)| *idx);
    Ok(stamps)
}

fn find_lowest(threshold: usize, nums: &[usize]) -> Result<(usize, usize), Error> {
    nums.into_iter()
        .map(|&num| (num, (threshold / num + 1) * num))
        .min_by_key(|(_num, higher_num)| *higher_num)
        .ok_or_else(|| "Cannot find lowest number".into())
}

fn read(input: &str) -> Result<(usize, Vec<usize>), Error> {
    let mut lines = input.split_terminator('\n');
    let threshold = usize::from_str(
        lines
            .next()
            .ok_or_else(|| -> Error { "Empty input".into() })?
            .trim(),
    )?;
    let nums: Vec<_> = lines
        .next()
        .ok_or_else(|| -> Error { "Insufficient input".into() })?
        .split(',')
        .filter_map(|s| usize::from_str(s.trim()).ok())
        .collect();
    Ok((threshold, nums))
}

fn gcd(x: isize, y: isize) -> isize {
    match y {
        0 => x.abs(),
        _ => gcd(y, x % y),
    }
}

fn lcm(x: isize, y: isize) -> isize {
    x * y / gcd(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"939
    7,13,x,x,59,x,31,19"#;

    #[test]
    fn test_find_lowest() {
        let (timestamp, ids) = read(SAMPLE).unwrap();
        let (id, time) = find_lowest(timestamp, &ids).unwrap();
        assert_eq!(59, id);
        assert_eq!(295, id * (time - timestamp))
    }

    #[test]
    fn test_find_repeat() {
        {
            let stamps = read_stamps(SAMPLE).unwrap();
            assert_eq!(1068781, find_repeat(&stamps).unwrap())
        }
        {
            let mut stamps: Vec<_> = "67,7,59,61"
                .split(',')
                .enumerate()
                .filter_map(|(idx, s)| match usize::from_str(s.trim()) {
                    Ok(num) => Some((idx, num)),
                    _ => None,
                })
                .collect();
            stamps.sort_unstable_by_key(|(idx, _)| *idx);
            assert_eq!(754018, find_repeat(&stamps).unwrap())
        }
        {
            let mut stamps: Vec<_> = "67,x,7,59,61"
                .split(',')
                .enumerate()
                .filter_map(|(idx, s)| match usize::from_str(s.trim()) {
                    Ok(num) => Some((idx, num)),
                    _ => None,
                })
                .collect();
            stamps.sort_unstable_by_key(|(idx, _)| *idx);
            assert_eq!(779210, find_repeat(&stamps).unwrap())
        }
        {
            let mut stamps: Vec<_> = "67,7,x,59,61"
                .split(',')
                .enumerate()
                .filter_map(|(idx, s)| match usize::from_str(s.trim()) {
                    Ok(num) => Some((idx, num)),
                    _ => None,
                })
                .collect();
            stamps.sort_unstable_by_key(|(idx, _)| *idx);
            assert_eq!(1261476, find_repeat(&stamps).unwrap())
        }
        {
            let mut stamps: Vec<_> = "1789,37,47,1889"
                .split(',')
                .enumerate()
                .filter_map(|(idx, s)| match usize::from_str(s.trim()) {
                    Ok(num) => Some((idx, num)),
                    _ => None,
                })
                .collect();
            stamps.sort_unstable_by_key(|(idx, _)| *idx);
            assert_eq!(1202161486, find_repeat(&stamps).unwrap())
        }
    }
}
