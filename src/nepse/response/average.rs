use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyItem {
    pub symbol: String,
    pub status: String,
    pub sector_name: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingAverageItem {
    pub symbol: String,
    pub closing_price_average: f64,
    pub total_traded_amount: f64,
    pub total_traded_shares: f64,
    pub total_trades: Option<f64>,
    pub weighted_average: f64,
    pub close_price: f64,
    pub closing_date: String,
}
