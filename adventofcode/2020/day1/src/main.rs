extern crate reqwest;

use std::error::Error;
use std::fmt;

const TARGET: &str = "https://adventofcode.com/2020/day/1/input";
const SESSION: &str = "53616c7465645f5ff2ccad970e7c40469c1c30bcd0570f4fcf985788bae11aa19a289f752642a87cf59fdad08d36644c";

/// The Errors that may occur when processing a `test file numbers`.
#[derive(Debug, Clone)]
struct FileNumberError {
    line: u32,
    content: String,
    msg: String,
}

impl Error for FileNumberError {}

impl fmt::Display for FileNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: invalid numbers {}, {}",
            self.line, self.content, self.msg
        )
    }
}

/// gets the payload from the distant url, with the correct session set.
async fn fetch_payload() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(TARGET)
        .header("Cookie", format!("session={}", SESSION))
        .send()
        .await?;
    Ok(res.text().await?)
}

/// parse a string into a u32, with custom error.
fn custom_parse(s: &str) -> Result<u32, FileNumberError> {
    match s.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(e) => Err(FileNumberError {
            line: 0, // How to get the line number?
            content: String::from(s),
            msg: String::from(format!("{}", e)),
        }),
    }
}

/// gets the numbers from the distant payload.
async fn fetch_numbers() -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let body = fetch_payload().await?;
    let lines = body
        .split("\n")
        // .inspect(|s| println!("XXXXXXXX {}", s))
        .filter(|s| *s != "")
        .map(|s| custom_parse(s))
        .collect::<Result<Vec<u32>, FileNumberError>>()?;
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
