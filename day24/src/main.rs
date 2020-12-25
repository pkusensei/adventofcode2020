use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("..\\input.txt");
    let tiles = get_tiles(input);
    assert_eq!(326, count_black(&tiles));
    let tiles = flip(tiles, 100);
    assert_eq!(3979, count_black(&tiles));
}

fn flip(mut tiles: HashMap<(i32, i32, i32), bool>, times: u32) -> HashMap<(i32, i32, i32), bool> {
    for _ in 0..times {
        let expanded: Vec<_> = tiles.keys().map(get_neighbors).flatten().unique().collect();
        let new_tiles = expanded
            .into_iter()
            .map(|tile| {
                let count = get_neighbors(&tile)
                    .into_iter()
                    .filter(|t| match tiles.get(t) {
                        Some(true) => true,
                        _ => false,
                    })
                    .count();
                match tiles.get(&tile) {
                    Some(true) => {
                        if count == 0 || count > 2 {
                            (tile, true)
                        } else {
                            (tile, false)
                        }
                    }
                    _ => {
                        if count == 2 {
                            (tile, true)
                        } else {
                            (tile, false)
                        }
                    }
                }
            })
            .collect();
        tiles = new_tiles;
    }
    tiles
}

fn get_neighbors(tile: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    const DELTAS: [(i32, i32, i32); 6] = [
        (-1, 1, 0),
        (0, 1, -1),
        (1, 0, -1),
        (1, -1, 0),
        (0, -1, 1),
        (-1, 0, 1),
    ];
    DELTAS
        .iter()
        .map(|(dx, dy, dz)| (tile.0 + dx, tile.1 + dy, tile.2 + dz))
        .collect()
}

fn count_black(tiles: &HashMap<(i32, i32, i32), bool>) -> usize {
    tiles.values().filter(|&&v| v == true).count()
}

fn get_tiles(input: &str) -> HashMap<(i32, i32, i32), bool> {
    let mut tiles: HashMap<_, bool> = HashMap::new();
    for line in input.lines() {
        let tile = find_tile(line);
        match tiles.get_mut(&tile) {
            Some(v) => *v = !(*v),
            None => {
                tiles.insert(tile, true); // true as flipped once -> black
            }
        }
    }
    tiles
}

// https://www.redblobgames.com/grids/hexagons/#coordinates-cube
fn parse_line(line: &[char]) -> Vec<(i32, i32, i32)> {
    if line.is_empty() {
        return vec![];
    }
    match line[0] {
        'w' => {
            let mut rest = parse_line(&line[1..]);
            rest.insert(0, (-1, 1, 0));
            rest
        }
        'e' => {
            let mut rest = parse_line(&line[1..]);
            rest.insert(0, (1, -1, 0));
            rest
        }
        'n' => {
            let mut rest = parse_line(&line[2..]);
            match line[1] {
                'w' => rest.insert(0, (0, 1, -1)),
                'e' => rest.insert(0, (1, 0, -1)),
                _ => unreachable!(),
            }
            rest
        }
        's' => {
            let mut rest = parse_line(&line[2..]);
            match line[1] {
                'w' => rest.insert(0, (-1, 0, 1)),
                'e' => rest.insert(0, (0, -1, 1)),
                _ => unreachable!(),
            }
            rest
        }
        _ => unreachable!(),
    }
}

fn find_tile(line: &str) -> (i32, i32, i32) {
    let line: Vec<_> = line.trim().chars().collect();
    parse_line(&line)
        .into_iter()
        .fold((0, 0, 0), |acc, i| (acc.0 + i.0, acc.1 + i.1, acc.2 + i.2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_tile() {
        const S1: &str = "esew";
        assert_eq!((0, -1, 1), find_tile(&S1));
        const S2: &str = "nwwswee";
        assert_eq!((0, 0, 0), find_tile(&S2));
    }

    const SAMPLE: &str = r#"sesenwnenenewseeswwswswwnenewsewsw
    neeenesenwnwwswnenewnwwsewnenwseswesw
    seswneswswsenwwnwse
    nwnwneseeswswnenewneswwnewseswneseene
    swweswneswnenwsewnwneneseenw
    eesenwseswswnenwswnwnwsewwnwsene
    sewnenenenesenwsewnenwwwse
    wenwwweseeeweswwwnwwe
    wsweesenenewnwwnwsenewsenwwsesesenwne
    neeswseenwwswnwswswnw
    nenwswwsewswnenenewsenwsenwnesesenew
    enewnwewneswsewnwswenweswnenwsenwsw
    sweneswneswneneenwnewenewwneswswnese
    swwesenesewenwneswnwwneseswwne
    enesenwswwswneneswsenwnewswseenwsese
    wnwnesenesenenwwnenwsewesewsesesew
    nenewswnwewswnenesenwnesewesw
    eneswnwswnwsenenwnwnwwseeswneewsenese
    neswnwewnwnwseenwseesewsenwsweewe
    wseweeenwnesenwwwswnew"#;

    #[test]
    fn test_count_black() {
        let tiles = get_tiles(SAMPLE);
        assert_eq!(10, count_black(&tiles));
    }

    #[test]
    fn test_flip_times() {
        let tiles = get_tiles(SAMPLE);
        let flipped = flip(tiles, 1);
        println!("== {} == ", count_black(&flipped));
    }
}
