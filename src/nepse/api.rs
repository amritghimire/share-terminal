use serde::de::DeserializeOwned;

use std::collections::HashMap;

const NEPSE_BASE: &str = "https://nepsealpha.com";
const NEPALSTOCK_BASE: &str = "https://www.nepalstock.com.np";

pub fn fetch<T: DeserializeOwned>(
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
    let response = client
        .get(url)
        .header("X-Requested-With", "XMLHttpRequest")
        .send()?
        .json()?;
    Ok(response)
}

pub fn fetch_nepse<T: DeserializeOwned>(
    path: &str,
    params: HashMap<&str, &str>,
) -> Result<T, reqwest::Error> {
    fetch(NEPSE_BASE, path, params)
}
pub fn fetch_nepalstock<T: DeserializeOwned>(
    path: &str,
    params: HashMap<&str, &str>,
) -> Result<T, reqwest::Error> {
    fetch(NEPALSTOCK_BASE, path, params)
}
