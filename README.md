# cligpt

[![dependency status](https://deps.rs/repo/github/schneiderfelipe/cligpt/status.svg)](https://deps.rs/repo/github/schneiderfelipe/cligpt)

`cligpt` is a command-line interface for interacting
with the `ChatGPT` API from `OpenAI`.
With `cligpt`,
you can quickly and easily generate text by sending prompts to `ChatGPT`
from your terminal.
Simply provide your `OpenAI` API key and the message you want to generate,
and `cligpt` will handle the rest.

`cligpt` is written in [Rust](https://www.rust-lang.org/) and uses the
[`clap`](https://github.com/clap-rs/clap)
and
[`async-openai`](https://github.com/64bit/async-openai) crates
to provide a user-friendly interface for the `ChatGPT` API.

**Keep reading in order to learn how to [install](#installation) and [use](#usage) `cligpt`.**

## Available models

- `--model=gpt35` ([ChatGPT](https://openai.com/blog/introducing-chatgpt-and-whisper-apis), [`gpt-3.5-turbo`](https://platform.openai.com/docs/guides/chat), default)
- `--model=gpt4` ([GPT-4](https://openai.com/product/gpt-4), [`gpt-4`](https://platform.openai.com/docs/guides/chat))

## Common use cases

- Generate creative writing prompts or ideas.
- Answer questions about specific topics or subjects.
- Draft emails or other pieces of professional writing.
- Summarize long pieces of text.
- Translate text between different languages.

## Installation

### Requirements

Before installing `cligpt`,
you need to make sure you have
[Rust](https://www.rust-lang.org/tools/install) (version 1.65.0 or later)
and [Cargo](https://doc.rust-lang.org/cargo/),
the package manager for Rust,
installed.

### From [crates.io](https://crates.io/crates/cligpt) (recommended)

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
*That's it!*

### Directly from [GitHub](https://github.com/schneiderfelipe/cligpt)

Alternatively,
you can install `cligpt` directly from the GitHub repository
using Cargo by running:

```bash
cargo install --git=https://github.com/schneiderfelipe/cligpt.git
```

### By cloning the GitHub repository

You can also build `cligpt` from source by cloning the GitHub repository
and running `cargo build`:

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
you need to provide your
[`OpenAI` API key](https://platform.openai.com/account/api-keys)
and the message you want to generate.
You can provide the API key using the `-k` or `--api-key` option,
or by setting the `OPENAI_API_KEY` environment variable.

Here's an example usage:

```bash
cligpt --api-key YOUR_API_KEY 'Hello, ChatGPT!'
```

This will send the message `'Hello, ChatGPT!'` to the `ChatGPT` API using
your API key and print the generated text to your terminal.

You can also specify additional options to customize the text generation,
such as temperature,
and the language model to use.

For example,
you can set the temperature to 0.9 and use GPT-4:

```bash
cligpt --temperature 0.9 --model gpt4 'Hello, ChatGPT!'
```

In the example above,
the API key will be read from the environment.

`cligpt` supports receiving prompt both from the standard input and as positional arguments.
Each prompt piece is concatenated and separated by a single space, with the standard input coming last:

```console
$ echo "Repeat this message exactly how you read it" | cligpt Hello 'world!'
Hello world! Repeat this message exactly how you read it.
```

For more information on available options,
run `cligpt --help`.

## Contributing

Contributions to `cligpt` are welcome!
If you find a bug,
have a feature request,
or want to contribute code,
please [open an issue](https://github.com/schneiderfelipe/cligpt/issues/new)
or [a pull request](https://github.com/schneiderfelipe/cligpt/pulls)
on the [GitHub repository](https://github.com/schneiderfelipe/cligpt).

## License

`cligpt` is released under the [MIT License](LICENSE).
