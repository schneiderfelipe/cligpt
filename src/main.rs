//! A command-line interface for `ChatGPT`.

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
