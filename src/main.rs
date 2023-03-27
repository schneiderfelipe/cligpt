//! A command-line interface for ChatGPT.

use clap::Parser;

/// A command-line interface for ChatGPT.
#[derive(Debug, Parser)]
#[command(version, author, about)]
struct Cli {
    /// Your OpenAI API key.
    #[arg(short = 'k', long, env = "OPENAI_API_KEY")]
    api_key: String,

    /// The message to send to ChatGPT.
    message: String,
}

fn main() {
    let cli = Cli::parse();

    println!("{cli:#?}");
}
