#[macro_use]
extern crate clap;

use clap::App;
pub mod nepse;
pub mod subcommands;

fn main() {
    let app = App::new("Share Terminal")
        .version(crate_version!())
        .author("Amrit Ghimire <oss@amritghimire.com>")
        .about(
            "This is a command line application that can be used as interacting with nepse alpha.",
        )
        .subcommand(subcommands::dividends::subcommand())
        .subcommand(subcommands::autocompletion::subcommand())
        .subcommand(subcommands::updates::subcommand());

    subcommands::handle_matches(app);
}
