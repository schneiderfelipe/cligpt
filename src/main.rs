//! [![dependency status](https://deps.rs/repo/github/schneiderfelipe/cligpt/status.svg)](https://deps.rs/repo/github/schneiderfelipe/cligpt)
//!
//! `cligpt` is a command-line interface for interacting
//! with the `ChatGPT` API from `OpenAI`.
//! With `cligpt`,
//! you can quickly and easily generate text by sending prompts to `ChatGPT`
//! from your terminal.
//! Simply provide your `OpenAI` API key and the message you want to generate,
//! and `cligpt` will handle the rest.
//!
//! `cligpt` is written in [Rust](https://www.rust-lang.org/) and uses the
//! [`clap`](https://github.com/clap-rs/clap)
//! and
//! [`async-openai`](https://github.com/64bit/async-openai) crates
//! to provide a user-friendly interface for the `ChatGPT` API.
//! It's perfect for developers who want
//! to quickly prototype text generation applications
//! or for anyone who wants to experiment with the power of the `ChatGPT` API
//! from the comfort of their terminal.
//!
//! # Installation
//!
//! ## Requirements
//!
//! Before installing `cligpt`,
//! you need to make sure you have
//! [Rust](https://www.rust-lang.org/tools/install) (version 1.65.0 or later)
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
//! you can install `cligpt` directly from the GitHub repository
//! using Cargo by running:
//!
//! ```bash
//! cargo install --git=https://github.com/schneiderfelipe/cligpt.git
//! ```
//!
//! ## By cloning the GitHub repository
//!
//! You can also build `cligpt` from source by cloning the GitHub repository
//! and running `cargo build`:
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
//! you need to provide your
//! [`OpenAI` API key](https://platform.openai.com/account/api-keys)
//! and the message you want to generate.
//! You can provide the API key using the `-k` or `--api-key` option,
//! or by setting the `OPENAI_API_KEY` environment variable.
//!
//! Here's an example usage:
//!
//! ```bash
//! cligpt --api-key YOUR_API_KEY 'Hello, ChatGPT!'
//! ```
//!
//! This will send the message `'Hello, ChatGPT!'` to the `ChatGPT` API using
//! your API key and print the generated text to your terminal.
//!
//! You can also specify additional options to customize the text generation,
//! such as temperature,
//! and the language model to use.
//!
//! For example,
//! you can set the temperature to 0.9 and use GPT-4:
//!
//! ```bash
//! cligpt --temperature 0.9 --model gpt4 'Hello, ChatGPT!'
//! ```
//!
//! In the example above,
//! the API key will be read from the environment.
//!
//! For more information on available options,
//! run `cligpt --help`.
//!
//! # Design decisions
//!
//! The primary goal of `cligpt` is to provide a user-friendly experience.
//! For this reason,
//! it is designed to generate only a single response,
//! whose maximum length is determined by the
//! [`OpenAI` API endpoint](https://platform.openai.com/docs/api-reference/chat/create#chat/create-max_tokens).
//!
//! As a command-line application,
//! `cligpt` allows for the use of
//! [pipes and redirections](https://askubuntu.com/q/172982/361183)
//! to load and save prompts and generated text,
//! making such features of limited use in `cligpt`.
//!
//! Lastly,
//! `cligpt` only supports the
//! [chat completion endpoint](https://platform.openai.com/docs/api-reference/chat/create#chat/create-model).
//!
//! # Contributing
//!
//! Contributions to `cligpt` are welcome!
//! If you find a bug,
//! have a feature request,
//! or want to contribute code,
//! please [open an issue](https://github.com/schneiderfelipe/cligpt/issues/new)
//! or [a pull request](https://github.com/schneiderfelipe/cligpt/pulls)
//! on the [GitHub repository](https://github.com/schneiderfelipe/cligpt).
//!
//! # License
//!
//! `cligpt` is released under the [MIT License](LICENSE).

use std::{
    fmt::Write as _,
    fs::{self, File},
    io::{self, Read, Write},
    ops::RangeInclusive,
    path::Path,
};

use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs,
        CreateEmbeddingRequestArgs, Role,
    },
    Client,
};
use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::eyre::{self, Context};
use futures_util::StreamExt;

/// A command-line interface to talk to `ChatGPT`.
#[derive(Debug, Parser)]
#[command(version, author, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Text to prepend to the message as context.
    context: Vec<String>,

    /// Model to use for the chat.
    #[arg(long, value_enum, default_value_t = Default::default())]
    model: Model,

    /// Temperature to use for the chat.
    #[arg(long, default_value_t = 0.7, value_parser = temperature_parser)]
    temperature: f32,

    /// Your OpenAI API key.
    #[arg(short = 'k', long, value_parser = api_key_parser, env = "OPENAI_API_KEY")]
    api_key: String,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create a new chat.
    NewChat,
    /// List all chats.
    ListChats,
    /// Delete a chat.
    DeleteChat {
        /// Name or ID of the chat to delete.
        chat_name: String,
    },
    /// Switch to a different chat.
    SwitchChat {
        /// Name or ID of the chat to switch to.
        chat_name: String,
    },
    /// Rename a chat.
    RenameChat {
        /// Name or ID of the chat to rename.
        chat_name: String,
        /// New name of the chat.
        new_name: String,
    },
}

