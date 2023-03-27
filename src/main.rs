//! `cligpt` is a command-line interface for interacting with the `ChatGPT` API from `OpenAI`.
//! With `cligpt`,
//! you can quickly and easily generate text by sending prompts to `ChatGPT` from your terminal.
//! Simply provide your `OpenAI` API key and the message you want to generate,
//! and `cligpt` will handle the rest.
//!
//! `cligpt` is written in Rust and uses the clap and `cligpt` crates to provide a user-friendly interface for the `ChatGPT` API.
//! It's perfect for developers who want to quickly prototype text generation applications or for anyone who wants to experiment with the power of the `ChatGPT` API from the comfort of their terminal.
//!
//! # Installation
//!
//! ## Requirements
//!
//! Before installing `cligpt`,
//! you need to make sure you have [Rust](https://www.rust-lang.org/tools/install) (version 1.64.0 or later) installed.
//!
//! ## Instructions
//!
//! Once you have Rust installed,
//! you can clone this repository and build it using [Cargo](https://doc.rust-lang.org/cargo/):
//!
//! ```bash
//! git clone https://github.com/schneiderfelipe/cligpt.git
//! cd cligpt
//! cargo install --path=.
//! ```
//!
//! This will download the necessary dependencies,
//! compile the `cligpt` binary,
//! and install it in your system.
//! Once the installation is complete,
//! you can run `cligpt` by typing `cligpt` in your terminal.
//!
//! # Usage
//!
//! To use `cligpt`,
//! you need to provide your [`OpenAI` API key](https://platform.openai.com/account/api-keys) and the message you want to generate.
//! You can provide the API key using the `-k` or `--api-key` option,
//! or by setting the `OPENAI_API_KEY` environment variable.
//!
//! Here's an example usage:
//!
//! ```bash
//! cligpt --api-key YOUR_API_KEY 'Hello, ChatGPT!'
//! ```
//!
//! This will send the message "Hello, `ChatGPT`!" to the `ChatGPT` API using your API key and print the generated text to your terminal.

use std::error::Error;

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};
use clap::Parser;
use futures_util::StreamExt;

/// A command-line interface for `ChatGPT`.
#[derive(Debug, Parser)]
#[command(version, author, about)]
struct Cli {
    /// Your OpenAI API key.
    #[arg(short = 'k', long, env = "OPENAI_API_KEY")]
    api_key: String,

    /// The message to send to ChatGPT.
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let message = cli.message;
    let api_key = cli.api_key;

    let client = Client::new().with_api_key(api_key);
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .max_tokens(1024u16)
        .temperature(0.7)
        .messages([ChatCompletionRequestMessageArgs::default()
            .content(message)
            .build()?])
        .build()?;
    let mut stream = client.chat().create_stream(request).await?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                if let Some(choice) = response.choices.get(0) {
                    if let Some(text) = &choice.delta.content {
                        print!("{text}");
                    }
                }
            }
            Err(error) => eprintln!("error: {error}"),
        }
    }
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
