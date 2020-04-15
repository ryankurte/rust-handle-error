
//! An error handling / bubbling macro to reduce error handling boilerplate.
//! 
//! For a given fallible expression (expression returning a result), such as:
//! 
//! ```no_run
//! # type E = ();
//! #
//! fn do_something() -> Result<(), E> {
//!     // ....
//! # Ok(())
//! }
//! ```
//! 
//! This can be used as follows:
//! 
//! ```no_run
//! #[macro_use]
//! extern crate log;
//! 
//! #[macro_use]
//! extern crate handle_error;
//! 
//! # type E = ();
//! #
//! # fn do_something() -> Result<(), E> {
//! #     unimplemented!()
//! # }
//! #
//! fn main() -> Result<(), E> {
//!   let v = handle_error!(do_something(), "Failed to do something");
//!   Ok(())
//! }
//! ```
//! ```no_run
//! #[macro_use]
//! extern crate log;
//! 
//! #[macro_use]
//! extern crate handle_error;
//! 
//! # type E = ();
//! #
//! # fn do_something() -> Result<(), E> {
//! #     unimplemented!()
//! # }
//! #
//! fn main() -> Result<(), E> {
//!   let v = retry_error!(3, do_something(), "Failed to do something");
//!   Ok(())
//! }
//! ```
//! 
//! Replacing the common patterns:
//! 
//! ```no_run
//! #[macro_use]
//! extern crate log;
//! 
//! #[macro_use]
//! extern crate handle_error;
//! 
//! # type E = ();
//! # fn do_something() -> Result<(), E> {
//! #     unimplemented!()
//! # }
//! #
//! // Match case where we care about the ok value
//! fn example_one() -> Result<(), E> {
//!   let v = match do_something() {
//!     Ok(v) => v,
//!     Err(e) => {
//!       error!("Failed to do something");
//!       return Err(e);
//!     }
//!   };
//! 
//!   Ok(())
//! }
//! 
//! // If let where we do not care about the ok value
//! fn example_two() -> Result<(), E> {
//!   if let Err(e) = do_something() {
//!     error!("Failed to do something");
//!     return Err(e);
//!   }
//! 
//!   Ok(())
//! }
//! 
//! # fn main() {}
//! ```


/// Log and propagate the error result from a given expression
///
/// This logs the provided message and exits the function scope on error, and returns
/// the unpacked Ok(value) on success.
#[macro_export]
macro_rules! handle_error {
    ($call:expr, $msg:expr, $($params:tt)*) => (
        match $call {
            Ok(v) => v,
            Err(e) => {
                error!($msg, $($params)*);
                return Err(e).into();
            },
        };
    );
    ($call:expr, $msg:expr) => (
        match $call {
            Ok(v) => v,
            Err(e) => {
                error!($msg);
                return Err(e).into();
            },
        };
    );
}

/// Retry a provided fallible function N times
///
/// This will optionally log a message, and returns the final error if all attempts fail
#[macro_export]
macro_rules! retry_error {
    ($retries:expr, $fallible:expr, $($params:tt)*) => (
        (|| {
            let mut i = 0;
            loop {
                match $fallible {
                    Ok(v) => break Ok(v),
                    Err(e) if i < $retries => {
                        i += 1;
                    },
                    Err(e) => {
                        error!($($params)*);
                        break Err(e)
                    },
                }
            }
        })()
    );
    ($retries:expr, $fallible:expr) => (
        (|| {
            let mut i = 0;
            loop {
                match $fallible {
                    Ok(v) => break Ok(v),
                    Err(e) if i < $retries => {
                        i += 1;
                    },
                    Err(e) => break Err(e),
                }
            }
        })()
    );
}

