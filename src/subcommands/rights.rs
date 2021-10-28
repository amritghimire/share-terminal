use clap::{App, Arg, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("rights")
        .about("Generate CSV with rights..")
        .arg(
            Arg::with_name("file")
                .short("f")
                .required(false)
                .long("file")
                .takes_value(true)
                .long_help("Name of file to export the rights to."),
        )
}
