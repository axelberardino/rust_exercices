const TARGET: &str = "https://adventofcode.com/2020/day/2/input";

/// parse a string into a u32, with custom error.
fn custom_parse(s: &str) -> Result<(u32, u32, char, String), common::LineError> {
    let mut it = s.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
            }
            'a' | 'b' => {
                it.next();
            }
            ' ' => {

            },
            '-' => {

            },
            ':' => {

            },
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            ' ' => {
                it.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }

    s.split(':')

    match s.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(e) => Err(common::LineError {
            line: 0, // How to get the line number?
            content: String::from(s),
            msg: String::from(format!("{}", e)),
        }),
    }
}

/// gets the rules from the distant payload.
async fn fetch_rules() -> Result<Vec<(u32, u32, char, String)>, Box<dyn std::error::Error>> {
    let body = common::fetch_payload(TARGET).await?;
    let lines = body
        .split("\n")
        // .inspect(|s| println!("XXXXXXXX {}", s))
        .filter(|s| *s != "")
        .map(|s| custom_parse(s))
        .collect::<Result<Vec<u32>, common::LineError>>()?;
    Ok(lines)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = fetch_numbers().await?;
    println!("{:?}", numbers);
    Ok(())
}
