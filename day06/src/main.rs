use std::collections::BTreeSet;

fn main() {
    let input = tools::read_input("input.txt").unwrap();
    println!(
        "No. of yes: {}",
        group_answers(&input)
            .iter()
            .map(|g| count_answers_group(g))
            .sum::<usize>()
    );
    println!(
        "No. of intersect: {}",
        group_answers(&input)
            .iter()
            .map(|g| count_intersect(g))
            .sum::<usize>()
    );
}

fn group_answers(lines: &[String]) -> Vec<Vec<&str>> {
    let mut groups = vec![];
    let mut one_group = vec![];
    for line in lines {
        if line.trim().is_empty() {
            groups.push(one_group.clone());
            one_group.clear()
        } else {
            one_group.push(line.trim())
        }
    }

    if !one_group.is_empty() {
        groups.push(one_group)
    }
    groups
}

fn count_answers_group(answers: &[&str]) -> usize {
    answers
        .into_iter()
        .map(|answer| answer.chars())
        .flatten()
        .collect::<BTreeSet<_>>()
        .len()
}

fn count_intersect(answers: &[&str]) -> usize {
    let full: BTreeSet<char> = ('a'..='z').into_iter().collect();
    answers
        .into_iter()
        .map(|answer| answer.chars().collect::<BTreeSet<_>>())
        .fold(full, |intersect, s| {
            intersect.intersection(&s).cloned().collect::<BTreeSet<_>>()
        })
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<String> {
        "abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b"
        .split('\n')
        .map(|s| s.to_owned())
        .collect()
    }

    #[test]
    fn test_count_answers() {
        let num: usize = group_answers(&sample())
            .iter()
            .map(|g| count_answers_group(g))
            .sum();
        assert_eq!(num, 11)
    }

    #[test]
    fn test_count_intersect() {
        let num: usize = group_answers(&sample())
            .iter()
            .map(|g| count_intersect(g))
            .sum();
        assert_eq!(num, 6)
    }
}
