use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::nepse::response::average::{CompanyItem, TradingAverageItem};

pub trait GainItem {
    fn clean(&mut self, re: Regex);
    fn add_company_info(&mut self, company: &CompanyItem);
    fn integrate(&mut self, average: &TradingAverageItem);
    fn get_symbol(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DividendListItem {
    pub symbol: String,
    pub company_name: Option<String>,
    pub bonus_dividend: String,
    pub cash_dividend: String,
    pub total: String,
    pub book_closure_date: Option<String>,
    pub fiscal_year: String,
    pub avg_3yr_dividend: String,
    pub dividend_growth_rate: String,
    pub status: String,
    pub closing_price_average: Option<f64>,
    pub total_traded_amount: Option<String>,
    pub total_traded_shares: Option<String>,
    pub total_trades: Option<f64>,
    pub weighted_average: Option<f64>,
    pub close_price: Option<f64>,
    pub closing_date: Option<String>,
    pub company_status: Option<String>,
    pub sector_name: Option<String>,
}

impl GainItem for DividendListItem {
    fn clean(&mut self, re: Regex) {
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

    fn add_company_info(&mut self, company: &CompanyItem) {
        self.sector_name = Some(company.company_group.clone());
    }
    fn integrate(&mut self, average: &TradingAverageItem) {
        self.total_traded_amount = Some(average.traded_amount.clone());
        self.total_traded_shares = Some(average.traded_shares.clone());
        self.total_trades = Some(average.no_of_transaction);
        self.close_price = Some(average.closing_price);
        self.closing_date = Some(average.as_of_date_short_string.clone());
    }
    fn get_symbol(&self) -> String {
        self.symbol.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RightListItem {
    pub symbol: String,
    pub company_name: Option<String>,
    pub book_closure_date: Option<String>,
    pub opening_date: String,
    pub closing_date: String,
    pub units: String,
    pub ratio: String,
    pub ratio_frac: Option<f64>,
    pub total_traded_amount: Option<String>,
    pub total_traded_shares: Option<String>,
    pub total_trades: Option<f64>,
    pub close_price: Option<f64>,
    pub closing_date_price: Option<String>,
    pub sector_name: Option<String>,
}

impl GainItem for RightListItem {
    fn clean(&mut self, re: Regex) {
        self.symbol = re.replace_all(self.symbol.as_str(), "").to_string();
        let ratio = &self.ratio;
        if !ratio.is_empty() {
            let v: Vec<&str> = ratio.split(':').collect();
            if v.len() > 1 {
                let num: f64 = v[0].parse().unwrap();
                let den: f64 = v[1].parse().unwrap();
                self.ratio_frac = Some(num / den);
            }
        }
    }

    fn add_company_info(&mut self, company: &CompanyItem) {
        self.sector_name = Some(company.company_group.clone());
    }
    fn integrate(&mut self, average: &TradingAverageItem) {
        self.total_traded_amount = Some(average.traded_amount.clone());
        self.total_traded_shares = Some(average.traded_shares.clone());
        self.total_trades = Some(average.no_of_transaction);
        self.close_price = Some(average.closing_price);
        self.closing_date_price = Some(average.as_of_date_short_string.clone());
    }
    fn get_symbol(&self) -> String {
        self.symbol.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct GainListResponse<T> {
    pub data: Vec<T>,
}
