use std::{collections::HashMap, str::FromStr};

use tools::*;

struct Passport {
    entries: HashMap<String, String>,
}

impl Passport {
    fn requires_entries(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|s| self.entries.contains_key(*s))
    }

    fn requires_valid_entries(&self) -> bool {
        self.entries.iter().all(|(k, v)| match k.as_str() {
            "byr" => match u32::from_str(v) {
                Ok(num) => 1920 <= num && num <= 2020,
                _ => false,
            },
            "iyr" => match u32::from_str(v) {
                Ok(num) => 2010 <= num && num <= 2020,
                _ => false,
            },
            "eyr" => match u32::from_str(v) {
                Ok(num) => 2020 <= num && num <= 2030,
                _ => false,
            },
            "hgt" => match &v[v.len() - 2..] {
                "cm" => {
                    let s = &v[..v.len() - 2];
                    match u32::from_str(s) {
                        Ok(num) => 150 <= num && num <= 193,
                        _ => false,
                    }
                }
                "in" => {
                    let s = &v[..v.len() - 2];
                    match u32::from_str(s) {
                        Ok(num) => 59 <= num && num <= 76,
                        _ => false,
                    }
                }
                _ => false,
            },
            "hcl" => match v.strip_prefix('#') {
                Some(s) => {
                    s.len() == 6
                        && s.chars().all(|c| match c {
                            '0'..='9' => true,
                            'a'..='f' => true,
                            _ => false,
                        })
                }
                _ => false,
            },
            "ecl" => match v.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            "pid" => v.len() == 9 && v.chars().all(|c| '0' <= c && c <= '9'),

            _ => true,
        })
    }
}

fn get_passports(lines: &[String]) -> Vec<Passport> {
    let mut entries = HashMap::new();
    let mut res = vec![];
    for line in lines {
        if line.trim().is_empty() {
            res.push(Passport {
                entries: entries.clone(),
            });
            entries.clear()
        } else {
            line.split_ascii_whitespace().for_each(|s| {
                let mut kvpair = s.split(':');
                match (kvpair.next(), kvpair.next()) {
                    (Some(k), Some(v)) => {
                        entries.insert(k.trim().to_owned(), v.trim().to_owned());
                    }
                    _ => (),
                }
            })
        }
    }
    if !entries.is_empty() {
        res.push(Passport { entries })
    }
    res
}

fn main() -> Result<(), Error> {
    let input = tools::read_input("input.txt")?;

    let passports: Vec<_> = get_passports(&input)
        .into_iter()
        .filter(Passport::requires_entries)
        .collect();
    println!("{}", passports.len());
    println!(
        "{}",
        passports
            .into_iter()
            .filter(Passport::requires_valid_entries)
            .count()
    );

    Ok(())
}
