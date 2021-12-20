#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};

use super::{EndianRead, Error};
use core::mem;
use safe_transmute::{transmute_many_permissive, TriviallyTransmutable};

pub type ReaderResult<T> = Result<T, Error>;

/// An interface to safely read values from a source.
pub trait Reader {
    /// Returns the data to be read from.
    fn get_slice(&self) -> &[u8];

    /// Gets a slice of bytes from an offset of a source where `slice.len() == size`.
    ///
    /// An error should be returned if the size is invalid (e.g. `offset + size` exceeds the available data)
    /// or if the alignment is incorrect.
    fn get_slice_of_size(&self, offset: usize, size: usize) -> ReaderResult<&[u8]> {
        let data = self.get_slice();
        let offset_end = offset + size;

        if data.len() < offset_end {
            return Err(Error::InvalidSize {
                wanted_size: size,
                available_size: data.len() - offset,
            });
        }

        Ok(&data[offset..offset_end])
    }

    /// Same as [Reader::get_slice_of_size], but uses `T.len()` for the size.
    fn get_sized_slice<T: Sized>(&self, offset: usize) -> ReaderResult<&[u8]> {
        let data = self.get_slice();
        let result_size = mem::size_of::<T>();
        let offset_end = offset + result_size;

        if data.len() < offset_end {
            return Err(Error::InvalidSize {
                wanted_size: result_size,
                available_size: data.len() - offset,
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
        Ok(T::read_le(bytes))
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
    /// that explicitly defines big endian.
    fn read_be<T: EndianRead>(&self, offset: usize) -> ReaderResult<T> {
        let bytes = self.get_sized_slice::<T>(offset)?;
        Ok(T::read_be(bytes))
    }

    /// Same as [Reader::read_be], but returns a default value if the read is invalid.
    ///
    /// Prefer endian agnostic methods when possible.
    /// This should only be used when reading data from a format or protocol
    /// that explicitly defines big endian.
    fn default_read_be<T: EndianRead + Default>(&self, offset: usize) -> T {
        self.read_be(offset).unwrap_or_default()
    }

    #[cfg(feature = "alloc")]
    /// Same as [Reader::get_slice_of_size], but converts the result to a vector.
    fn read_byte_vec(&self, offset: usize, size: usize) -> ReaderResult<Vec<u8>> {
        Ok(self.get_slice_of_size(offset, size)?.to_vec())
    }

    #[cfg(feature = "alloc")]
    /// Same as [Reader::read_byte_vec], but returns a zeroed
    /// out vector of the correct size if the read is invalid.
    fn default_read_byte_vec(&self, offset: usize, size: usize) -> Vec<u8> {
        self.read_byte_vec(offset, size)
            .unwrap_or_else(|_| vec![0; size])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct MockReader {
        bytes: [u8; 8],
    }

    impl MockReader {
        fn new(bytes: [u8; 8]) -> Self {
            Self { bytes }
        }
    }

    impl Reader for MockReader {
        fn get_slice(&self) -> &[u8] {
            &self.bytes
        }
    }

    mod get_slice_of_size {
        use super::*;

        #[test]
        fn should_return_a_slice_of_a_given_size() {
            let reader = MockReader::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let slice = reader
                .get_slice_of_size(4, 4)
                .expect("Read should have been successful.");

            let result = [5, 6, 7, 8];
            assert_eq!(slice, result);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let reader = MockReader::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let error = reader
                .get_slice_of_size(6, 4)
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 4,
                    available_size: 2
                }
            );
        }
    }

    mod get_sized_slice {
        use super::*;

        #[test]
        fn should_return_sized_slice() {
            let reader = MockReader::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let slice = reader
                .get_sized_slice::<u32>(4)
                .expect("Read should have been successful.");

            let result = [5, 6, 7, 8];
            assert_eq!(slice, result);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let reader = MockReader::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let error = reader
                .get_sized_slice::<u32>(6)
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 4,
                    available_size: 2
                }
            );
        }
    }

    mod get_transmutable {
        use super::*;

        #[test]
        fn should_return_a_reference() {
            let reader = MockReader::new(u64::to_ne_bytes(0x11223344aabbccdd));
            let slice = reader
                .get_sized_slice::<u32>(4)
                .expect("Read should have been successful.");

            let result = u32::to_ne_bytes(0x11223344);
            assert_eq!(slice, &result);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let reader = MockReader::new(u64::to_ne_bytes(0x11223344aabbccdd));
            let error = reader
                .get_sized_slice::<u32>(6)
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 4,
                    available_size: 2
                }
            );
        }
    }

    mod read {
        use super::*;

        #[test]
        fn should_return_a_value() {
            let reader = MockReader::new(u64::to_ne_bytes(0x1122334411223344));
            let value = reader
                .read::<u32>(4)
                .expect("Read should have been successful.");

            assert_eq!(value, 0x11223344);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let reader = MockReader::new(u64::to_ne_bytes(0x1122334411223344));
            let error = reader
                .read::<u32>(6)
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 4,
                    available_size: 2
                }
            );
        }

        #[test]
        fn should_return_error_if_alignment_is_invalid() {
            let reader = MockReader::new(u64::to_ne_bytes(0x1122334411223344));
            let error = reader
                .read::<u32>(3)
                .expect_err("Alignment should have been invalid");

            assert_eq!(
                error,
                Error::InvalidAlignment {
                    wanted_size: 4,
                    source_size: 4,
                    source_offset: 3,
                }
            );
        }
    }

    mod default_read {
        use super::*;

        #[test]
        fn should_return_a_value() {
            let reader = MockReader::new(u64::to_ne_bytes(0x11223344aabbccdd));
            let value = reader.default_read::<u32>(4);
            assert_eq!(value, 0x11223344);
        }

        #[test]
        fn should_return_default_if_size_is_too_large_for_offset() {
            let reader = MockReader::new(u64::to_ne_bytes(0x11223344aabbccdd));
            let value = reader.default_read::<u32>(6);
            assert_eq!(value, u32::default());
        }

        #[test]
        fn should_return_default_if_alignment_is_invalid() {
            let reader = MockReader::new(u64::to_ne_bytes(0x11223344aabbccdd));
            let value = reader.default_read::<u32>(3);
            assert_eq!(value, u32::default());
        }
    }

    mod read_le {
        use super::*;

        #[test]
        fn should_return_a_value() {
            let reader = MockReader::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            let value = reader
                .read_le::<u32>(4)
                .expect("Read should have been successful.");

            assert_eq!(value, 0xddccbbaa);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let reader = MockReader::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            let error = reader
                .read_le::<u32>(6)
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 4,
                    available_size: 2
                }
            );
        }
    }

    mod default_read_le {
        use super::*;

        #[test]
        fn should_return_a_value() {
            let reader = MockReader::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            let value = reader.default_read_le::<u32>(4);
            assert_eq!(value, 0xddccbbaa);
        }

        #[test]
        fn should_return_default_if_size_is_too_large_for_offset() {
            let reader = MockReader::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            let value = reader.default_read_le::<u32>(6);
            assert_eq!(value, u32::default());
        }
    }

    mod read_be {
        use super::*;

        #[test]
        fn should_return_a_value() {
            let reader = MockReader::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            let value = reader
                .read_be::<u32>(4)
                .expect("Read should have been successful.");

            assert_eq!(value, 0xaabbccdd);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let reader = MockReader::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            let error = reader
                .read_be::<u32>(6)
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 4,
                    available_size: 2
                }
            );
        }
    }

    mod default_read_be {
        use super::*;

        #[test]
        fn should_return_a_value() {
            let reader = MockReader::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            let value = reader.default_read_be::<u32>(4);
            assert_eq!(value, 0xaabbccdd);
        }

        #[test]
        fn should_return_default_if_size_is_too_large_for_offset() {
            let reader = MockReader::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            let value = reader.default_read_be::<u32>(6);
            assert_eq!(value, u32::default());
        }
    }

    mod read_byte_vec {
        use super::*;

        #[test]
        fn should_return_a_value() {
            let reader = MockReader::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            let value = reader
                .read_byte_vec(4, 3)
                .expect("Read should have been successful.");

            assert_eq!(value, vec![0xaa, 0xbb, 0xcc]);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let reader = MockReader::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            let error = reader
                .read_byte_vec(6, 4)
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 4,
                    available_size: 2
                }
            );
        }
    }

    mod default_read_byte_vec {
        use super::*;

        #[test]
        fn should_return_a_value() {
            let reader = MockReader::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            let value = reader.default_read_byte_vec(4, 3);
            assert_eq!(value, vec![0xaa, 0xbb, 0xcc]);
        }

        #[test]
        fn should_return_default_if_size_is_too_large_for_offset() {
            let reader = MockReader::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            let value = reader.default_read_byte_vec(6, 4);
            assert_eq!(value, vec![0, 0, 0, 0]);
        }
    }
}
