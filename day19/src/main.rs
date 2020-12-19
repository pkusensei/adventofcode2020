use std::collections::HashMap;

use itertools::Itertools;

type Error = Box<dyn std::error::Error>;

fn main() {
    let input = include_str!("..\\input.txt");
    let (r, messages) = read(input);
    let rule_map = parse_rules(&r).unwrap();
    let rules = build_rule(&rule_map, 0);
    assert_eq!(222, count_valid(&rules, &messages));
    assert_eq!(339, count_valid_loop(&rule_map, &messages));
    println!("All done")
}

fn count_valid_loop(rules: &HashMap<u32, Rule>, messages: &[&str]) -> usize {
    messages
        .into_iter()
        .filter(|message| {
            let mut count_42 = 0;
            let mut remainder = **message;
            let mut result = check_valid(rules, &rules[&42], remainder);

            while let Ok(new_remainder) = result {
                count_42 += 1;
                remainder = new_remainder;
                result = check_valid(rules, &rules[&42], remainder);
            }

            if count_42 < 2 {
                return false;
            }

            let mut count_31 = 0;
            result = check_valid(rules, &rules[&31], remainder);

            while let Ok(new_remainder) = result {
                count_31 += 1;
                remainder = new_remainder;
                result = check_valid(rules, &rules[&31], remainder);
            }

            remainder.is_empty() && count_31 > 0 && count_42 > count_31
        })
        .count()
}

