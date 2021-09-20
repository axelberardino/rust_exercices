const TARGET: &str = "https://adventofcode.com/2020/day/3/input";

type Slope = Vec<Vec<char>>;

// How to get a debug? (fmt::Display for alias?)
fn print_slopes(slopes: &Slope) {
    for lines in slopes {
        for c in lines {
            print!("{}", c);
        }
        println!();
    }
}

fn custom_parse(s: &str, line: u32) -> Result<Vec<char>, common::LineError> {
    let row = s
        .chars()
        .enumerate()
        .map(|(pos, c)| {
            if c == '.' || c == '#' {
                Ok(c)
            } else {
                Err(common::LineError {
                    line: line,
                    char: pos as u32,
                    content: String::from(s),
                    msg: format!("invalid character {}", c),
                })
            }
        })
        .collect::<Result<Vec<char>, common::LineError>>()?;
    Ok(row)
}

/// gets the slopes from the distant payload.
async fn fetch_slopes() -> common::AnyResult<Slope> {
    let body = common::fetch_payload(TARGET).await?;
    let lines = body
        .split("\n")
        // .inspect(|s| println!("XXXXXXXX {}", s))
        .filter(|s| *s != "")
        .enumerate()
        .map(|(i, s)| custom_parse(s, i as u32))
        .collect::<Result<Slope, common::LineError>>()?;
    Ok(lines)
}

fn solve(slope: &Slope, horizontal_shift: u32, vertical_shift: u32) -> u32 {
    let nb = slope
        .iter()
        .skip(vertical_shift as usize)
        .step_by(vertical_shift as usize)
        .enumerate()
        // .inspect(|x| println!("{:?}", x))
        .filter(|(i, line)| line[((i + 1) * horizontal_shift as usize) % line.len()] == '#')
        .count();
    nb as u32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let slopes = fetch_slopes().await?;
    // println!("{:?}", slopes);
    print_slopes(&slopes);
    // Part 1
    println!("{}", solve(&slopes, 3, 1));
    // Part 2
    let res = vec![
        solve(&slopes, 1, 1),
        solve(&slopes, 3, 1),
        solve(&slopes, 5, 1),
        solve(&slopes, 7, 1),
        solve(&slopes, 1, 2),
    ];
    let mul: u64 = res.iter().fold(1, |x, y| x * (*y as u64));
    // let mul: u64 = res.iter().product();
    println!("{:?} = {}", res, mul);
    Ok(())
}
