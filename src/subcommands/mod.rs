pub mod autocompletion;
pub mod dividends;

use clap::{App, Shell};
use std::io;
use std::str::FromStr;

use crate::nepse;

pub fn handle_matches(mut app: App) {
    let matches = app.clone().get_matches();

    if let Some(dividends_matches) = matches.subcommand_matches("dividends") {
        nepse::handle_dividends_matches(dividends_matches);
    } else if let Some(autocompletion) = matches.subcommand_matches("autocompletion") {
        let shell_name = autocompletion.value_of("shell").unwrap();
        let shell_parse = Shell::from_str(shell_name);
        if shell_parse.is_err() {
            eprintln!(
                "Invalid shell name passed. Only bash, fish, zsh, powershell, elvish are allowed."
            )
        }
        let shell = shell_parse.unwrap();
        app.gen_completions_to("jira-terminal", shell, &mut io::stdout());
    } else {
        let result = app.print_long_help();
        if result.is_err() {
            println!("Use share-terminal help to view the available commands.");
        }
        println!();
    }
}
