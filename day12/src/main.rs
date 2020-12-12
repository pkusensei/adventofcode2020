use std::str::FromStr;

type Error = Box<dyn std::error::Error>;

fn main() {
    let input = include_str!(r#"..\input.txt"#);
    let insts = Instruction::read(input).unwrap();
    {
        let mut ship = Ship::new();
        ship.apply_insts(&insts, Ship::apply_inst).unwrap();
        debug_assert_eq!(2879, ship.manhattan_dist())
    }
    {
        let mut ship = Ship::new();
        ship.apply_insts(&insts, Ship::apply_inst_wp).unwrap();
        debug_assert_eq!(178986, ship.manhattan_dist())
    }
    println!("All done")
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn rotate(&self, degree: i32) -> Result<Self, Error> {
        match degree {
            90 | -270 => match self {
                Self::East => Ok(Self::North),
                Self::North => Ok(Self::West),
                Self::West => Ok(Self::South),
                Self::South => Ok(Self::East),
            },
            -90 | 270 => match self {
                Self::East => Ok(Self::South),
                Self::South => Ok(Self::West),
                Self::West => Ok(Self::North),
                Self::North => Ok(Self::East),
            },
            180 | -180 => match self {
                Self::East => Ok(Self::West),
                Self::West => Ok(Self::East),
                Self::North => Ok(Self::South),
                Self::South => Ok(Self::North),
            },
            _ => Err(format!("Invalid rotation degree {}", degree).into()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    N(i32),
    S(i32),
    W(i32),
    E(i32),
    L(i32),
    R(i32),
    F(i32),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let num = i32::from_str(&s[1..])?;
        match s.chars().next() {
            Some('N') => Ok(Self::N(num)),
            Some('S') => Ok(Self::S(num)),
            Some('W') => Ok(Self::W(num)),
            Some('E') => Ok(Self::E(num)),
            Some('L') => Ok(Self::L(num)),
            Some('R') => Ok(Self::R(num)),
            Some('F') => Ok(Self::F(num)),
            _ => Err(format!("Invalid input {}", s).into()),
        }
    }
}

impl Instruction {
    fn read(input: &str) -> Result<Vec<Instruction>, Error> {
        let insts = input
            .split_ascii_whitespace()
            .map(|line| Instruction::from_str(line))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(insts)
    }
}

#[derive(Debug, Copy, Clone)]
struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn new() -> Self {
        Self { x: 10, y: 1 }
    }

    fn rotate(&mut self, degree: i32) -> Result<(), Error> {
        let (x, y) = match degree {
            90 | -270 => (-self.y, self.x),
            -90 | 270 => (self.y, -self.x),
            180 | -180 => (-self.x, -self.y),
            _ => return Err(format!("Invalid rotation degree {}", degree).into()),
        };
        self.x = x;
        self.y = y;
        Ok(())
    }

    fn apply_inst(&mut self, inst: &Instruction) -> Result<(), Error> {
        match inst {
            Instruction::N(num) => self.y += num,
            Instruction::S(num) => self.y -= num,
            Instruction::W(num) => self.x -= num,
            Instruction::E(num) => self.x += num,
            Instruction::L(num) => self.rotate(*num)?,
            Instruction::R(num) => self.rotate(-num)?,
            Instruction::F(_) => return Err("F(value) does not apply to waypoint".into()),
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
struct Ship {
    x: i32,
    y: i32,
    dir: Direction,
    wp: Waypoint,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: Direction::East,
            wp: Waypoint::new(),
        }
    }

    fn manhattan_dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn apply_inst(&mut self, inst: &Instruction) -> Result<(), Error> {
        match inst {
            Instruction::N(num) => self.y += num,
            Instruction::S(num) => self.y -= num,
            Instruction::W(num) => self.x -= num,
            Instruction::E(num) => self.x += num,
            Instruction::L(num) => self.dir = self.dir.rotate(*num)?,
            Instruction::R(num) => self.dir = self.dir.rotate(-num)?,
            Instruction::F(num) => match self.dir {
                Direction::North => self.y += num,
                Direction::South => self.y -= num,
                Direction::West => self.x -= num,
                Direction::East => self.x += num,
            },
        }
        Ok(())
    }

    fn apply_insts(
        &mut self,
        insts: &[Instruction],
        func: fn(&mut Ship, &Instruction) -> Result<(), Error>,
    ) -> Result<(), Error> {
        for inst in insts {
            func(self, inst)?;
        }
        Ok(())
    }

    fn apply_inst_wp(&mut self, inst: &Instruction) -> Result<(), Error> {
        match inst {
            Instruction::F(num) => {
                self.x += num * self.wp.x;
                self.y += num * self.wp.y;
                Ok(())
            }
            _ => self.wp.apply_inst(inst),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"F10
    N3
    F7
    R90
    F11"#;

    #[test]
    fn test_manhattan_dist() {
        let insts = Instruction::read(SAMPLE).unwrap();
        let mut ship = Ship::new();
        ship.apply_insts(&insts, Ship::apply_inst).unwrap();
        assert_eq!(25, ship.manhattan_dist())
    }

    #[test]
    fn test_manhattan_dist_wp() {
        let insts = Instruction::read(SAMPLE).unwrap();
        let mut ship = Ship::new();
        ship.apply_insts(&insts, Ship::apply_inst_wp).unwrap();
        assert_eq!(286, ship.manhattan_dist())
    }
}
