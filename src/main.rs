use std::path::PathBuf;

use clap::Parser;
use config::FormatterConfig;

mod config;
mod error;

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
    let command = CurrencyFmtCmd::parse();
    let _config = FormatterConfig::load(command.configuration).unwrap_or_else(|err| {
        eprintln!("[ERROR] {}", err);
        std::process::exit(1);
    });

    println!("Hello, world!");
}
