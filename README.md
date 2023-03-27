# cligpt

`cligpt` is a command-line interface for interacting with the `ChatGPT` API from `OpenAI`.
With `cligpt`,
you can quickly and easily generate text by sending prompts to `ChatGPT` from your terminal.
Simply provide your `OpenAI` API key and the message you want to generate,
and `cligpt` will handle the rest.

`cligpt` is written in [Rust](https://www.rust-lang.org/) and uses the clap and `cligpt` crates to provide a user-friendly interface for the `ChatGPT` API.
It's perfect for developers who want to quickly prototype text generation applications or for anyone who wants to experiment with the power of the `ChatGPT` API from the comfort of their terminal.

## Installation

### Requirements

Before installing `cligpt`,
you need to make sure you have [Rust](https://www.rust-lang.org/tools/install) (version 1.65.0 or later)
and [Cargo](https://doc.rust-lang.org/cargo/),
the package manager for Rust,
installed.

### From [crates.io](https://crates.io/) (recommended)

Once you have Rust and Cargo installed,
you can install `cligpt` from [crates.io](https://crates.io/) using Cargo:

```bash
cargo install cligpt
```

This will download the necessary dependencies,
compile the `cligpt` binary,
and install it in your system.
Once the installation is complete,
you can run `cligpt` by typing `cligpt` in your terminal.

### Directly from [GitHub](https://github.com/schneiderfelipe/cligpt)

Alternatively,
you can install `cligpt` directly from the GitHub repository using Cargo by running:

```bash
cargo install --git=https://github.com/schneiderfelipe/cligpt.git
```

### By cloning the GitHub repository

You can also build `cligpt` from source by cloning the GitHub repository and running `cargo build`:

```bash
git clone https://github.com/schneiderfelipe/cligpt.git
cd cligpt
cargo build --release
```

After building,
the binary will be located at `target/release/cligpt`.

Alternatively,
you can install the binary directly instead of just building it.
Just run `cargo install --path=.` instead of `cargo build --release`.

## Usage

To use `cligpt`,
you need to provide your [`OpenAI` API key](https://platform.openai.com/account/api-keys) and the message you want to generate.
You can provide the API key using the `-k` or `--api-key` option,
or by setting the `OPENAI_API_KEY` environment variable.

Here's an example usage:

```bash
cligpt --api-key YOUR_API_KEY 'Hello, ChatGPT!'
```

This will send the message `'Hello, ChatGPT!'` to the `ChatGPT` API using your API key and print the generated text to your terminal.
