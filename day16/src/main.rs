use std::collections::HashMap;

type Error = Box<dyn std::error::Error>;

fn main() {
    let input = include_str!("..\\input.txt");
    let (rules, ticket, nearby) = parse(input).unwrap();
    assert_eq!(23044, scan_error_rate(&rules, &nearby));
    assert_eq!(3765150732757, p2(&rules, &ticket, &nearby));
    println!("All done")
}

fn p2(
    rules: &HashMap<&str, (u32, u32, u32, u32)>,
    ticket: &[u32],
    nearby: &Vec<Vec<u32>>,
) -> usize {
    let valid_tickets = find_valid_tickets(rules, nearby);
    let fields = match_fields(rules, &valid_tickets);
    fields
        .iter()
        .filter_map(|(field, idx)| match field.starts_with("departure") {
            true => Some(idx),
            false => None,
        })
        .map(|idx| ticket[*idx] as usize)
        .product()
}

fn match_fields<'a>(
    rules: &'a HashMap<&'a str, (u32, u32, u32, u32)>,
    valid_tickets: &'a Vec<Vec<u32>>,
) -> HashMap<&'a str, usize> {
    let length = valid_tickets[0].len();
    let mut tmp: HashMap<&str, Vec<_>> = HashMap::new();

    for (field, limit) in rules {
        for idx in 0..length {
            if valid_tickets.iter().all(|ticket| {
                let value = ticket[idx];
                (limit.0 <= value && value <= limit.1) || (limit.2 <= value && value <= limit.3)
            }) {
                match tmp.get_mut(field) {
                    Some(v) => v.push(idx),
                    None => {
                        tmp.insert(*field, vec![idx]);
                    }
                }
            }
        }
    }

    let mut fields = HashMap::new();

    while fields.len() < tmp.len() {
        for (k, v) in tmp.iter_mut() {
            if v.len() == 1 {
                fields.insert(*k, v[0]);
            } else {
                for i in fields.values() {
                    match v.iter().position(|x| *x == *i) {
                        Some(pos) => {
                            v.remove(pos);
                        }
                        None => (),
                    }
                }
            }
        }
    }

    fields
}

fn find_valid_tickets(
    rules: &HashMap<&str, (u32, u32, u32, u32)>,
    nearby: &Vec<Vec<u32>>,
) -> Vec<Vec<u32>> {
    nearby
        .iter()
        .filter(|ticket| check_invalid_ticket(rules, ticket).is_none())
        .cloned()
        .collect()
}

fn check_invalid_ticket(
    rules: &HashMap<&str, (u32, u32, u32, u32)>,
    ticket: &[u32],
) -> Option<u32> {
    for &value in ticket {
        if rules
            .values()
            .all(|limit| value < limit.0 || limit.3 < value || (limit.1 < value && value < limit.2))
        {
            return Some(value);
        }
    }
    None
}

fn scan_error_rate(rules: &HashMap<&str, (u32, u32, u32, u32)>, nearby: &Vec<Vec<u32>>) -> u32 {
    nearby
        .iter()
        .filter_map(|ticket| check_invalid_ticket(rules, ticket))
        .sum()
}

fn parse(
    input: &str,
) -> Result<(HashMap<&str, (u32, u32, u32, u32)>, Vec<u32>, Vec<Vec<u32>>), Error> {
    let rules: HashMap<_, _> = input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(parse_rule)
        .collect::<Result<HashMap<_, _>, Error>>()?;
    let ticket = input
        .lines()
        .skip_while(|line| !line.trim().starts_with("your ticket"))
        .skip(1)
        .map(parse_ticket)
        .next()
        .ok_or_else(|| "Invalid ticket input")??;
    let nearby = input
        .lines()
        .skip_while(|line| !line.trim().starts_with("nearby tickets"))
        .skip(1)
        .take_while(|line| !line.trim().is_empty())
        .map(parse_ticket)
        .collect::<Result<Vec<_>, Error>>()?;

    Ok((rules, ticket, nearby))
}

fn parse_rule(line: &str) -> Result<(&str, (u32, u32, u32, u32)), Error> {
    let mut kvpair = line.split(':');
    let field = kvpair.next().ok_or_else(|| "Invalid rule input")?.trim();
    let limits = kvpair
        .next()
        .ok_or_else(|| "Invalid rule input")?
        .trim()
        .split("or")
        .map(|pair| pair.trim().split('-').map(|num| num.parse()))
        .flatten()
        .collect::<Result<Vec<_>, _>>()?;
    Ok((field, (limits[0], limits[1], limits[2], limits[3])))
}

fn parse_ticket(line: &str) -> Result<Vec<u32>, Error> {
    Ok(line
        .split(',')
        .map(|n| n.trim().parse())
        .collect::<Result<Vec<u32>, _>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = r#"class: 1-3 or 5-7
    row: 6-11 or 33-44
    seat: 13-40 or 45-50
    
    your ticket:
    7,1,14
    
    nearby tickets:
    7,3,47
    40,4,50
    55,2,20
    38,6,12"#;

    const SAMPLE2: &str = r#"class: 0-1 or 4-19
    row: 0-5 or 8-19
    seat: 0-13 or 16-19
    
    your ticket:
    11,12,13
    
    nearby tickets:
    3,9,18
    15,1,5
    5,14,9"#;

    #[test]
    fn test_scan_error_rate() {
        let (rules, _, nearby) = parse(SAMPLE1).unwrap();
        assert_eq!(71, scan_error_rate(&rules, &nearby));
        // println!("{:?}", find_valid_tickets(&rules, &nearby))
    }

    #[test]
    fn test_match_fields() {
        let (rules, _, nearby) = parse(SAMPLE2).unwrap();
        println!("{:?}", match_fields(&rules, &nearby))
    }
}
