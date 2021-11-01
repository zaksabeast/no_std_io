use super::{EndianWrite, Error};
use core::mem;
use safe_transmute::{transmute_one_to_bytes, TriviallyTransmutable};

pub type WriterResult<T> = Result<T, Error>;

/// An interface to safely write values to a source.
pub trait Writer {
    /// Returns the data to be read from.
    fn get_mut_slice(&mut self) -> &mut [u8];

    /// Gets a slice of bytes with a specified length from an offset of a source.
    ///
    /// An error should be returned if the size is invalid.
    fn get_sized_mut_slice(&mut self, offset: usize, length: usize) -> WriterResult<&mut [u8]> {
        let data = self.get_mut_slice();
        let offset_end = offset + length;

        if data.len() < offset_end {
            return Err(Error::InvalidSize {
                wanted_size: length,
                available_size: offset_end,
            });
        }

        Ok(&mut data[offset..offset_end])
    }

    /// Same as [Writer::get_sized_mut_slice], except the length comes from `T.len()`.
    fn get_type_sized_mut_slice<T: Sized>(&mut self, offset: usize) -> WriterResult<&mut [u8]> {
        let length = mem::size_of::<T>();
        self.get_sized_mut_slice(offset, length)
    }

    /// Writes bytes to an offset and returns the number of bytes written.
    ///
    /// Errors if the byte slice length will not fit at the offset.
    fn write_bytes(&mut self, offset: usize, bytes: &[u8]) -> WriterResult<usize> {
        let length = bytes.len();
        let slice = self.get_sized_mut_slice(offset, length)?;

        slice.copy_from_slice(bytes);
        Ok(length)
    }

    /// Same as [Writer::write_bytes], but writes a [TriviallyTransmutable] type by converting it to bytes.
    fn write<T: TriviallyTransmutable>(&mut self, offset: usize, value: &T) -> WriterResult<usize> {
        let bytes = transmute_one_to_bytes(value);
        self.write_bytes(offset, bytes)
    }

    /// Writes a value in its little endian representation.
    ///
    /// Prefer endian agnostic methods when possible.
    /// This should only be used when reading data from a format or protocol
    /// that explicitly defines little endian.
    fn write_le<T: EndianWrite>(&mut self, offset: usize, value: &T) -> WriterResult<()> {
        let bytes = self.get_type_sized_mut_slice::<T>(offset)?;
        T::write_le(value, bytes);
        Ok(())
    }

    /// Writes a value in its big endian representation.
    ///
    /// Prefer endian agnostic methods when possible.
    /// This should only be used when reading data from a format or protocol
    /// that explicitly defines big endian.
    fn write_be<T: EndianWrite>(&mut self, offset: usize, value: &T) -> WriterResult<()> {
        let bytes = self.get_type_sized_mut_slice::<T>(offset)?;
        T::write_be(value, bytes);
        Ok(())
    }
}
