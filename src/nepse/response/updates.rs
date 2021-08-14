use chrono::{NaiveDate, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItem {
    pub symbol: String,
    pub company_name: Option<String>,
    pub bonus_dividend: Option<String>,
    pub cash_dividend: Option<String>,
    pub total: Option<String>,
    pub book_closure_date: Option<String>,
    pub fiscal_year: Option<String>,
    pub status: Option<String>,

    pub ratio: Option<String>,
    pub units: Option<String>,
    pub opening_date: Option<String>,
    pub closing_date: Option<String>,

    pub price: Option<String>,
    pub last_closing_date: Option<String>,

    pub open_date: Option<String>,
    pub close_date: Option<String>,

    pub agm_date: Option<String>,
    pub title: Option<String>,
    pub agm_sgm: Option<String>,
    pub book_close_date: Option<String>,
    pub body: Option<String>,
}

impl UpdateItem {
    pub fn clean(&mut self, re: Regex) {
        self.symbol = re.replace_all(self.symbol.as_str(), "").to_string();
        if let Some(status) = &self.status {
            self.status = Some(re.replace_all(status.as_str(), "").to_string());
        }
    }

    pub fn should_display(&mut self, section: &str) -> bool {
        if section == "IPO" && self.symbol.contains("Local") {
            return false;
        }
        let date_field = match section {
            "AGM" => self.agm_date.as_ref(),
            "AUCTION" => self.close_date.as_ref(),
            "DIVIDEND" => self.book_closure_date.as_ref(),
            _ => self.closing_date.as_ref(),
        };
        if let Some(closing_date) = date_field {
            let date_parsed = NaiveDate::parse_from_str(closing_date.as_str(), "%Y-%m-%d");
            if let Ok(date) = date_parsed {
                let now = Utc::now();
                date > now.naive_local().date()
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn display(&mut self, section: &str) {
        match section {
            "IPO" => {
                println!(
                    "{} units for {} {} ({} - {}) ",
                    self.units.as_ref().unwrap_or(&"".to_string()),
                    self.symbol,
                    self.status.as_ref().unwrap_or(&"".to_string()),
                    self.opening_date.as_ref().unwrap_or(&"".to_string()),
                    self.closing_date.as_ref().unwrap_or(&"".to_string())
                );
            }
            "DIVIDEND" => {
                println!(
                    "{} bonus + {} cash of {} for {} {} ({})",
                    self.bonus_dividend.as_ref().unwrap_or(&"".to_string()),
                    self.cash_dividend.as_ref().unwrap_or(&"".to_string()),
                    self.fiscal_year.as_ref().unwrap_or(&"".to_string()),
                    self.symbol,
                    self.status.as_ref().unwrap_or(&"".to_string()),
                    self.book_closure_date.as_ref().unwrap_or(&"".to_string())
                )
            }
            "RIGHTS" => {
                println!(
                    "{} units ({}) for {} ({} - {}).\n Book closed on {} ",
                    self.units.as_ref().unwrap_or(&"".to_string()),
                    self.ratio.as_ref().unwrap_or(&"".to_string()),
                    self.symbol,
                    self.opening_date.as_ref().unwrap_or(&"".to_string()),
                    self.closing_date.as_ref().unwrap_or(&"".to_string()),
                    self.book_closure_date.as_ref().unwrap_or(&"".to_string())
                );
            }
            "MUTUAL" => {
                println!(
                    "{} units at {} for {} {} ({} - {}) ",
                    self.units.as_ref().unwrap_or(&"".to_string()),
                    self.price.as_ref().unwrap_or(&"".to_string()),
                    self.symbol,
                    self.status.as_ref().unwrap_or(&"".to_string()),
                    self.opening_date.as_ref().unwrap_or(&"".to_string()),
                    self.closing_date.as_ref().unwrap_or(&"".to_string())
                );
            }
            "FPO" => {
                println!(
                    "{} units at {} for {} {} ({} - {}) ",
                    self.units.as_ref().unwrap_or(&"".to_string()),
                    self.price.as_ref().unwrap_or(&"".to_string()),
                    self.symbol,
                    self.status.as_ref().unwrap_or(&"".to_string()),
                    self.opening_date.as_ref().unwrap_or(&"".to_string()),
                    self.closing_date.as_ref().unwrap_or(&"".to_string())
                );
            }
            "AUCTION" => {
                println!(
                    "{} units for {} {} ({} - {}) ",
                    self.units.as_ref().unwrap_or(&"".to_string()),
                    self.symbol,
                    self.status.as_ref().unwrap_or(&"".to_string()),
                    self.open_date.as_ref().unwrap_or(&"".to_string()),
                    self.close_date.as_ref().unwrap_or(&"".to_string())
                );
            }
            "AGM" => {
                println!(
                    "{} for {} at {}",
                    self.fiscal_year.as_ref().unwrap_or(&"".to_string()),
                    self.symbol,
                    self.agm_date.as_ref().unwrap_or(&"".to_string())
                );
            }
            _ => {}
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateList {
    pub dividents: Vec<UpdateItem>,
    pub rights: Vec<UpdateItem>,
    pub ipo: Vec<UpdateItem>,
    pub fpo: Vec<UpdateItem>,
    pub auction: Vec<UpdateItem>,
    pub mutual_fund: Vec<UpdateItem>,
    pub bonds_deb: Vec<UpdateItem>,
    pub agms: Vec<UpdateItem>,
}

#[derive(Debug, Deserialize)]
pub struct LatestDividendList {
    pub data: Vec<UpdateItem>,
}
