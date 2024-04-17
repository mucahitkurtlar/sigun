# sigun

Simple and fast singleton static file server.

## Installation

### From source

To install `sigun` from source, you need to have `cargo` installed on your system. If you don't have `cargo` installed, you can install it by following the instructions on the [official website](https://doc.rust-lang.org/cargo/getting-started/installation.html).

```bash
git clone https://github.com/mucahitkurtlar/sigun.git
cd sigun
cargo install --path .
```

### From binary

You can download the latest binary from the [lates release](https://github.com/mucahitkurtlar/sigun/releases/latest) page.

## Usage

Before running the server, you need to create a configuration file. You can create a new one using example configuration file [`example.config.toml`](https://github.com/mucahitkurtlar/sigun/blob/master/example.config.toml) provided in the repository. The configuration file should be named `config.toml` and should be placed in the same directory as the running binary.

If you want to run the server with the `info` or `debug` log level, you can set the `RUST_LOG` environment variable to `info` or `debug` respectively.

```bash
RUST_LOG=info sigun
# or
RUST_LOG=debug sigun
```
