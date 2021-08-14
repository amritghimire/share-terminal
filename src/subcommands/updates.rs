use clap::{App, Arg, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("updates")
        .about("Get the latest updates..")
        .arg(
            Arg::with_name("type")
                .short("t")
                .required(false)
                .long("type")
                .takes_value(true)
                .long_help("Type of update to fetch (Comma separated)."),
        )
        .arg(
            Arg::with_name("all")
                .short("a")
                .required(false)
                .long("all")
                .takes_value(false)
                .long_help("Show all entries."),
        )
}
