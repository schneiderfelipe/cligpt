//! A command-line interface for ChatGPT.

use clap::Parser;

/// A command-line interface for ChatGPT.
#[derive(Parser)]
#[command(version, author, about)]
struct Cli {
    /// Your OpenAI API key.
    #[arg(short = 'k', long)]
    api_key: String,
}

fn main() {
    let cli = Cli::parse();

    println!("Hello, world!");
}
