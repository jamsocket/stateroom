use crate::API_BASE;
use anyhow::{anyhow, Result};
use reqwest::StatusCode;

pub fn authenticate(token: &str) -> Result<bool> {
    let url = format!("{}authcheck", API_BASE);

    let client = reqwest::blocking::Client::new();
    let res = client.get(url).query(&[("token", token)]).send()?;

    if res.status().is_success() {
        Ok(true)
    } else if res.status() == StatusCode::FORBIDDEN {
        Ok(false)
    } else {
        Err(anyhow!("Unexpected error code: {}", res.status()))
    }
}
