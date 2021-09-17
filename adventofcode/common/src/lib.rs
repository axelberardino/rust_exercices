extern crate reqwest;

use std::error::Error;
use std::fmt;

const SESSION: &str = "53616c7465645f5ff2ccad970e7c40469c1c30bcd0570f4fcf985788bae11aa19a289f752642a87cf59fdad08d36644c";

/// The Errors that may occur when processing a `test file numbers`.
#[derive(Debug, Clone)]
pub struct LineError {
    pub line: u32,
    pub content: String,
    pub msg: String,
}

impl Error for LineError {}

impl fmt::Display for LineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: invalid line {}, {}",
            self.line, self.content, self.msg
        )
    }
}

/// gets the payload from the distant url, with the correct session set.
pub async fn fetch_payload(target: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(target)
        .header("Cookie", format!("session={}", SESSION))
        .send()
        .await?;
    Ok(res.text().await?)
}
