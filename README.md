derive(Error)
=============

[![Build Status](https://api.travis-ci.com/dtolnay/thiserror.svg?branch=master)](https://travis-ci.com/dtolnay/thiserror)
[![Latest Version](https://img.shields.io/crates/v/thiserror.svg)](https://crates.io/crates/thiserror)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/thiserror)

This library provides a convenient derive macro for the standard library's
[`std::error::Error`] trait.

[`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html

```toml
[dependencies]
thiserror = "1.0"
```

*Compiler support: requires rustc 1.31+*

<br>

## Example

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[source] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```

<br>

## Details

- Thiserror deliberately does not appear in your public API. You get the same
  thing as if you had written an implementation of `std::error::Error` by hand,
  and switching from handwritten impls to thiserror or vice versa is not a
  breaking change.

- Errors may be enums, structs with named fields, tuple structs, or unit
  structs.

- A `Display` impl is generated for your error if you provide `#[error("...")]`
  messages on the struct or each variant of your enum, as shown above in the
  example.

  The messages support a shorthand for interpolating fields from the error.

    - `#[error("{var}")]` ⟶ `write!("{}", self.var)`
    - `#[error("{0}")]` ⟶ `write!("{}", self.0)`
    - `#[error("{var:?}")]` ⟶ `write!("{:?}", self.var)`
    - `#[error("{0:?}")]` ⟶ `write!("{:?}", self.0)`

  You may alternatively write out the full format args yourself, using arbitrary
  expressions.

  When providing your own format args, the shorthand does not kick in so you
  need to specify `.var` in the argument list to refer to named fields and `.0`
  to refer to tuple fields.

  ```rust
  #[derive(Error, Debug)]
  pub enum Error {
      #[error("invalid rdo_lookahead_frames {} (expected < {})", .0, i32::max_value())]
      InvalidLookahead(i32),
  }
  ```

- The Error trait's `source()` method is implemented to return whichever field
  has a `#[source]` attribute, if any. This is for identifying the underlying
  lower level error that caused your error.

  Any error type that implements `std::error::Error` or dereferences to `dyn
  std::error::Error` will work as a source.

  ```rust
  #[derive(Error, Debug)]
  pub struct MyError {
      msg: String,
      #[source]
      source: anyhow::Error,
  }
  ```

- The Error trait's `backtrace()` method is implemented to return whichever
  field has a type named `Backtrace`, if any.

  ```rust
  use std::backtrace::Backtrace;

  #[derive(Error, Debug)]
  pub struct MyError {
      msg: String,
      backtrace: Backtrace, // automatically detected
  }
  ```

- See also the [`anyhow`] library for a convenient single error type to use in
  application code.

  [`anyhow`]: https://github.com/dtolnay/anyhow

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
