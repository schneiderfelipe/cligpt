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
//! cligpt --api-key YOUR_API_KEY chat 'Hello, ChatGPT!'
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
//! cligpt --temperature 0.9 --model gpt4 chat 'Hello, ChatGPT!'
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

use std::fmt::Write as _;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::ops::RangeInclusive;
use std::path::Path;

use async_openai::types::ChatCompletionRequestMessage;
use async_openai::types::ChatCompletionRequestMessageArgs;
use async_openai::types::ChatCompletionResponseStream;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_openai::types::CreateEmbeddingRequestArgs;
use async_openai::types::Role;
use async_openai::Client;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use color_eyre::eyre;
use color_eyre::eyre::Context;
use directories::ProjectDirs;
use futures_util::StreamExt;

const API_KEY_RANGE: RangeInclusive<usize> = 40..=50;
const TEMPERATURE_RANGE: RangeInclusive<f32> = 0.0..=1.0;

const EMBEDDING_LENGTH: usize = 1536;

type Embedding = Vec<f32>;
type EmbeddedMessage = (ChatCompletionRequestMessage, Embedding);

/// A command-line interface to talk to `ChatGPT`.
#[derive(Debug, Parser)]
#[command(version, author, about)]
struct Cli {
    /// Command to perform if not chatting with the AI.
    #[command(subcommand)]
    command: Option<Command>,

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
    /// Show a chat.
    #[command(alias = "s")]
    Show,
}

/// Different language models that can be used for natural language processing
/// tasks.
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

    let path = {
        let Some(proj_dirs) = ProjectDirs::from("com", "schneiderfelipe", "cligpt") else {
            eyre::bail!("failed to obtain project directory");
        };
        let cache_dir = proj_dirs.cache_dir();
        fs::create_dir_all(cache_dir)?;
        cache_dir.join("chat.json")
    };

    if let Some(command) = cli.command {
        match command {
            Command::Show => handle_show(path)?,
        }
    } else {
        handle_chat(cli.model, cli.temperature, cli.api_key, path).await?;
    }

    Ok(())
}

#[inline]
fn read_message_from_stdin() -> eyre::Result<String> {
    let mut message = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut message)
        .context("failed to read from the standard input")?;
    Ok(message)
}

#[inline]
async fn process_chat_response(stream: &mut ChatCompletionResponseStream) -> eyre::Result<String> {
    let mut stdout = io::stdout().lock();
    let mut buffer = String::new();

    writeln!(stdout).context("failed to write new line to the standard output")?;
    while let Some(result) = stream.next().await {
        let response = result.context("failed to obtain a stream response")?;
        if let Some(choice) = response.choices.get(0) {
            if let Some(text) = &choice.delta.content {
                write!(stdout, "{text}")
                    .context("failed to write response delta to the standard output")?;
                write!(buffer, "{text}").context("failed to write response delta to buffer")?;
                stdout.flush()?
            }
        }
    }
    writeln!(stdout).context("failed to write new line to the standard output")?;
    writeln!(buffer).context("failed to write new line to buffer")?;
    Ok(buffer)
}

#[inline]
fn handle_show(path: impl AsRef<Path>) -> eyre::Result<()> {
    let chat = read_chat_from_path(path)?;

    let mut stdout = io::stdout().lock();
    for (message, _) in chat {
        if let Some(name) = message.name {
            writeln!(stdout, "{name}:")?;
        } else {
            writeln!(stdout, "{name}:", name = message.role)?;
        }
        writeln!(stdout, "{}", message.content)?;
        writeln!(stdout)?;
        stdout.flush()?;
    }

    Ok(())
}

