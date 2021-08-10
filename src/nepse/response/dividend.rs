use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::nepse::response::average::{CompanyItem, TradingAverageItem};

#[derive(Debug, Serialize, Deserialize)]
pub struct DividendListItem {
    pub symbol: String,
    company_name: Option<String>,
    bonus_dividend: String,
    cash_dividend: String,
    total: String,
    book_closure_date: Option<String>,
    fiscal_year: String,
    avg_3yr_dividend: String,
    dividend_growth_rate: String,
    status: String,
    closing_price_average: Option<f64>,
    total_traded_amount: Option<f64>,
    total_traded_shares: Option<f64>,
    total_trades: Option<f64>,
    weighted_average: Option<f64>,
    close_price: Option<f64>,
    closing_date: Option<String>,
    company_status: Option<String>,
    sector_name: Option<String>,
}

impl DividendListItem {
    pub fn clean(&mut self, re: Regex) {
        if self.avg_3yr_dividend == "--" {
            self.avg_3yr_dividend = "0".to_string();
        }
        if self.dividend_growth_rate == "--" {
            self.dividend_growth_rate = "0".to_string();
        }
        self.symbol = re.replace_all(self.symbol.as_str(), "").to_string();
        self.status = re.replace_all(self.status.as_str(), "").to_string();
        self.bonus_dividend = self.bonus_dividend.replace("%", "");
        self.cash_dividend = self.cash_dividend.replace("%", "");
        self.total = self.total.replace("%", "");
    }

    pub fn add_company_info(&mut self, company: &CompanyItem) {
        self.company_status = Some(company.status.clone());
        self.sector_name = Some(company.sector_name.clone());
    }
    pub fn integrate(&mut self, average: &TradingAverageItem) {
        self.closing_price_average = Some(average.closing_price_average);
        self.total_traded_amount = Some(average.total_traded_amount);
        self.total_traded_shares = Some(average.total_traded_shares);
        self.total_trades = average.total_trades;
        self.weighted_average = Some(average.weighted_average);
        self.close_price = Some(average.close_price);
        self.closing_date = Some(average.closing_date.clone());
    }
}

#[derive(Debug, Deserialize)]
pub struct DividendListResponse {
    pub data: Vec<DividendListItem>,
}
