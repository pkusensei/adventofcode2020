// Totally didn't do this by my self i.e take the code from others
// Bit manipulation is not remotely the most interesting thing :(

use std::{collections::HashMap, str::FromStr};

type Error = Box<dyn std::error::Error>;

fn main() {
    let input = include_str!("..\\input.txt");
    debug_assert_eq!(5875750429995, p1(input).unwrap());
    debug_assert_eq!(5272149590143, p2(input).unwrap());
    println!("All done")
}

fn p1(input: &str) -> Result<usize, Error> {
    let mut memory = HashMap::new();
    let mut and_or = (0, 0);
    for line in input.lines() {
        let line = line.trim();
        match line.strip_prefix("mask = ") {
            Some(m) => {
                and_or =
                    m.bytes()
                        .rev()
                        .enumerate()
                        .fold((usize::MAX, 0), |(and, or), (idx, byte)| match byte {
                            b'0' => (and & !(1 << idx), or),
                            b'1' => (and, or | 1 << idx),
                            _ => (and, or),
                        })
            }
            None => {
                let (k, v) = read_value(line)?;
                let value = v & and_or.0 | and_or.1;
                memory.insert(k, value);
            }
        }
    }
    Ok(memory.values().sum())
}

fn p2(input: &str) -> Result<usize, Error> {
    let mut memory = HashMap::new();
    let mut float_address = vec![];
    let mut whitelist = 0;
    for line in input.lines() {
        let line = line.trim();
        match line.strip_prefix("mask = ") {
            Some(mask) => {
                float_address.clear();
                let mut float_base = 0;
                let mut float_bits = vec![];
                whitelist = 0;

                mask.bytes()
                    .rev()
                    .enumerate()
                    .for_each(|(idx, byte)| match byte {
                        b'0' => whitelist |= 1 << idx,
                        b'1' => float_base |= 1 << idx,
                        b'X' => float_bits.push(idx),
                        _ => unreachable!(),
                    });
                float_address = (0..2usize.pow(float_bits.len() as u32))
                    .map(|template| {
                        float_bits
                            .iter()
                            .enumerate()
                            .fold(float_base, |addr, (idx, fb)| {
                                addr | (template & 1 << idx) << fb - idx
                            })
                    })
                    .collect()
            }
            None => {
                let (k, v) = read_value(line)?;
                let key = k & whitelist;
                for addr in float_address.as_slice() {
                    memory.insert(key | addr, v);
                }
            }
        }
    }

    Ok(memory.values().sum())
}

fn read_value(line: &str) -> Result<(usize, usize), Error> {
    let mut kvpair = line.split(" = ");
    let k = kvpair
        .next()
        .ok_or_else(|| -> Error { "Invalid value input".into() })?
        .strip_prefix("mem[")
        .ok_or_else(|| -> Error { "Invalid value input".into() })?
        .strip_suffix(']')
        .ok_or_else(|| -> Error { "Invalid value input".into() })?;
    let key = usize::from_str(k)?;
    let v = kvpair
        .next()
        .ok_or_else(|| -> Error { "Invalid value input".into() })?;
    let value = usize::from_str(v)?;
    Ok((key, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        const SAMPLE: &str = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0"#;

        let sum = p1(SAMPLE).unwrap();
        assert_eq!(165, sum)
    }

    #[test]
    fn test_p2() {
        const SAMPLE: &str = r#"mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1"#;

        let sum = p2(SAMPLE).unwrap();
        assert_eq!(208, sum)
    }
}
