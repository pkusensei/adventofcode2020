use std::collections::HashMap;

use itertools::Itertools;

type Error = Box<dyn std::error::Error>;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

fn main() {
    let input = include_str!("..\\input.txt");
    let tiles = read(&input, "\r\n\r\n").unwrap();
    let (corners, _tiles) = find_corners(tiles);
    assert_eq!(28057939502729, corners.into_iter().product::<usize>());
}

fn find_corners(mut tiles: Vec<Tile>) -> (Vec<usize>, HashMap<usize, Tile>) {
    let neighbors: HashMap<_, _> = tiles
        .iter()
        .map(|tile| tile.find_neighbors(&tiles))
        .collect();
    for tile in tiles.iter_mut() {
        tile.neighbors = neighbors[&tile.id].clone()
    }
    let corners = tiles
        .iter()
        .filter_map(|tile| match tile.neighbors.len() {
            2 => Some(tile.id),
            _ => None,
        })
        .collect();
    let tiles = tiles.into_iter().map(|t| (t.id, t)).collect();
    (corners, tiles)
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    dots: [[bool; WIDTH]; HEIGHT],
    neighbors: Vec<usize>,
}

impl Tile {
    fn get_possible_borders(&self) -> Vec<[bool; WIDTH]> {
        let mut borders = vec![];
        let top = self.dots[0];
        borders.push(top);
        borders.push(Self::reverse_border(top));
        let btm = self.dots[HEIGHT - 1];
        borders.push(btm);
        borders.push(Self::reverse_border(btm));

        let mut left = [false; HEIGHT];
        for (idx, values) in self.dots.iter().enumerate() {
            left[idx] = values[0]
        }
        borders.push(left);
        borders.push(Self::reverse_border(left));

        let mut right = [false; HEIGHT];
        for (idx, values) in self.dots.iter().enumerate() {
            right[idx] = values[WIDTH - 1]
        }
        borders.push(right);
        borders.push(Self::reverse_border(right));

        borders
    }

    fn find_neighbors(&self, tiles: &[Tile]) -> (usize, Vec<usize>) {
        let borders = self.get_possible_borders();
        let ns = tiles
            .into_iter()
            .filter_map(|t| {
                if t.id == self.id {
                    None
                } else if t
                    .get_possible_borders()
                    .iter()
                    .cartesian_product(borders.iter())
                    .filter(|(other, this)| **other == **this)
                    .count()
                    == 2
                // count in reverse ordered border
                {
                    Some(t.id)
                } else {
                    None
                }
            })
            .collect();
        (self.id, ns)
    }

    fn reverse_border(b: [bool; WIDTH]) -> [bool; WIDTH] {
        let mut r = [false; WIDTH];
        for (idx, value) in r.iter_mut().enumerate() {
            *value = b[WIDTH - 1 - idx]
        }
        r
    }
}

fn read(input: &str, split_pattern: &str) -> Result<Vec<Tile>, Error> {
    let tiles: Vec<_> = input.split(split_pattern).collect();

    let mut res = vec![];
    for tile in tiles {
        let lines: Vec<_> = tile.split('\n').collect();
        let id = lines[0]
            .strip_prefix("Tile ")
            .and_then(|s| s.trim().strip_suffix(':'))
            .ok_or_else(|| "Invalid input")?
            .parse()?;

        let mut dots = [[false; 10]; 10];
        for (row, line) in lines.into_iter().skip(1).enumerate() {
            for (col, ch) in line.trim().char_indices() {
                dots[row][col] = ch == '#'; // '#' becomes `true`
            }
        }
        res.push(Tile {
            id,
            dots,
            neighbors: Vec::with_capacity(4),
        });
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;

    #[test]
    fn test_p1() {
        let tiles = read(SAMPLE, "\n\n").unwrap();
        assert_eq!(
            20899048083289,
            (find_corners(tiles).0.into_iter().product::<usize>())
        );
    }
}
