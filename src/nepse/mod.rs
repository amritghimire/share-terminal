pub mod api;
mod dividends;
mod excel;
pub mod response;
mod updates;

extern crate clap;
use clap::ArgMatches;

pub fn handle_dividends_matches(matches: &ArgMatches) {
    let file = String::from(matches.value_of("file").unwrap_or("dividends.xlsx"));
    let response = dividends::export_dividends();
    let write_response = excel::write_to_excel(file, response);
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
