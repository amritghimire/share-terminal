use serde::de::DeserializeOwned;

use std::collections::HashMap;

const NEPSE_BASE: &str = "https://nepsealpha.com";

pub fn fetch<T: DeserializeOwned>(
    path: &str,
    params: HashMap<&str, &str>,
) -> Result<T, reqwest::Error> {
    let mut query_params = Vec::new();
    for (key, value) in params.iter() {
        query_params.push(format!("{}={}", key, value));
    }
    let query_string = query_params.join("&");
    let url = format!("{}{}?{}", NEPSE_BASE, path, query_string);
    let client = reqwest::blocking::Client::new();
    let response: T = client
        .get(url)
        .header("X-Requested-With", "XMLHttpRequest")
        .send()?
        .json()?;
    Ok(response)
}
