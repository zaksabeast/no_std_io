#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod reader;
pub use reader::*;

mod writer;
pub use writer::*;

mod error;
pub use error::*;

mod endian;
pub use endian::*;
