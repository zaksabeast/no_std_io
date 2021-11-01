use snafu::Snafu;

#[derive(Debug, PartialEq, Snafu)]
pub enum Error {
    #[snafu(display(
        "Invalid size, wanted: {}, available: {} ",
        wanted_size,
        available_size
    ))]
    InvalidSize {
        wanted_size: usize,
        available_size: usize,
    },
    #[snafu(display(
        "Invalid alignment: wanted size: {}, source size: {}, source offset: {}",
        wanted_size,
        source_size,
        source_offset
    ))]
    InvalidAlignment {
        wanted_size: usize,
        source_size: usize,
        source_offset: usize,
    },
    /// Generic read error message to describe a custom read error by the implementor.
    #[snafu(display("Invalid read: {}", message))]
    InvalidRead { message: &'static str },
    /// Generic write error message to describe a custom write error by the implementor.
    #[snafu(display("Invalid write: {}", message))]
    InvalidWrite { message: &'static str },
}
