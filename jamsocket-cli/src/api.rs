use crate::API_BASE;
use anyhow::{anyhow, Result};
use reqwest::StatusCode;
use serde::Deserialize;

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

#[derive(Deserialize)]
pub struct UploadServiceResponse {
    pub module: String,
    pub service: String,
}

pub fn upload(token: &str, service_id: &str, module: &[u8]) -> Result<UploadServiceResponse> {
    let url = format!("{}service/{}/module", API_BASE, service_id);

    let client = reqwest::blocking::Client::new();
    let res = client.post(url).query(&[("token", token)]).body(module.to_vec()).send()?;

    match res.status() {
        StatusCode::NOT_FOUND => Err(anyhow!("service_id not found.")),
        StatusCode::FORBIDDEN => Err(anyhow!("Configured token is not authorized for given service_id.")),
        StatusCode::OK => {
            res.json::<UploadServiceResponse>().map_err(|e| e.into())
        }
        sc => Err(anyhow!("Received error status code from jamsocket API: {} {:?}", sc, res.text()?))
    }    
}