/// Different language models that can be used for natural language processing tasks.
#[derive(Clone, Copy, Debug, Default, ValueEnum)]
enum Model {
    /// A highly capable GPT-3.5 model optimized for chat at a reduced cost.
    #[default]
    Gpt35,

    /// A more capable model than any GPT-3.5,
    /// designed for complex tasks and optimized for chat.
    Gpt4,
}

impl Model {
    #[inline]
    const fn name(self) -> &'static str {
        match self {
            Self::Gpt35 => "gpt-3.5-turbo",
            Self::Gpt4 => "gpt-4",
        }
    }
}

const TEMPERATURE_RANGE: RangeInclusive<f32> = 0.0..=1.0;

#[inline]
fn temperature_parser(temperature: &str) -> eyre::Result<f32> {
    let temperature: f32 = temperature.parse()?;
    eyre::ensure!(
        temperature >= *TEMPERATURE_RANGE.start(),
        "too low (minimum value is {:.1})",
        *TEMPERATURE_RANGE.start()
    );
    eyre::ensure!(
        temperature <= *TEMPERATURE_RANGE.end(),
        "too high (maximum value is {:.1})",
        *TEMPERATURE_RANGE.end()
    );

    Ok(temperature)
}

const API_KEY_RANGE: RangeInclusive<usize> = 40..=50;

// Logic from <https://docs.gitguardian.com/secrets-detection/detectors/specifics/openai_apikey>.
#[inline]
fn api_key_parser(api_key: &str) -> eyre::Result<String> {
    eyre::ensure!(
        !api_key.is_empty(),
        "cannot use empty string as OpenAI API key"
    );
    eyre::ensure!(
        !api_key.trim().is_empty(),
        "cannot use all-whitespace string as OpenAI API key"
    );

    eyre::ensure!(
        api_key.starts_with("sk-"),
        "'{api_key}' does not start with 'sk-'"
    );

    let suffix = &api_key[3..];
    if let Some(offending_char) = suffix.chars().find(|c| !c.is_ascii_alphanumeric()) {
        eyre::bail!("'{api_key}' contains invalid character '{offending_char}'");
    }

    let key_len = suffix.len();
    eyre::ensure!(
        key_len >= *API_KEY_RANGE.start(),
        "'{api_key}' is too short (expected at least {} characters)",
        API_KEY_RANGE.start()
    );
    eyre::ensure!(
        key_len <= *API_KEY_RANGE.end(),
        "'{api_key}' is too long (expected at most {} characters)",
        API_KEY_RANGE.end()
    );

    Ok(api_key.into())
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install().context("failed to install error report handler")?;

    let cli = Cli::parse();
    if let Some(command) = cli.command {
        match command {
            Command::NewChat => todo!(),
            Command::ListChats => todo!(),
            Command::DeleteChat { chat_name: _ } => todo!(),
            Command::SwitchChat { chat_name: _ } => todo!(),
            Command::RenameChat {
                chat_name: _,
                new_name: _,
            } => todo!(),
        }
    } else {
        let message = {
            let mut message = String::new();
            io::stdin()
                .lock()
                .read_to_string(&mut message)
                .context("failed to read from the standard input")?;
            message
        };

        let context = cli.context.join(" ");
        let message = match (context.is_empty(), message.is_empty()) {
            (false, false) => [context, message].join(" "),
            (false, true) => context,
            (true, false) => message,
            (true, true) => eyre::bail!("cannot use empty string as chat message"),
        };
        eyre::ensure!(
            !message.trim().is_empty(),
            "cannot use all-whitespace string as chat message"
        );

        let path = Path::new("cligpt.chat.json");
        let mut embedded_messages = if path.try_exists()? {
            eprintln!("Reading contents from {}", path.display());

            let contents = fs::read_to_string(path)
                .with_context(|| format!("failed to read from {}", path.display()))?;

            // https://github.com/serde-rs/json/issues/160#issuecomment-253446892
            serde_json::from_str(&contents)
                .with_context(|| format!("failed to deserialize contents of {}", path.display()))?
        } else {
            Vec::new()
        };

        let api_key = cli.api_key;
        let client = Client::new().with_api_key(api_key);

        let message = strip_trailing_newline(&message);
        let message_embedding = embed(&client, message).await?;
        embedded_messages.push((
            ChatCompletionRequestMessageArgs::default()
                .content(message)
                .build()
                .context("failed to build chat message")?,
            message_embedding,
        ));

        let model = cli.model;
        let temperature = cli.temperature;

        let request = CreateChatCompletionRequestArgs::default()
            .model(model.name())
            .temperature(temperature)
            .messages(
                embedded_messages
                    .iter()
                    .cloned()
                    .map(|(message, _)| message)
                    .collect::<Vec<_>>(),
            )
            .build()
            .context("failed to build the completion request")?;
        let mut stream = client
            .chat()
            .create_stream(request)
            .await
            .context("failed to create the completion stream")?;

        let buffer = {
            let mut stdout = io::stdout().lock();
            let mut buffer = String::new();

            writeln!(stdout).context("failed to write new line to the standard output")?;
            while let Some(result) = stream.next().await {
                let response = result.context("failed to obtain a stream response")?;
                if let Some(choice) = response.choices.get(0) {
                    if let Some(text) = &choice.delta.content {
                        write!(stdout, "{text}")
                            .context("failed to write response delta to the standard output")?;
                        write!(buffer, "{text}")
                            .context("failed to write response delta to buffer")?;
                    }
                }
            }
            writeln!(stdout).context("failed to write new line to the standard output")?;
            writeln!(buffer).context("failed to write new line to buffer")?;
            buffer
        };

        let buffer = strip_trailing_newline(&buffer);
        let buffer_embedding = embed(&client, buffer).await?;
        embedded_messages.push((
            ChatCompletionRequestMessageArgs::default()
                .content(buffer)
                .role(Role::Assistant)
                .build()
                .context("failed to build chat message")?,
            buffer_embedding,
        ));

        let mut iter = embedded_messages.iter().enumerate().rev();
        let last_response = iter.next().unwrap();
        let last_request = iter.next().unwrap();
        let most_similar = iter
            .map(|(n, (c, e))| {
                (
                    n,
                    c,
                    cosine_similarity(e, &last_request.1 .1)
                        .max(cosine_similarity(e, &last_response.1 .1)),
                )
            })
            .max_by(|(_, _, x), (_, _, y)| x.partial_cmp(y).unwrap());
        eprintln!("{most_similar:#?}");
        if matches!(
            most_similar.map(|(_, m, _)| &m.role),
            Some(&Role::Assistant)
        ) {
            eprintln!("Should get the previous one actually");
        }

        if true {
            eprintln!("\nWriting contents to {}", path.display());

            let file = File::create(path)?;
            serde_json::to_writer(file, &embedded_messages)
                .with_context(|| format!("failed to serialize contents to {}", path.display()))?;
        }
    }

    Ok(())
}

