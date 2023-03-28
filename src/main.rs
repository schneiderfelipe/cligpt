//! `cligpt` is a command-line interface for interacting with the `ChatGPT` API from `OpenAI`.
//! With `cligpt`,
//! you can quickly and easily generate text by sending prompts to `ChatGPT` from your terminal.
//! Simply provide your `OpenAI` API key and the message you want to generate,
//! and `cligpt` will handle the rest.
//!
//! `cligpt` is written in [Rust](https://www.rust-lang.org/) and uses the clap and `cligpt` crates to provide a user-friendly interface for the `ChatGPT` API.
//! It's perfect for developers who want to quickly prototype text generation applications or for anyone who wants to experiment with the power of the `ChatGPT` API from the comfort of their terminal.
//!
//! # Installation
//!
//! ## Requirements
//!
//! Before installing `cligpt`,
//! you need to make sure you have [Rust](https://www.rust-lang.org/tools/install) (version 1.65.0 or later)
//! and [Cargo](https://doc.rust-lang.org/cargo/),
//! the package manager for Rust,
//! installed.
//!
//! ## From [crates.io](https://crates.io/crates/cligpt) (recommended)
//!
//! Once you have Rust and Cargo installed,
//! you can install `cligpt` from [crates.io](https://crates.io/) using Cargo:
//!
//! ```bash
//! cargo install cligpt
//! ```
//!
//! This will download the necessary dependencies,
//! compile the `cligpt` binary,
//! and install it in your system.
//! Once the installation is complete,
//! you can run `cligpt` by typing `cligpt` in your terminal.
//!
//! ## Directly from [GitHub](https://github.com/schneiderfelipe/cligpt)
//!
//! Alternatively,
//! you can install `cligpt` directly from the GitHub repository using Cargo by running:
//!
//! ```bash
//! cargo install --git=https://github.com/schneiderfelipe/cligpt.git
//! ```
//!
//! ## By cloning the GitHub repository
//!
//! You can also build `cligpt` from source by cloning the GitHub repository and running `cargo build`:
//!
//! ```bash
//! git clone https://github.com/schneiderfelipe/cligpt.git
//! cd cligpt
//! cargo build --release
//! ```
//!
//! After building,
//! the binary will be located at `target/release/cligpt`.
//!
//! Alternatively,
//! you can install the binary directly instead of just building it.
//! Just run `cargo install --path=.` instead of `cargo build --release`.
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
//! This will send the message `'Hello, ChatGPT!'` to the `ChatGPT` API using your API key and print the generated text to your terminal.

use std::{
    io::{self, Read},
    ops::RangeInclusive,
};

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};
use clap::Parser;
use color_eyre::eyre::{self, Context};
use futures_util::StreamExt;

/// A command-line interface to talk to `ChatGPT`.
#[derive(Debug, Parser)]
#[command(version, author, about)]
struct Cli {
    /// Text to prepend to the message as context.
    context: Vec<String>,

    /// Model to use for the chat.
    #[arg(long, default_value = "gpt-3.5-turbo", value_parser = model_parser)]
    model: String,

    /// Temperature to use for the chat.
    #[arg(long, default_value_t = 0.7, value_parser = temperature_parser)]
    temperature: f32,

    /// Your OpenAI API key.
    #[arg(short = 'k', long, env = "OPENAI_API_KEY", value_parser = api_key_parser)]
    api_key: String,
}

fn model_parser(model: &str) -> Result<String, String> {
    match model {
        "gpt-3.5-turbo" | "gpt-4" => Ok(model.into()),
        _ => Err(format!("'{model}' is not a valid model name")),
    }
}

const TEMPERATURE_RANGE: RangeInclusive<f32> = 0.0..=1.0;

fn temperature_parser(temperature: &str) -> Result<f32, String> {
    let temperature: f32 = temperature.parse().map_err(|err| format!("{err}"))?;
    if temperature < *TEMPERATURE_RANGE.start() {
        Err(format!(
            "too low (minimum value is {:.1})",
            *TEMPERATURE_RANGE.start()
        ))
    } else if temperature > *TEMPERATURE_RANGE.end() {
        Err(format!(
            "too high (maximum value is {:.1})",
            *TEMPERATURE_RANGE.end()
        ))
    } else {
        Ok(temperature)
    }
}

const API_KEY_RANGE: RangeInclusive<usize> = 40..=50;

// Logic from <https://docs.gitguardian.com/secrets-detection/detectors/specifics/openai_apikey>.
fn api_key_parser(api_key: &str) -> Result<String, String> {
    if !api_key.starts_with("sk-") {
        return Err(format!("'{api_key}' does not start with 'sk-'"));
    }

    let suffix = &api_key[3..];
    if let Some(offending_char) = suffix.chars().find(|c| !c.is_ascii_alphanumeric()) {
        return Err(format!(
            "'{api_key}' contains invalid character '{offending_char}'"
        ));
    }

    let key_len = suffix.len();
    if key_len < *API_KEY_RANGE.start() {
        return Err(format!(
            "'{api_key}' is too short (expected at least {} characters)",
            API_KEY_RANGE.start()
        ));
    } else if key_len > *API_KEY_RANGE.end() {
        return Err(format!(
            "'{api_key}' is too long (expected at most {} characters)",
            API_KEY_RANGE.end()
        ));
    }

    Ok(api_key.into())
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install().context("failed to install error report handler")?;

    let cli = Cli::parse();

    let mut message = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut message)
        .context("failed to read from the standard input")?;
    let context = cli.context.join(" ");
    let message = format!("{context} {message}");

    let api_key = cli.api_key;
    let model = cli.model;
    let temperature = cli.temperature;

    let client = Client::new().with_api_key(api_key);
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .temperature(temperature)
        .messages([ChatCompletionRequestMessageArgs::default()
            .content(message)
            .build()
            .context("failed to build a message")?])
        .build()
        .context("failed to build the completion request")?;
    let mut stream = client
        .chat()
        .create_stream(request)
        .await
        .context("failed to create the completion stream")?;

    while let Some(result) = stream.next().await {
        let response = result.context("failed to obtain a stream response")?;
        if let Some(choice) = response.choices.get(0) {
            if let Some(text) = &choice.delta.content {
                print!("{text}");
            }
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
