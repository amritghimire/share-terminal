use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DividendListItem {
    symbol: String,
    company_name: Option<String>,
    bonus_dividend: String,
    cash_dividend: String,
    total: String,
    book_closure_date: Option<String>,
    fiscal_year: String,
    avg_3yr_dividend: String,
    dividend_growth_rate: String,
    status: String,
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
}

#[derive(Debug, Deserialize)]
pub struct DividendListResponse {
    pub data: Vec<DividendListItem>,
}
