[![crates.io](https://img.shields.io/crates/v/yuto51942-servant)](https://crates.io/crates/yuto51942-servant)

# Servant

Utilities cli written in Rust.

## How to use

### Install

```bash
cargo install yuto51942-servant
```

### Usage

```bash
# show version
sv --version

# show help
sv --help
```

```bash
servant 1.0.0
servant is utils cli.

USAGE:
    sv <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    lang       check installed programming languages
    nyancat    nyanyanyanyanya
    timer      countdown timer
    track      tracking
```

#### lang

```bash
sv lang --help
```

```bash
sv-lang 1.0.0
check installed programming languages

USAGE:
    sv lang [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --language <language>
```

#### nyancat

```bash
sv nyancat --help
```

```bash
sv-nyancat 1.0.0
nyanyanyanyanya

USAGE:
    sv nyancat

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

#### timer

```bash
sv timer --help
```

```bash
sv-timer 0.1.4
countdown timer

USAGE:
    sv timer --time <time>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --time <time>
```

#### track

```bash
sv track --help
```

```bash
sv-track 1.0.0
tracking

USAGE:
    sv track <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    create     Create tracking link
    delete     Delete tracking link and access history
    help       Prints this message or the help of the given subcommand(s)
    history    Show access history
    list       List all tracking links
```

## LICENSE

[MIT](LICENSE)