#[inline]
async fn handle_chat(
    model: Model,
    temperature: f32,
    api_key: impl Into<String>,
    path: impl AsRef<Path>,
) -> eyre::Result<()> {
    let message = read_message_from_stdin()?;
    eyre::ensure!(
        !message.trim().is_empty(),
        "cannot use all-whitespace string as chat message"
    );

    let mut chat = read_chat_from_path(&path)?;

    let client = Client::new().with_api_key(api_key);

    let message = strip_trailing_newline(&message);
    let message_embedding = embed(&client, message).await?;
    chat.push((
        ChatCompletionRequestMessageArgs::default()
            .content(message)
            .build()
            .context("failed to build chat message")?,
        message_embedding,
    ));

    let request = CreateChatCompletionRequestArgs::default()
        .model(model.name())
        .temperature(temperature)
        .messages(
            chat.iter()
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

    let buffer = process_chat_response(&mut stream).await?;

    let buffer = strip_trailing_newline(&buffer);
    let buffer_embedding = embed(&client, buffer).await?;
    chat.push((
        ChatCompletionRequestMessageArgs::default()
            .content(buffer)
            .role(Role::Assistant)
            .build()
            .context("failed to build chat message")?,
        buffer_embedding,
    ));

    let (current_chat, _outdated_chat) = split_chat(chat)?;

    write_chat_to_path(&current_chat, path)?;

    Ok(())
}

#[inline]
fn read_chat_from_path(path: impl AsRef<Path>) -> eyre::Result<Vec<EmbeddedMessage>> {
    let path = path.as_ref();

    let chat = if path.try_exists()? {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read from {}", path.display()))?;

        // https://github.com/serde-rs/json/issues/160#issuecomment-253446892
        serde_json::from_str(&contents)
            .with_context(|| format!("failed to deserialize contents of {}", path.display()))?
    } else {
        Vec::new()
    };
    Ok(chat)
}

#[inline]
fn write_chat_to_path(chat: &[EmbeddedMessage], path: impl AsRef<Path>) -> eyre::Result<()> {
    let path = path.as_ref();

    let file = fs::File::create(path)?;
    serde_json::to_writer(file, chat)
        .with_context(|| format!("failed to serialize contents to {}", path.display()))?;

    Ok(())
}

#[inline]
fn split_chat(
    mut chat: Vec<EmbeddedMessage>,
) -> eyre::Result<(Vec<EmbeddedMessage>, Option<Vec<EmbeddedMessage>>)> {
    if chat.len() < 4 {
        return Ok((chat, None));
    }

    let (mut n_most_similar, mut n_least_similar) = {
        let mut iter = chat.iter().enumerate().rev();
        let last_response = iter.next().unwrap();
        let last_request = iter.next().unwrap();

        let most_similar = iter
            .map(|(n, (_, embedding))| {
                (
                    n,
                    cosine_similarity(embedding, &last_request.1 .1)
                        .max(cosine_similarity(embedding, &last_response.1 .1)),
                )
            })
            .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap());

        let mut iter = chat.iter().enumerate().rev();
        let last_response = iter.next().unwrap();
        let last_request = iter.next().unwrap();

        let least_similar = iter
            .map(|(n, (_, embedding))| {
                (
                    n,
                    cosine_similarity(embedding, &last_request.1 .1)
                        .max(cosine_similarity(embedding, &last_response.1 .1)),
                )
            })
            .min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap());

        let (most_similar, least_similar) = (most_similar.unwrap(), least_similar.unwrap());
        eyre::ensure!(
            most_similar.1 >= least_similar.1,
            "most similar is less similar than least similar"
        );
        (most_similar.0, least_similar.0)
    };

    if chat[n_most_similar].0.role == Role::Assistant {
        n_most_similar -= 1;
    }
    if chat[n_least_similar].0.role == Role::Assistant {
        n_least_similar -= 1;
    }
    if n_most_similar <= n_least_similar {
        return Ok((chat, None));
    }

    let current_chat = chat.split_off(n_least_similar);
    let outdated_chat = chat;

    Ok((current_chat, Some(outdated_chat)))
}

#[inline]
async fn embed(client: &Client, input: &str) -> eyre::Result<Embedding> {
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
