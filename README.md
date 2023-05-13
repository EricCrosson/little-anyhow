# little-anyhow

[![Build Status]](https://github.com/EricCrosson/little-anyhow/actions/workflows/release.yml)

[build status]: https://github.com/EricCrosson/little-anyhow/actions/workflows/release.yml/badge.svg?event=push

**little-anyhow** is a vendorable error type with anyhow-style stack traces.
This little snippet helps you write low-dependency Rust binaries without your users noticing.

## Install

**little-anyhow** is intentionally not published to [crates.io] -- that would defeat the point!
It's not that the goal is use dependencies without transitive dependencies, or dependencies that never require updates.
The goal is to enable you to write ergonomic Rust binaries without _any_ dependencies!

Consequently, **little-anyhow** should be vendored in your binary crate:

```sh
curl \
  --location \
  --create-dirs \
  --no-clobber \
  --output src/little_anyhow.rs \
  https://raw.githubusercontent.com/EricCrosson/little-anyhow/master/src/lib.rs
```

[crates.io]: https://crates.io

## Use

Include the `little_anyhow` module in your `main.rs`:

```rust
mod little_anyhow;
```

**little-anyhow** is meant to be used in one place: the error type of the `Result` your `main` function returns.
This converts any error your `main` function produces into a `little_anyhow::Error`, which provides human-readabale error messages for your users.

For example:

```rust
fn main() -> Result<(), little_anyhow::Error> {
    writeln!(
        io::stdout(),
        "You can create a little_anyhow::Error from any type implementing `std::error::Error`"
    )?;
    let simulated_error = std::fmt::Error;  // an easy-to-create error type
    Err(simulated_error)?
}
```

Error messages for errors without a source look like:

```
Error: error reading `Blocks.txt`
```

Error messages for errors with a source look like:

```
Error: error reading `Blocks.txt`

Caused by:
    0: invalid Blocks.txt data on line 223
    1: one end of range is not a valid hexidecimal integer
    2: invalid digit found in string
```

## Acknowledgments

- [dtolnay/anyhow](https://github.com/dtolnay/anyhow)
- [Modular Errors in Rust](https://sabrinajewson.org/blog/errors)
