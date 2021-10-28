pub mod api;

use crate::nepse::response::gain;
mod excel;
mod gains;
pub mod response;
mod updates;

extern crate clap;
use clap::ArgMatches;

const DIVIDENDS_PATH: &str = "/investment-calandar/dividend";
const RIGHTS_PATH: &str = "/investment-calandar/rights";

pub fn handle_dividends_matches(matches: &ArgMatches) {
    let file = String::from(matches.value_of("file").unwrap_or("dividends.xlsx"));
    let response = gains::export_gains::<gain::DividendListItem>(DIVIDENDS_PATH);
    let write_response = excel::dividend::write_to_excel(file, response);
    if write_response.is_err() {
        eprintln!("Error writing to excel file.");
        std::process::exit(1);
    }
}

pub fn handle_rights_matches(matches: &ArgMatches) {
    let file = String::from(matches.value_of("file").unwrap_or("rights.xlsx"));
    let response = gains::export_gains::<gain::RightListItem>(RIGHTS_PATH);
    let write_response = excel::rights::write_to_excel(file, response);
    if write_response.is_err() {
        eprintln!("Error writing to excel file.");
        std::process::exit(1);
    }
}

pub fn handle_updates_matches(matches: &ArgMatches) {
    let types = String::from(
        matches
            .value_of("type")
            .unwrap_or("dividend,ipo,right,auction,agm,fpo,mf"),
    );
    let show_all = matches.is_present("all");
    updates::check_updates(types, show_all);
}
