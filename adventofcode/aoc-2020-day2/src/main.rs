const TARGET: &str = "https://adventofcode.com/2020/day/2/input";

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct RangeExpr {
    from: u8,
    to: u8,
    symbol: char,
    expr: String,
}

#[derive(Debug)]
enum StateMachine {
    None,
    From,
    Range,
    To,
    Space1,
    Symbol,
    SemiColon,
    Space2,
    Expr,
}

/// to_num convert a char to a num.
fn to_num(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => 0,
    }
}

/// parse a string into a range expr, with custom error.
/// Grammar is: [0-9]-[0-9] [a-z]: [a-z]+
fn custom_parse(s: &str, line: u32) -> Result<RangeExpr, common::LineError> {
    let mut expr = RangeExpr {
        from: 0,
        to: 0,
        symbol: 'a',
        expr: String::from(""),
    };
    let mut state = StateMachine::None;
    let mut i = 0;
    let mut it = s.chars();
    while let Some(c) = it.next() {
        match c {
            '0'..='9' => {
                match state {
                    StateMachine::None => {
                        state = StateMachine::From;
                        expr.from = to_num(c);
                    }
                    StateMachine::From => {
                        expr.from = expr.from * 10 + to_num(c);
                    }
                    StateMachine::Range => {
                        state = StateMachine::To;
                        expr.to = to_num(c);
                    }
                    StateMachine::To => {
                        expr.to = expr.to * 10 + to_num(c);
                    }
                    _ => {
                        return Err(common::LineError {
                            line: line,
                            char: i,
                            content: String::from(s),
                            msg: String::from("numbers wasn't expected"),
                        });
                    }
                };
            }
            '-' => {
                match state {
                    StateMachine::From => {}
                    _ => {
                        return Err(common::LineError {
                            line: line,
                            char: i,
                            content: String::from(s),
                            msg: String::from("hyphen wasn't expected"),
                        });
                    }
                }

                state = StateMachine::Range;
            }
            ':' => {
                match state {
                    StateMachine::Symbol => {}
                    _ => {
                        return Err(common::LineError {
                            line: line,
                            char: i,
                            content: String::from(s),
                            msg: String::from("semi-colon wasn't expected"),
                        });
                    }
                };
                state = StateMachine::SemiColon;
            }
            ' ' => {
                match state {
                    StateMachine::To => {
                        state = StateMachine::Space1;
                    }
                    StateMachine::SemiColon => {
                        state = StateMachine::Space2;
                    }
                    _ => {
                        return Err(common::LineError {
                            line: line,
                            char: i,
                            content: String::from(s),
                            msg: String::from("space wasn't expected"),
                        });
                    }
                };
            }
            'a'..='z' => {
                match state {
                    StateMachine::Space1 => {
                        state = StateMachine::Symbol;
                        expr.symbol = c;
                    }
                    StateMachine::Space2 => {
                        state = StateMachine::Expr;
                        expr.expr = String::from(c);
                    }
                    StateMachine::Expr => {
                        expr.expr = format!("{}{}", expr.expr, c);
                    }
                    _ => {
                        return Err(common::LineError {
                            line: line,
                            char: i,
                            content: String::from(s),
                            msg: String::from("char wasn't expected"),
                        });
                    }
                };
            }
            _ => {
                return Err(common::LineError {
                    line: line,
                    char: i,
                    content: String::from(s),
                    msg: format!("unexpected character {}", c),
                });
            }
        };
        i += 1;
    }
    Ok(expr)
}

/// gets the rules from the distant payload.
async fn fetch_rules() -> Result<Vec<RangeExpr>, Box<dyn std::error::Error>> {
    let body = common::fetch_payload(TARGET).await?;
    let lines = body
        .split("\n")
        // .inspect(|s| println!("XXXXXXXX {}", s))
        .filter(|s| *s != "")
        .enumerate()
        .map(|(i, s)| custom_parse(s, i as u32))
        .collect::<Result<Vec<RangeExpr>, common::LineError>>()?;
    Ok(lines)
}

/// checks the given password stored as a range is valid.
fn valid_password1(re: &RangeExpr) -> bool {
    let nb = re.expr.chars().filter(|x| *x == re.symbol).count();
    nb >= re.from as usize && nb <= re.to as usize
}

/// checks the given password stored as a range is valid.
fn valid_password2(re: &RangeExpr) -> bool {
    let mut nb = 0;
    if re.expr.chars().nth((re.from - 1) as usize).unwrap() == re.symbol {
        nb += 1;
    }
    if re.expr.chars().nth((re.to - 1) as usize).unwrap() == re.symbol {
        nb += 1;
    }
    nb == 1
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rules = fetch_rules().await?;
    // println!("{:?}", rules);
    let nb = rules.iter().filter(|x| valid_password1(x)).count();
    println!("{}", nb);
    let nb = rules.iter().filter(|x| valid_password2(x)).count();
    println!("{}", nb);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_custom_parse() {
        assert_eq!(
            custom_parse(""),
            // Ok(RangeExpr { // FIXME, not working!
            //     from: 0,
            //     to: 0,
            //     symbol: 'a',
            //     expr: String::from(""),
            // }),
            Err(common::LineError {
                line: line,
                char: 32,
                content: String::from("toto"),
                msg: format!("unexpected character {}", 'a'),
            }),
        );
    }
}
