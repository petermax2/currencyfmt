use std::path::PathBuf;

use clap::Parser;

/// A configurable command line tool for currency and commodity amount formatting and pretty-printing.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CurrencyFmtCmd {
    /// optional path to the configuration file to be used.
    /// You can overwrite the default configuration path using this option.
    #[arg(short, long)]
    configuration: Option<PathBuf>,

    /// optional delimiter string that separates amount-commodity pairs inside the application input
    #[arg(short, long)]
    delimiter: Option<String>,

    /// amount-commodity pairs to be pretty printed
    numbers: Vec<String>,
}

fn main() {
    let _cli = CurrencyFmtCmd::parse();

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_empty() {
        // this is a placeholder - will be removed as soon as useful test cases are available
        assert_eq!(1, 1);
    }
}
