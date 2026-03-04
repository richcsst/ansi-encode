use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
struct Cli {
    /// Print version information
    #[clap(short, long)]
    version: bool,

    /// Print help information
    #[clap(short, long)]
    help: bool,

    /// Token options
    #[clap(long)]
    tokens: Option<String>,

    /// Raw token options
    #[clap(long)]
    rawtokens: Option<String>,

    /// Symbol options
    #[clap(long)]
    symbols: Option<String>,

    /// Dump options
    #[clap(long)]
    dump: bool,

    /// Unicode options
    #[clap(long)]
    unicode: bool,

    /// Color options
    #[clap(long)]
    colors: bool,

    /// Frame options
    #[clap(long)]
    frames: bool,

    /// Horizontal rule options
    #[clap(long)]
    horizontal_rules: bool,

    /// ANSI modes
    #[clap(long)]
    ansi_modes: Option<String>,

    /// Baud rate
    #[clap(long, value_parser)]
    baud: Option<u32>,

    /// Terminal width
    #[clap(long, value_parser)]
    width: Option<u32>,

    /// Input file
    #[clap(value_parser)]
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    // TODO: Implement the CLI functionality here
}