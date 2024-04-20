# sigun

Simple and fast static file server.

## Installation

### From source

To install `sigun` from source, you need to have `cargo` installed on your system. If you don't have `cargo` installed, you can install it by following the instructions on the [official website](https://doc.rust-lang.org/cargo/getting-started/installation.html).

```bash
git clone https://github.com/mucahitkurtlar/sigun.git
cd sigun
cargo install --path .
```

### From binary

You can download the latest binary from the [latest release](https://github.com/mucahitkurtlar/sigun/releases/latest) page.

## Usage

Before running the server, you need to create a configuration file. You can create a new one using example configuration file [`example.sigun.toml`](example.sigun.toml) provided in the repository. The default configuration file name is `sigun.toml` in the current directory. But you can specify a different configuration file using the `--config` argument.

```bash
sigun --config /path/to/your/config.toml
```

If you want to run the server with the `info` or `debug` log level, you can set the `RUST_LOG` environment variable to `info` or `debug` respectively.

```bash
RUST_LOG=info sigun
# or
RUST_LOG=debug sigun
```

In this documentation, we will use the `curl` command to demonstrate how to interact with the server. You can use any other tool you prefer.

### Generating an OTP

To generate a one-time password (OTP), you can make a `POST` request to the `/new-otp` endpoint.

```bash
curl --location 'http://127.0.0.1:4598/new-otp' \
--header 'Content-Type: application/x-www-form-urlencoded' \
--data-urlencode 'secret=your_secret_here'
```

- example response:

```json
otp-RdKgcDvsnPfjefCa
```

### Uploading files

```bash
curl --location 'http://127.0.0.1:4598/upload' \
--form 'token="your_secret_here"' \ # or OTP
--form 'subdir="/images"' \
--form 'hello.png=@"path/to/hello.png"'
```

Also, you can upload multiple files at once.

```bash
curl --location 'http://127.0.0.1:4598/upload' \
--form 'token="your_secret_here"' \ # or OTP
--form 'subdir="/pdfs"' \
--form 'agreement.pdf=@"path/to/user-agreement.pdf"' \
--form 'terms.pdf=@"path/to/terms-and-conditions.pdf"' \
--form 'subdir="/images"' \
--form 'profile.jpg=@"path/to/default-profile.jpg"'
```

### Downloading files

You can simply download files by making a `GET` request to the file path.

```bash
curl --location 'http://127.0.0.1:4598/media/images/hello.png'
```

So you can directly access the file by visiting the following URL in your browser. Here is an HTML example:

```html
<img src="http://127.0.0.1:4598/media/images/hello.png" alt="hello.png" />
```

### Deleting files

To delete a file, you can make a `DELETE` request as follows:

```bash
curl --location --request DELETE 'http://127.0.0.1:4598/delete' \
--header 'Content-Type: application/x-www-form-urlencoded' \
--data-urlencode 'token=your_secret_here' \ # or OTP
--data-urlencode 'path=images/hello.png'
```
