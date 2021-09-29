pub const API_BASE: &str = "https://beta.jamsocket.com/";
pub const WS_BASE: &str = "wss://beta.jamsocket.com/";

#[cfg(feature="client")]
use anyhow::{anyhow, Result};
#[cfg(feature="client")]
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

#[cfg(feature="client")]
pub struct JamsocketApi {
    token: String,
}

#[cfg(feature="client")]

impl JamsocketApi {
    pub fn new(token: &str) -> Self {
        JamsocketApi {
            token: token.to_string(),
        }
    }

    pub fn authenticate(&self) -> Result<bool> {
        let url = format!("{}authcheck", API_BASE);

        let client = reqwest::blocking::Client::new();
        let res = client.get(url).query(&[("token", &self.token)]).send()?;

        if res.status().is_success() {
            Ok(true)
        } else if res.status() == StatusCode::FORBIDDEN {
            Ok(false)
        } else {
            Err(anyhow!("Unexpected error code: {}", res.status()))
        }
    }

    pub fn upload(&self, service_id: &str, module: &[u8]) -> Result<UploadServiceResponse> {
        let url = format!("{}service/{}/module", API_BASE, service_id);

        let client = reqwest::blocking::Client::new();
        let res = client
            .post(url)
            .query(&[("token", &self.token)])
            .body(module.to_vec())
            .send()?;

        match res.status() {
            StatusCode::NOT_FOUND => Err(anyhow!("service_id not found.")),
            StatusCode::FORBIDDEN => Err(anyhow!(
                "Configured token is not authorized for given service_id."
            )),
            StatusCode::OK => res.json::<UploadServiceResponse>().map_err(|e| e.into()),
            sc => Err(anyhow!(
                "Received error status code from jamsocket API: {} {:?}",
                sc,
                res.text()?
            )),
        }
    }

    pub fn new_service(&self) -> Result<String> {
        let url = format!("{}service", API_BASE);

        let client = reqwest::blocking::Client::new();
        let res = client
            .post(url)
            .query(&[("token", &self.token)])
            .body("")
            .send()?;

        match res.status() {
            StatusCode::FORBIDDEN => Err(anyhow!(
                "Configured token is not valid."
            )),
            StatusCode::OK => {
                let response = res.json::<CreateServiceResponse>()?;
                Ok(response.service_id.to_string())
            },
            sc => Err(anyhow!(
                "Received error status code from jamsocket API: {} {:?}",
                sc,
                res.text()?
            )),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UploadServiceResponse {
    pub module: String,
    pub service: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateServiceResponse {
    pub service_id: String,
}
