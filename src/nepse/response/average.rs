use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompanyItem {
    pub stock_symbol: String,
    pub company_name: String,
    pub company_group: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TradingAverageItem {
    pub stock_symbol: String,
    pub traded_amount: String,
    pub traded_shares: String,
    pub no_of_transaction: f64,
    pub closing_price: f64,
    pub as_of_date_short_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseInD<T> {
    pub d: Vec<T>,
}
