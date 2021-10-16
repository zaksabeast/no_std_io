use super::EndianRead;
use core::mem;
use safe_transmute::{transmute_many_permissive, TriviallyTransmutable};
use snafu::Snafu;

#[derive(Debug, Snafu)]
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
}

pub type ReaderResult<T> = Result<T, Error>;

/// An interface to safely read values from a source.
pub trait Reader {
    /// Returns the data to be read from.
    fn get_slice(&self) -> &[u8];

    /// Gets a slice of bytes from an offset of a source where `slice.len() == T.len()`.
    ///
    /// An error should be returned if the size is invalid (e.g. `offset + T.len()` exceeds the available data)
    /// or if the alignment is incorrect.
    fn get_sized_slice<T: Sized>(&self, offset: usize) -> ReaderResult<&[u8]> {
        let data = self.get_slice();
        let result_size = mem::size_of::<T>();
        let offset_end = offset + result_size;

        if data.len() < offset_end {
            return Err(Error::InvalidSize {
                wanted_size: result_size,
                available_size: data.len(),
            });
        }

        Ok(&data[offset..offset_end])
    }

    /// Safely gets a [TriviallyTransmutable] reference.
    /// Errors will be returned if the offset does not have enough data for the target type
    /// or is unaligned.
    fn get_transmutable<T: TriviallyTransmutable>(&self, offset: usize) -> ReaderResult<&T> {
        // Read enough bytes for one of the type
        let bytes = self.get_sized_slice::<T>(offset)?;

        // Transmute to a slice as a hack to transmute a reference
        let read_value =
            transmute_many_permissive::<T>(bytes).map_err(|_| Error::InvalidAlignment {
                wanted_size: mem::size_of::<T>(),
                source_size: bytes.len(),
                source_offset: offset,
            })?;

        // If we get here we're guaranteed to have one value (and only one)
        // so we can unwrap
        Ok(read_value.first().unwrap())
    }

    /// Same as [Reader::get_transmutable], but copies the reference to be an owned value.
    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> ReaderResult<T> {
        Ok(*self.get_transmutable(offset)?)
    }

    /// Same as [Reader::read], but returns a default value if the read is invalid.
    fn default_read<T: TriviallyTransmutable + Default>(&self, offset: usize) -> T {
        self.read(offset).unwrap_or_default()
    }

    /// Reads a value from its little endian representation.
    ///
    /// Prefer endian agnostic methods when possible.
    /// This should only be used when reading data from a format or protocol
    /// that explicitly defines little endian.
    fn read_le<T: EndianRead>(&self, offset: usize) -> ReaderResult<T> {
        let bytes = self.get_sized_slice::<T>(offset)?;
        Ok(T::read_le(&bytes))
    }

    /// Same as [Reader::read_le], but returns a default value if the read is invalid.
    ///
    /// Prefer endian agnostic methods when possible.
    /// This should only be used when reading data from a format or protocol
    /// that explicitly defines little endian.
    fn default_read_le<T: EndianRead + Default>(&self, offset: usize) -> T {
        self.read_le(offset).unwrap_or_default()
    }

    /// Reads a value from its big endian representation.
    ///
    /// Prefer endian agnostic methods when possible.
    /// This should only be used when reading data from a format or protocol
    /// that explicitly defines little endian.
    fn read_be<T: EndianRead>(&self, offset: usize) -> ReaderResult<T> {
        let bytes = self.get_sized_slice::<T>(offset)?;
        Ok(T::read_be(&bytes))
    }

    /// Same as [Reader::read_be], but returns a default value if the read is invalid.
    ///
    /// Prefer endian agnostic methods when possible.
    /// This should only be used when reading data from a format or protocol
    /// that explicitly defines little endian.
    fn default_read_be<T: EndianRead + Default>(&self, offset: usize) -> T {
        self.read_be(offset).unwrap_or_default()
    }
}
