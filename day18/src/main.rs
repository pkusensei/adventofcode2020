type Error = Box<dyn std::error::Error>;

fn main() {
    let input = include_str!("..\\input.txt");
    assert_eq!(24650385570008, p1(input));
    assert_eq!(158183007916215, p2(input));
    println!("All done")
}

fn p1(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .filter_map(Result::ok)
        .map(|tokens| evaluate(&tokens, 0, true, false).0)
        .sum()
}

fn p2(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .filter_map(Result::ok)
        .map(|tokens| evaluate(&tokens, 0, false, false).0)
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Num(i64),
    Op(BinaryOp),
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone, Copy)]
enum BinaryOp {
    Add,
    Mul,
}

fn evaluate(tokens: &[Token], mut idx: usize, no_precedence: bool, deepdown: bool) -> (i64, usize) {
    let mut res = 0;
    let mut op = BinaryOp::Add;
    while idx < tokens.len() {
        match tokens[idx] {
            Token::Num(num) => match op {
                BinaryOp::Add => res += num,
                BinaryOp::Mul => res *= num,
            },
            Token::Op(bin_op) => match bin_op {
                BinaryOp::Add => op = BinaryOp::Add,
                BinaryOp::Mul => {
                    if no_precedence {
                        op = BinaryOp::Mul;
                    } else if deepdown {
                        return (res, idx - 1);
                    } else {
                        let (sub_result, i) = evaluate(tokens, idx + 1, no_precedence, true);
                        res *= sub_result;
                        idx = i
                    }
                }
            },
            Token::LeftParen => {
                let (sub_result, i) = evaluate(tokens, idx + 1, no_precedence, false);
                match op {
                    BinaryOp::Add => res += sub_result,
                    BinaryOp::Mul => res *= sub_result,
                }
                idx = i
            }
            Token::RightParen => {
                if deepdown {
                    return (res, idx - 1);
                } else {
                    return (res, idx);
                }
            }
        }
        idx += 1
    }
    (res, idx)
}

fn parse_line(line: &str) -> Result<Vec<Token>, Error> {
    let mut chars = line.chars().filter(|c| !c.is_ascii_whitespace()).peekable();
    let mut tokens = vec![];
    while let Some(c) = chars.next() {
        match c {
            '+' => tokens.push(Token::Op(BinaryOp::Add)),
            '*' => tokens.push(Token::Op(BinaryOp::Mul)),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '0'..='9' => {
                let mut num = String::new();
                num.push(c);
                while let Some(next_char) = chars.peek() {
                    if next_char.is_ascii_digit() {
                        num.push(*next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Num(num.parse()?))
            }
            _ => unreachable!(),
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        {
            let tokens = parse_line("1 + 2 * 3 + 4 * 5 + 6").unwrap();
            assert_eq!(71, evaluate(&tokens, 0, true, false).0)
        }
        {
            let tokens = parse_line("2 * 3 + (4 * 5)").unwrap();
            assert_eq!(26, evaluate(&tokens, 0, true, false).0)
        }
        {
            let tokens = parse_line("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap();
            assert_eq!(437, evaluate(&tokens, 0, true, false).0)
        }
        {
            let tokens = parse_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap();
            assert_eq!(12240, evaluate(&tokens, 0, true, false).0)
        }
        {
            let tokens = parse_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap();
            assert_eq!(13632, evaluate(&tokens, 0, true, false).0)
        }
    }

    #[test]
    fn test_p2() {
        {
            let tokens = parse_line("1 + 2 * 3 + 4 * 5 + 6").unwrap();
            assert_eq!(231, evaluate(&tokens, 0, false, false).0)
        }
        {
            let tokens = parse_line("1 + (2 * 3) + (4 * (5 + 6))").unwrap();
            assert_eq!(51, evaluate(&tokens, 0, false, false).0)
        }
        {
            let tokens = parse_line("2 * 3 + (4 * 5)").unwrap();
            assert_eq!(46, evaluate(&tokens, 0, false, false).0)
        }
        {
            let tokens = parse_line("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap();
            assert_eq!(1445, evaluate(&tokens, 0, false, false).0)
        }
        {
            let tokens = parse_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap();
            assert_eq!(669060, evaluate(&tokens, 0, false, false).0)
        }
        {
            let tokens = parse_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap();
            assert_eq!(23340, evaluate(&tokens, 0, false, false).0)
        }
    }
}
