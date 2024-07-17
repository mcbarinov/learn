use clap::Parser;

/// Some text about the cli
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Some docs here
    #[arg(short, long)]
    name: String,

    /// Describe value parameter. Value is optional
    value: Option<u16>,

    /// Bla bla bla
    #[arg(short, long, default_value_t = false)]
    debug: bool,
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
