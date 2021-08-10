use crate::nepse::api;
use crate::nepse::response::average::{CompanyItem, TradingAverageItem};
use crate::nepse::response::dividend::DividendListResponse;
use regex::Regex;
use std::collections::HashMap;

const DIVIDENDS_PATH: &str = "/investment-calandar/dividend";
const AVERAGE_PATH: &str = "/api/nots/nepse-data/trading-average";
const COMPANY_LIST_PATH: &str = "/api/nots/company/list";

pub fn export_dividends(file_path: String) {
    let mut average_params = HashMap::new();
    let mut average_entries = HashMap::new();
    let mut company_entries = HashMap::new();
    let company_params: HashMap<&str, &str> = HashMap::new();
    average_params.insert("nDays", "5");
    let mut query_params = HashMap::new();
    query_params.insert("draw", "1");
    query_params.insert("length", "3000");
    query_params.insert("start", "0");
    let average_response: Result<Vec<TradingAverageItem>, _> =
        api::fetch_nepalstock(AVERAGE_PATH, average_params);
    if let Ok(average_items) = average_response {
        for average_item in average_items.into_iter() {
            average_entries.insert(average_item.symbol.clone(), average_item);
        }
    } else {
        println!("Error occured: {:?}", average_response.unwrap_err());
    }

    let company_response: Result<Vec<CompanyItem>, _> =
        api::fetch_nepalstock(COMPANY_LIST_PATH, company_params);
    if let Ok(company_items) = company_response {
        for company_item in company_items.into_iter() {
            company_entries.insert(company_item.symbol.clone(), company_item);
        }
    } else {
        println!("Error occured: {:?}", company_response.unwrap_err());
    }

    let response: Result<DividendListResponse, _> = api::fetch_nepse(DIVIDENDS_PATH, query_params);
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
            if let Some(entry) = company_entries.get(&dividend.symbol) {
                dividend.add_company_info(entry);
            }
            if let Some(entry) = average_entries.get(&dividend.symbol) {
                dividend.integrate(entry);
            }
            wrt.serialize(dividend);
        }
        wrt.flush();
    } else {
        println!("Error occured: {:?}", response.unwrap_err());
    }
}
