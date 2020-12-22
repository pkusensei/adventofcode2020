use std::collections::{HashSet, VecDeque};

type Error = Box<dyn std::error::Error>;

fn main() {
    let input = include_str!("..\\input.txt");
    let (p1, p2) = read(input).unwrap();
    {
        let w = play(p1.clone(), p2.clone());
        assert_eq!(31455, calc_score(&w));
    }
    {
        let (_, w) = play_rec(p1, p2);
        assert_eq!(32528, calc_score(&w));
    }
}

fn calc_score(p: &VecDeque<usize>) -> usize {
    p.into_iter()
        .rev()
        .enumerate()
        .map(|(idx, num)| (idx + 1) * num)
        .sum()
}

fn play_rec(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> (u8, VecDeque<usize>) {
    let mut p1seen = HashSet::new();
    let mut p2seen = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        if p1seen.contains(&p1) || p2seen.contains(&p2) {
            return (1, p1);
        } else {
            p1seen.insert(p1.clone());
            p2seen.insert(p2.clone());
        }

        let card1 = p1.pop_front().unwrap();
        let card2 = p2.pop_front().unwrap();

        let w = if card1 <= p1.len() && card2 <= p2.len() {
            let new_p1 = p1.iter().take(card1).cloned().collect();
            let new_p2 = p2.iter().take(card2).cloned().collect();
            play_rec(new_p1, new_p2).0
        } else {
            if card1 > card2 {
                1
            } else {
                2
            }
        };
        match w {
            1 => {
                p1.push_back(card1);
                p1.push_back(card2);
            }
            2 => {
                p2.push_back(card2);
                p2.push_back(card1)
            }
            _ => unreachable!(),
        }
    }

    if p1.is_empty() {
        (2, p2)
    } else {
        (1, p1)
    }
}

fn play(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> VecDeque<usize> {
    while !p1.is_empty() && !p2.is_empty() {
        let card1 = p1.pop_front().unwrap();
        let card2 = p2.pop_front().unwrap();
        if card1 > card2 {
            p1.push_back(card1);
            p1.push_back(card2)
        } else {
            p2.push_back(card2);
            p2.push_back(card1)
        }
    }
    if p1.is_empty() {
        p2
    } else {
        p1
    }
}

fn read(input: &str) -> Result<(VecDeque<usize>, VecDeque<usize>), Error> {
    let p1 = input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .skip(1)
        .map(|num| num.trim().parse())
        .collect::<Result<_, _>>()?;

    let p2 = input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(2)
        .map(|num| num.trim().parse())
        .collect::<Result<_, _>>()?;

    Ok((p1, p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"Player 1:
    9
    2
    6
    3
    1
    
    Player 2:
    5
    8
    4
    7
    10"#;

    #[test]
    fn test() {
        let (p1, p2) = read(SAMPLE).unwrap();
        {
            let w = play(p1.clone(), p2.clone());
            assert_eq!(306, calc_score(&w));
        }
        {
            let (_, w) = play_rec(p1, p2);
            assert_eq!(291, calc_score(&w))
        }
    }
}
