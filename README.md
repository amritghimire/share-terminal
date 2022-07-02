# share-terminal

A rust application to use some of share related information in Nepal


## Installation Guide
To install this application, clone this repo to your local directory.
```
git clone https://github.com/amritghimire/share-terminal.git
cd share-terminal
```

If you dont have a Rust environment set up rust and its toolchain from [Rust official website](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Once you have rust setup, you can use the cargo install command:
```cargo install --path .```

Cargo will build the share-terminal binary and place it in $HOME/.cargo/bin)


## Usage
```
share-terminal help
Share Terminal 0.1.0
Amrit Ghimire <oss@amritghimire.com>
This is a command line application that can be used as interacting with nepse alpha.

USAGE:
    share-terminal [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    autocompletion    Generate autocompletion script..
    dividends         Generate CSV with Dividends..
    help              Prints this message or the help of the given subcommand(s)
    rights            Generate CSV with rights..
    updates           Get the latest updates..
```

```
share-terminal help autocompletion
share-terminal-autocompletion
Generate autocompletion script..

USAGE:
    share-terminal autocompletion --shell <shell>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --shell <shell>    Name of shell to create the autocompletion. Possible values are:   bash, fish, zsh,
                           powershell, elvish
```

```
share-terminal help dividends
share-terminal-dividends
Generate CSV with Dividends..

USAGE:
    share-terminal dividends [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>    Name of file to export the dividends to.
```

```
share-terminal help rights
share-terminal-rights
Generate CSV with rights..

USAGE:
    share-terminal rights [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>    Name of file to export the rights to.
```

```
share-terminal help updates
share-terminal-updates
Get the latest updates..

USAGE:
    share-terminal updates [FLAGS] [OPTIONS]

FLAGS:
    -a, --all        Show all entries.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --type <type>    Type of update to fetch (Comma separated).
```
