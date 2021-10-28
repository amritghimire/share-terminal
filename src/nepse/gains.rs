use crate::nepse::api;
use crate::nepse::response::average::{CompanyItem, ResponseInD, TradingAverageItem};
use crate::nepse::response::gain::{GainItem, GainListResponse};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use regex::Regex;

const AVERAGE_PATH: &str = "/GraphModule/webservices/MarketWatchService.asmx/GetTodaySharePrices";
const COMPANY_LIST_PATH: &str =
    "/SimilarCompanies/webservices/CompanyService.asmx/GetListedCompaniesByCompanyCode";

pub fn export_gains<T: GainItem + DeserializeOwned>(
    path: &str,
) -> Result<GainListResponse<T>, reqwest::Error> {
    let mut average_params = HashMap::new();
    let mut average_entries = HashMap::new();
    let mut company_entries = HashMap::new();
    let mut company_params: HashMap<&str, &str> = HashMap::new();
    average_params.insert("fromdate", "");
    average_params.insert("toDate", "");
    average_params.insert("stockSymbol", "");
    average_params.insert("offset", "1");
    average_params.insert("limit", "1000");

    company_params.insert("offset", "1");
    company_params.insert("limit", "1000");
    company_params.insert("companyCode", "");
    company_params.insert("sectorArea", "---Select Sector---");
    let mut query_params = HashMap::new();
    query_params.insert("draw", "1");
    query_params.insert("length", "3000");
    query_params.insert("start", "0");
    let average_response: Result<ResponseInD<TradingAverageItem>, _> =
        api::fetch_nepalipaisa(AVERAGE_PATH, average_params);
    if let Ok(average_items) = average_response {
        for average_item in average_items.d.into_iter() {
            average_entries.insert(average_item.stock_symbol.clone(), average_item);
        }
    } else {
        println!(
            "Error occured while fetching trading average: {:?}",
            average_response.unwrap_err()
        );
    }

    let company_response: Result<ResponseInD<CompanyItem>, _> =
        api::fetch_nepalipaisa(COMPANY_LIST_PATH, company_params);
    if let Ok(company_items) = company_response {
        for company_item in company_items.d.into_iter() {
            company_entries.insert(company_item.stock_symbol.clone(), company_item);
        }
    } else {
        println!(
            "Error occured while fetching company entries: {:?}",
            company_response.unwrap_err()
        );
    }

    let mut response: Result<GainListResponse<T>, reqwest::Error> =
        api::fetch_nepse(path, query_params);
    if let Ok(ref mut list_response) = response {
        let re = Regex::new(r"<[^>]*>").unwrap();
        for gain in list_response.data.iter_mut() {
            gain.clean(re.clone());
            if let Some(entry) = company_entries.get(&gain.get_symbol()) {
                gain.add_company_info(entry);
            }
            if let Some(entry) = average_entries.get(&gain.get_symbol()) {
                gain.integrate(entry);
            }
        }
    }
    response
}
