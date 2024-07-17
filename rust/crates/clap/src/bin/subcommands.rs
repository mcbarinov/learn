use clap::{Args, Parser, Subcommand};

/// Bla bla bla
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Bla bla bla
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Bla bla 1
    Cmd1 {
        /// Bla bla
        name: String,

        /// Bla bla
        #[arg(short, long)]
        value: Option<u16>,
    },

    /// Bla bla 2
    Cmd2(Cmd2Args),
}

#[derive(Debug, Args)]
struct Cmd2Args {
    /// Bla bla
    name: String,

    /// Bla bla
    #[arg(short, long)]
    value: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    dbg!(cli);
}

#[cfg(test)]
mod test {
    use clap::CommandFactory;

    use crate::Cli;

    #[test]
    fn test_cli() {
        Cli::command().debug_assert();
    }
}
