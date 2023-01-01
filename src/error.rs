use snafu::Snafu;

#[derive(Debug, PartialEq, Eq, Snafu)]
pub enum Error {
    #[snafu(display(
        "Invalid size: wanted 0x{:x} at offset offset: 0x{:x}, but data length is 0x{:x} ",
        wanted_size,
        offset,
        data_len
    ))]
    InvalidSize {
        wanted_size: usize,
        offset: usize,
        data_len: usize,
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

#[inline(always)]
pub(crate) fn add_error_context<T>(
    error: Result<T, Error>,
    offset: usize,
    data_len: usize,
) -> Result<T, Error> {
    error.map_err(|error| match error {
        Error::InvalidSize {
            wanted_size,
            offset: error_offset,
            ..
        } => Error::InvalidSize {
            wanted_size,
            offset: offset + error_offset,
            data_len,
        },
        _ => error,
    })
}