const EMBEDDING_LENGTH: usize = 1536;

#[inline]
async fn embed(client: &Client, input: &str) -> eyre::Result<Vec<f32>> {
    let request = CreateEmbeddingRequestArgs::default()
        .model("text-embedding-ada-002")
        .input(input)
        .build()?;
    let response = client.embeddings().create(request).await?;
    let data = response.data.into_iter().next();
    let embedding = data
        .map(|data| data.embedding)
        .ok_or_else(|| eyre::eyre!("failed to embed '{input}'"))?;
    eyre::ensure!(
        embedding.len() == EMBEDDING_LENGTH,
        "embedding has incorrect length (expected {EMBEDDING_LENGTH}, got {})",
        embedding.len()
    );
    Ok(embedding)
}

// https://github.com/openai/openai-python/blob/47ce29542e7fc496c1cd0bb323293b7991f45bb0/openai/embeddings_utils.py#L67-L68
#[inline]
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    #[inline]
    fn dot(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b).map(|(a, b)| a * b).sum()
    }
    dot(a, b) / (dot(a, a) * dot(b, b)).sqrt()
}

// https://stackoverflow.com/a/66401342/4039050
#[inline]
fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or_else(|| input.strip_suffix('\n'))
        .unwrap_or(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;

        Cli::command().debug_assert();
    }

    #[test]
    fn strip_newline_works() {
        assert_eq!(strip_trailing_newline("Test0\r\n\r\n"), "Test0\r\n");
        assert_eq!(strip_trailing_newline("Test1\r\n"), "Test1");
        assert_eq!(strip_trailing_newline("Test2\n"), "Test2");
        assert_eq!(strip_trailing_newline("Test3"), "Test3");
    }

    #[test]
    fn cosine_similarity_works() {
        use approx::assert_abs_diff_eq;

        assert_abs_diff_eq!(cosine_similarity(&[0.0, 1.0], &[0.0, 1.0]), 1.0);
        assert_abs_diff_eq!(cosine_similarity(&[0.0, 1.0], &[1.0, 0.0]), 0.0);
        assert_abs_diff_eq!(cosine_similarity(&[0.0, 1.0], &[0.5, 0.5]), 0.707_106_77);
    }
}
