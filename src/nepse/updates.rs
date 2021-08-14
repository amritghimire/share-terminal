use crate::nepse::api;
use crate::nepse::response::updates::{LatestDividendList, UpdateList};
use regex::Regex;

use std::collections::HashMap;

const DIVIDENDS_PATH: &str = "/api/smx9841/investment_calander";
const DIVIDEND_PATH: &str = "/investment-calandar/dividend";

pub fn check_updates(types: String, show_all: bool) {
    let calendar_params: HashMap<&str, &str> = HashMap::new();
    let sections: Vec<&str> = types.split(',').collect();
    let calendar_response: Result<UpdateList, _> =
        api::fetch_nepse(DIVIDENDS_PATH, calendar_params);
    if let Ok(mut calendar_items) = calendar_response {
        let re = Regex::new(r"<[^>]*>").unwrap();
        if sections.contains(&"ipo") {
            println!("IPO");
            println!("---");
            for ipo_item in calendar_items.ipo.iter_mut() {
                if show_all || ipo_item.should_display("IPO") {
                    ipo_item.clean(re.clone());
                    ipo_item.display("IPO");
                }
            }
        }

        if sections.contains(&"dividend") {
            let mut dividend_params = HashMap::new();
            dividend_params.insert("columns[0][data]", "book_closure_date");
            dividend_params.insert("order[0][column]", "0");
            dividend_params.insert("order[0][dir]", "desc");
            dividend_params.insert("start", "0");
            dividend_params.insert("length", "10");
            let dividend_response: Result<LatestDividendList, _> =
                api::fetch_nepse(DIVIDEND_PATH, dividend_params);
            if let Ok(mut dividend_data) = dividend_response {
                println!("\nDividends\n---------");
                for dividend_item in dividend_data.data.iter_mut() {
                    if show_all || dividend_item.should_display("DIVIDEND") {
                        dividend_item.clean(re.clone());
                        dividend_item.display("DIVIDEND");
                    }
                }
            }
        }

        if sections.contains(&"right") {
            println!("\nRight\n---------");
            for calendar_item in calendar_items.rights.iter_mut() {
                if show_all || calendar_item.should_display("RIGHTS") {
                    calendar_item.clean(re.clone());
                    calendar_item.display("RIGHTS");
                }
            }
        }

        if sections.contains(&"agm") {
            println!("\nAGMs\n---------");
            for calendar_item in calendar_items.agms.iter_mut() {
                if show_all || calendar_item.should_display("AGM") {
                    calendar_item.clean(re.clone());
                    calendar_item.display("AGM");
                }
            }
        }

        if sections.contains(&"fpo") {
            println!("\nFPO\n---------");
            for calendar_item in calendar_items.fpo.iter_mut() {
                if show_all || calendar_item.should_display("FPO") {
                    calendar_item.clean(re.clone());
                    calendar_item.display("FPO");
                }
            }
        }

        if sections.contains(&"auction") {
            println!("\nAuction\n---------");
            for calendar_item in calendar_items.auction.iter_mut() {
                if show_all || calendar_item.should_display("AUCTION") {
                    calendar_item.clean(re.clone());
                    calendar_item.display("AUCTION");
                }
            }
        }
        if sections.contains(&"mf") {
            println!("\nMutual Fund\n---------");
            for calendar_item in calendar_items.mutual_fund.iter_mut() {
                if show_all || calendar_item.should_display("MUTUAL") {
                    calendar_item.clean(re.clone());
                    calendar_item.display("MUTUAL");
                }
            }
        }
    } else {
        eprintln!("Error occured with api call.");
        std::process::exit(1);
    }
}
