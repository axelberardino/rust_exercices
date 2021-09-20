const TARGET: &str = "https://adventofcode.com/2020/day/1/input";

/// parse a string into a u32, with custom error.
fn custom_parse(s: &str, i: u32) -> Result<u32, common::LineError> {
    match s.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(e) => Err(common::LineError {
            line: i, // How to get the line number?
            content: String::from(s),
            char: 0,
            msg: String::from(format!("{}", e)),
        }),
    }
}

/// gets the numbers from the distant payload.
async fn fetch_numbers() -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let body = common::fetch_payload(TARGET).await?;
    let lines = body
        .split("\n")
        // .inspect(|s| println!("XXXXXXXX {}", s))
        .filter(|s| *s != "")
        .enumerate()
        .map(|(i, s)| custom_parse(s, i as u32))
        .collect::<Result<Vec<u32>, common::LineError>>()?;
    Ok(lines)
}

/// finds the first summable couple.
fn find_summable_couple(numbers: &Vec<u32>, value: u32) -> (u32, u32, u32) {
    for (index, first) in numbers.iter().enumerate() {
        if let Some(second) = numbers.iter().skip(index).find(|n| *n + *first == value) {
            return (*second, *first, *first * *second);
        }
    }
    (0, 0, 0)
}

/// finds the first summable tuple.
fn find_summable_tuple(numbers: &Vec<u32>, value: u32) -> (u32, u32, u32, u32) {
    for (index, first) in numbers.iter().enumerate() {
        for (sub_index, second) in numbers.iter().skip(index).enumerate() {
            if let Some(third) = numbers
                .iter()
                .skip(sub_index)
                .find(|n| *n + *first + *second == value)
            {
                return (*second, *first, *third, *first * *second * *third);
            }
        }
    }
    (0, 0, 0, 0)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = fetch_numbers().await?;
    println!("{:?}", numbers);
    let (first, second, mult) = find_summable_couple(&numbers, 2020);
    println!("PART1: res is {} + {} => {}", first, second, mult);
    let (first, second, third, mult) = find_summable_tuple(&numbers, 2020);
    println!(
        "PART2: res is {} + {} + {} => {}",
        first, second, third, mult
    );
    Ok(())
}
