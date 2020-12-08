use std::str::FromStr;

use tools::Error;

fn main() {
    let input = tools::read_input("input.txt").unwrap();
    let lines: Vec<&str> = input.iter().map(|s| s.as_str()).collect();
    let mut console = Console::from_strings(&lines).unwrap();
    println!("Acc before loop: {}", console.run().unwrap());
    println!("Acc after fix: {}", console.fix_instruction().unwrap());
}

#[derive(Debug, Clone)]
struct Console<'a> {
    accumulator: i32,
    ip: usize,
    instructions: Vec<(&'a str, i32)>,
    inst_order: Vec<usize>,
}

impl<'a> Console<'a> {
    fn from_strings(lines: &'a [&'a str]) -> Result<Self, Error> {
        let instructions = lines
            .into_iter()
            .map(|line| -> Result<(&str, i32), Error> {
                let mut op_arg = line.trim().split_ascii_whitespace();
                let op = op_arg
                    .next()
                    .ok_or_else(|| -> Error { "Input error".into() })?;
                let arg = i32::from_str(
                    op_arg
                        .next()
                        .ok_or_else(|| -> Error { "Input error".into() })?,
                )?;
                Ok((op, arg))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(Self {
            accumulator: 0,
            ip: 0,
            instructions,
            inst_order: vec![],
        })
    }

    fn run_one_inst(&mut self) -> Result<(), Error> {
        let (op, arg) = self.instructions[self.ip];
        self.inst_order.push(self.ip);
        match op {
            "acc" => {
                self.accumulator += arg;
                self.ip += 1;
                Ok(())
            }
            "jmp" => {
                self.ip = if arg.is_negative() {
                    self.ip.saturating_sub(arg.wrapping_abs() as usize)
                } else {
                    self.ip + arg as usize
                };
                Ok(())
            }
            "nop" => {
                self.ip += 1;
                Ok(())
            }
            _ => Err(format!("Invalid op: {}", op).into()),
        }
    }

    fn run(&mut self) -> Result<i32, Error> {
        while !self.inst_order.contains(&self.ip) && self.ip < self.instructions.len() {
            self.run_one_inst()?
        }
        Ok(self.accumulator)
    }

    fn fix_instruction(&mut self) -> Result<i32, Error> {
        let jmp_or_nop: Vec<_> = self
            .instructions
            .iter()
            .enumerate()
            .filter_map(|(idx, (op, _))| {
                if *op == "jmp" || *op == "nop" {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        for idx in jmp_or_nop {
            match self.instructions[idx] {
                ("jmp", arg) => self.instructions[idx] = ("nop", arg),
                ("nop", arg) => self.instructions[idx] = ("jmp", arg),
                _ => {
                    return Err(format!("Wrong op: {} at {}", self.instructions[idx].0, idx).into())
                }
            }
            self.run()?;
            if self.ip == self.instructions.len() {
                break;
            } else {
                self.reset(idx)?
            }
        }
        match self.ip {
            0 => Err("Error fixing code".into()),
            _ => Ok(self.accumulator),
        }
    }

    fn reset(&mut self, idx: usize) -> Result<(), Error> {
        self.accumulator = 0;
        self.ip = 0;
        self.inst_order.clear();

        match self.instructions[idx] {
            ("jmp", arg) => self.instructions[idx] = ("nop", arg),
            ("nop", arg) => self.instructions[idx] = ("jmp", arg),
            _ => return Err(format!("Wrong op: {} at {}", self.instructions[idx].0, idx).into()),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const sample: &str = r#"nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6"#;

    #[test]
    fn test_console() {
        let lines = sample.split('\n').collect::<Vec<_>>();
        let mut console = Console::from_strings(&lines).unwrap();
        assert_eq!(console.run().unwrap(), 5);
        assert_eq!(console.fix_instruction().unwrap(), 8)
    }
}
