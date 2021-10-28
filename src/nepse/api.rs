use serde::de::DeserializeOwned;

use reqwest::header;
use std::collections::HashMap;

const NEPSE_BASE: &str = "https://nepsealpha.com";
const NEPALIPAISA_BASE: &str = "https://nepalipaisa.com/Modules";

pub fn fetch_internal<T: DeserializeOwned>(
    base: &str,
    path: &str,
    params: HashMap<&str, &str>,
) -> Result<T, reqwest::Error> {
    let mut query_params = Vec::new();
    for (key, value) in params.iter() {
        query_params.push(format!("{}={}", key, value));
    }
    let query_string = query_params.join("&");
    let url = format!("{}{}?{}", base, path, query_string);
    let client = reqwest::blocking::Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "X-Requested-With",
        header::HeaderValue::from_static("XMLHttpRequest"),
    );
    let response = client.get(url.clone()).headers(headers.clone()).send()?;
    Ok(response.json()?)
}

pub fn fetch<T: DeserializeOwned>(
    base: &str,
    path: &str,
    params: HashMap<&str, &str>,
) -> Result<T, reqwest::Error> {
    fetch_internal(base, path, params)
}

pub fn fetch_nepse<T: DeserializeOwned>(
    path: &str,
    params: HashMap<&str, &str>,
) -> Result<T, reqwest::Error> {
    fetch(NEPSE_BASE, path, params)
}

pub fn fetch_nepalipaisa<T: DeserializeOwned>(
    path: &str,
    body: HashMap<&str, &str>,
) -> Result<T, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}{}", NEPALIPAISA_BASE, path);
    let response = client.post(url.clone()).json(&body).send()?;
    Ok(response.json()?)
}