// Took this from others
// Can't figure out what I did wrong
// even tho the basic idea is the same
fn check_valid<'a>(
    rules: &HashMap<u32, Rule>,
    rule: &Rule,
    message: &'a str,
) -> Result<&'a str, Error> {
    match rule {
        Rule::Single(ch) => {
            if message.starts_with(*ch) {
                Ok(&message[1..])
            } else {
                Err(format!("{} does not start with {}", message, ch).into())
            }
        }
        Rule::Sequence(ids) => {
            let mut remainder = message;
            for id in ids {
                remainder = check_valid(rules, &rules[id], remainder)?;
            }
            Ok(remainder)
        }
        Rule::Either { left, right } => {
            let left_res = check_valid(rules, &Rule::Sequence(left.clone()), message);
            let right_res = check_valid(rules, &Rule::Sequence(right.clone()), message);
            if left_res.is_ok() {
                left_res
            } else if right_res.is_ok() {
                right_res
            } else {
                Err("Nope".into())
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Sequence(Vec<u32>),
    Either { left: Vec<u32>, right: Vec<u32> },
    Single(char),
}

fn count_valid(rules: &[String], messages: &[&str]) -> usize {
    messages
        .into_iter()
        .filter(|m| rules.iter().any(|r| r == *m))
        .count()
}

fn build_rule(rules: &HashMap<u32, Rule>, rule_id: u32) -> Vec<String> {
    fn concat_strs(left: Vec<String>, right: Vec<String>) -> Vec<String> {
        left.into_iter()
            .cartesian_product(right.into_iter())
            .map(|(l, r)| {
                let mut res = l.clone();
                res.push_str(&r);
                res
            })
            .collect()
    }
    fn build_strs_from_ids(rules: &HashMap<u32, Rule>, ids: &[u32]) -> Vec<String> {
        let mut res = build_rule(rules, ids[0]);
        for id in &ids[1..] {
            let sub_rules = build_rule(rules, *id);
            res = concat_strs(res, sub_rules);
        }
        res
    }

    let res = match &rules[&rule_id] {
        Rule::Single(c) => vec![c.to_string()],
        Rule::Sequence(ids) => build_strs_from_ids(rules, ids),
        Rule::Either { left, right } => {
            let mut left_res = build_strs_from_ids(rules, left);
            let right_res = build_strs_from_ids(rules, right);
            left_res.extend(right_res.into_iter());
            left_res
        }
    };

    res.into_iter().unique().collect()
}

fn parse_rules(lines: &[&str]) -> Result<HashMap<u32, Rule>, Error> {
    lines.into_iter().map(|line| parse_rule(line)).collect()
}

fn parse_rule(line: &str) -> Result<(u32, Rule), Error> {
    let rule_id: u32 = line
        .split(':')
        .next()
        .ok_or_else(|| "Invalid line of rule")?
        .parse()?;
    let rule_str = line
        .split(':')
        .skip(1)
        .next()
        .ok_or_else(|| "Invalid line of rule")?
        .trim();
    let rule_set = if rule_str.contains('|') {
        let nums: Vec<_> = rule_str
            .split('|')
            .map(|pair| {
                pair.trim()
                    .split_ascii_whitespace()
                    .map(|s| s.trim().parse())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<_, _>>()?;
        Rule::Either {
            left: nums[0].clone(),
            right: nums[1].clone(),
        }
    } else if rule_str.contains('\"') {
        let ch = rule_str
            .trim()
            .strip_prefix('\"')
            .and_then(|s| s.strip_suffix('\"'))
            .and_then(|s| s.chars().next())
            .ok_or_else(|| "Invalid line of rule")?;

        Rule::Single(ch)
    } else {
        let nums = rule_str
            .trim()
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Rule::Sequence(nums)
    };

    Ok((rule_id, rule_set))
}

fn read(input: &str) -> (Vec<&str>, Vec<&str>) {
    let rules: Vec<_> = input
        .lines()
        .map(str::trim)
        .take_while(|line| !line.is_empty())
        .collect();
    let messages: Vec<_> = input
        .lines()
        .map(str::trim)
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .collect();
    (rules, messages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        const SAMPLE: &str = r#"0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"
        
        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb"#;

        let (r, m) = read(SAMPLE);
        let rule_map = parse_rules(&r).unwrap();
        let rules = build_rule(&rule_map, 0);
        assert_eq!(2, count_valid(&rules, &m));
    }

    #[test]
    fn test_p2() {
        const SAMPLE: &str = r#"42: 9 14 | 10 1
        9: 14 27 | 1 26
        10: 23 14 | 28 1
        1: "a"
        11: 42 31
        5: 1 14 | 15 1
        19: 14 1 | 14 14
        12: 24 14 | 19 1
        16: 15 1 | 14 14
        31: 14 17 | 1 13
        6: 14 14 | 1 14
        2: 1 24 | 14 4
        0: 8 11
        13: 14 3 | 1 12
        15: 1 | 14
        17: 14 2 | 1 7
        23: 25 1 | 22 14
        28: 16 1
        4: 1 1
        20: 14 14 | 1 15
        3: 5 14 | 16 1
        27: 1 6 | 14 18
        14: "b"
        21: 14 1 | 1 14
        25: 1 1 | 1 14
        22: 14 14
        8: 42
        26: 14 22 | 1 20
        18: 15 15
        7: 14 5 | 1 21
        24: 14 1
        
        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
        bbabbbbaabaabba
        babbbbaabbbbbabbbbbbaabaaabaaa
        aaabbbbbbaaaabaababaabababbabaaabbababababaaa
        bbbbbbbaaaabbbbaaabbabaaa
        bbbababbbbaaaaaaaabbababaaababaabab
        ababaaaaaabaaab
        ababaaaaabbbaba
        baabbaaaabbaaaababbaababb
        abbbbabbbbaaaababbbbbbaaaababb
        aaaaabbaabaaaaababaa
        aaaabbaaaabbaaa
        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
        babaaabbbaaabaababbaabababaaab
        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

        let (r, m) = read(SAMPLE);
        let rule_map = parse_rules(&r).unwrap();
        let rules = build_rule(&rule_map, 0);
        assert_eq!(3, count_valid(&rules, &m));
        assert_eq!(12, count_valid_loop(&rule_map, &m));
    }
}
