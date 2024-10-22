//! Define and read configuration file.
//!
//! A configuration file must not be: execute by owner; read, write or execute
//! by other users; read, write or execute by other groups. That mean a
//! configuration file may has the following modes: `400` or `600`. Other than
//! that, reading returns [Error::Insecured](result::Error::Insecured).
//!
//! # Example
//!
//! ```
#![doc = include_str!("../examples/read_jsonc.rs")]
//!```

mod reader;

pub mod result;

pub use reader::*;
