# handle-error

An error handling / bubbling macro to reduce rust error handling boilerplate where `?` doesn't work because the _site_ of the error matters.

[![GitHub tag](https://img.shields.io/github/tag/ryankurte/rust-handle-error.svg)](https://github.com/ryankurte/rust-handle-error)
[![Travis Build Status](https://travis-ci.com/ryankurte/rust-handle-error.svg?branch=master)](https://travis-ci.com/ryankurte/rust-handle-error)
[![Crates.io](https://img.shields.io/crates/v/handle-error.svg)](https://crates.io/crates/handle-error)
[![Docs.rs](https://docs.rs/handle-error/badge.svg)](https://docs.rs/handle-error)

For a given fallible expression (expression returning a result), such as:

```rust
fn do_something() -> Result<(), E> {
    // ....
}
```

This can be used as follows:

```rust
#[macro_use]
extern crate log;

#[macro_use]
extern crate handle_error;

fn main() -> Result<(), E> {
  let v = handle_error!(do_something(), "Failed to do something");
  Ok(())
}
```

Replacing the common patterns:

```rust
#[macro_use]
extern crate log;

// Match case where we care about the ok value
fn example_one() -> Result<(), E> {
  let v = match do_something() {
    Ok(v) => v,
    Err(e) => {
      error!("Failed to do something");
      return Err(e);
    }
  };

  Ok(())
}

// If let where we do not care about the ok value
fn example_two() -> Result<(), E> {
  if let Err(e) = do_something() {
    error!("Failed to do something");
    return Err(e);
  }

  Ok(())
}
```
