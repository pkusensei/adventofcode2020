use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = tools::read_input("input.txt").unwrap();
    let rules = parse_rules(&input);
    println!("To Shiny gold: {}", count_shiny_gold(&rules));
    println!(
        "Shiny gold contains: {}",
        count_contained(&rules, "shiny gold") - 1
    )
}

fn count_shiny_gold(rules: &HashMap<&str, Vec<(u32, &str)>>) -> usize {
    rules
        .keys()
        .filter(|k| contains_color(rules, k, "shiny gold"))
        .count()
}

fn count_contained(rules: &HashMap<&str, Vec<(u32, &str)>>, start: &str) -> usize {
    match rules.get(&start) {
        Some(ncpairs) => {
            if ncpairs.is_empty() {
                1
            } else {
                1 + ncpairs
                    .iter()
                    .map(|(num, color)| (*num as usize) * count_contained(rules, color))
                    .sum::<usize>()
            }
        }
        _ => 0,
    }
}

fn contains_color(rules: &HashMap<&str, Vec<(u32, &str)>>, start: &str, target: &str) -> bool {
    match rules.get(&start) {
        Some(colors) => {
            if colors.iter().map(|(_, color)| color).any(|&c| c == target) {
                true
            } else {
                colors
                    .iter()
                    .map(|(_, color)| contains_color(rules, color, target))
                    .fold(false, |acc, i| acc || i)
            }
        }
        _ => false,
    }
}

fn parse_rules(lines: &[String]) -> HashMap<&str, Vec<(u32, &str)>> {
    lines
        .into_iter()
        .map(|s| {
            let mut kvpair = s.split("bags contain");
            let key = kvpair.next().unwrap().trim();
            let value = kvpair.next().unwrap().trim();
            (key, parse_contained(value))
        })
        .collect()
}

fn parse_contained(line: &str) -> Vec<(u32, &str)> {
    if line.starts_with("no other") {
        vec![]
    } else {
        line.split(',')
            .map(|s| {
                let mut num_color_pair = s.trim().splitn(2, ' ');
                let num = u32::from_str(num_color_pair.next().unwrap()).unwrap();
                let color = num_color_pair
                    .next()
                    .unwrap()
                    .rsplitn(2, ' ')
                    .skip(1)
                    .next()
                    .unwrap();
                (num, color)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<String> {
        r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags."#
            .split('\n')
            .map(|s| s.trim().to_owned())
            .collect()
    }

    #[test]
    fn test_count_shiny_gold() {
        let lines = sample();
        let rules = parse_rules(&lines);
        assert_eq!(4, count_shiny_gold(&rules))
    }

    #[test]
    fn test_count_contained() {
        {
            let lines = sample();
            let rules = parse_rules(&lines);
            assert_eq!(32, count_contained(&rules, "shiny gold") - 1);
        }
        {
            let lines: Vec<_> = r#"shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags."#
                .split('\n')
                .map(|s| s.trim().to_owned())
                .collect();
            let rules = parse_rules(&lines);
            assert_eq!(126, count_contained(&rules, "shiny gold") - 1);
        }
    }
}
