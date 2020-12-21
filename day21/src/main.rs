use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Error = Box<dyn std::error::Error>;

fn main() {
    let input = include_str!("..\\input.txt");
    let lines = read(input).unwrap();
    let mut ai_map = find_ingredients(&lines);
    assert_eq!(1945, count_non_allergy(&ai_map, &lines));
    match_ingredients(&mut ai_map);
    assert_eq!(
        "pgnpx,srmsh,ksdgk,dskjpq,nvbrx,khqsk,zbkbgp,xzb",
        order_ingredients(&ai_map)
    );
    println!("All done")
}

fn order_ingredients(ai_map: &HashMap<&str, HashSet<&str>>) -> String {
    ai_map
        .into_iter()
        .map(|(allergen, ingredients)| {
            let ingredient = ingredients.into_iter().cloned().next().unwrap();
            (*allergen, ingredient)
        })
        .sorted_by_key(|(a, _)| *a)
        .map(|(_, ing)| ing)
        .join(",")
}

fn match_ingredients(ai_map: &mut HashMap<&str, HashSet<&str>>) {
    let total = ai_map.len();
    let mut unique_ingredients = HashSet::new();

    while unique_ingredients.len() < total {
        for (_, ingredients) in ai_map.iter_mut() {
            if ingredients.len() == 1 {
                let ing = ingredients.iter().next().cloned().unwrap();
                unique_ingredients.insert(ing);
            } else {
                ingredients.retain(|x| !unique_ingredients.contains(x));
            }
        }
    }
}

fn count_non_allergy(
    ai_map: &HashMap<&str, HashSet<&str>>,
    lines: &[(Vec<&str>, Vec<&str>)],
) -> usize {
    let allergic: HashSet<_> = ai_map.values().flatten().cloned().collect();
    lines
        .into_iter()
        .map(|(ingredients, _)| {
            ingredients
                .into_iter()
                .filter(|item| !allergic.contains(**item))
                .count()
        })
        .sum()
}

fn find_ingredients<'a>(lines: &'a [(Vec<&str>, Vec<&str>)]) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut ai_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (ingredients, allergens) in lines {
        for allergen in allergens {
            match ai_map.get_mut(allergen) {
                Some(v) => {
                    let ing: HashSet<_> = ingredients.iter().cloned().collect();
                    *v = v.intersection(&ing).cloned().collect()
                }
                None => {
                    ai_map.insert(*allergen, ingredients.into_iter().cloned().collect());
                }
            }
        }
    }

    ai_map
}

fn read(input: &str) -> Result<Vec<(Vec<&str>, Vec<&str>)>, Error> {
    input.lines().map(read_line).collect()
}

fn read_line(line: &str) -> Result<(Vec<&str>, Vec<&str>), Error> {
    let mut pair = line.split("(contains");
    let ingredients = pair
        .next()
        .and_then(|s| Some(s.trim().split_ascii_whitespace().collect()))
        .ok_or_else(|| "Invalid input")?;
    let allergens = pair
        .next()
        .and_then(|s| s.trim().strip_suffix(')'))
        .and_then(|s| Some(s.split(", ").collect()))
        .ok_or_else(|| "Invalid input")?;
    Ok((ingredients, allergens))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    trh fvjkl sbzzf mxmxvkd (contains dairy)
    sqjhc fvjkl (contains soy)
    sqjhc mxmxvkd sbzzf (contains fish)"#;

    #[test]
    fn test() {
        let lines = read(SAMPLE).unwrap();
        let mut ai_map = find_ingredients(&lines);
        assert_eq!(5, count_non_allergy(&ai_map, &lines));
        match_ingredients(&mut ai_map);
        assert_eq!("mxmxvkd,sqjhc,fvjkl", order_ingredients(&ai_map))
    }
}
