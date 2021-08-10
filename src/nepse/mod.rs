pub mod api;
mod dividends;
mod excel;
pub mod response;

extern crate clap;
use clap::ArgMatches;

pub fn handle_dividends_matches(matches: &ArgMatches) {
    let file = String::from(matches.value_of("file").unwrap_or("dividends.csv"));
    dividends::export_dividends(file);
}
