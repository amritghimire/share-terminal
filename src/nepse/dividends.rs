use crate::nepse::{api, response};
use regex::Regex;
use std::collections::HashMap;

const DIVIDENDS_PATH: &str = "/investment-calandar/dividend";

pub fn export_dividends(file_path: String) {
    let mut query_params = HashMap::new();
    query_params.insert("draw", "1");
    query_params.insert("length", "3000");
    query_params.insert("start", "0");
    let response: Result<response::DividendListResponse, _> =
        api::fetch(DIVIDENDS_PATH, query_params);
    if let Ok(mut list_response) = response {
        let writer_attempt = csv::Writer::from_path(file_path);
        if writer_attempt.is_err() {
            eprintln!("Could not open writer here. ");
            return;
        }
        let mut wrt = writer_attempt.unwrap();
        let re = Regex::new(r"<[^>]*>").unwrap();
        for dividend in list_response.data.iter_mut() {
            dividend.clean(re.clone());
            wrt.serialize(dividend);
        }
        wrt.flush();
    } else {
        println!("Error occured: {:?}", response.unwrap_err());
    }
}
