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
                available_size: data.len() - offset,
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::Reader;

    pub struct MockWriter {
        bytes: [u8; 8],
    }

    impl MockWriter {
        fn new(bytes: [u8; 8]) -> Self {
            Self { bytes }
        }

        fn get_bytes(&self) -> [u8; 8] {
            self.bytes.clone()
        }
    }

    impl Writer for MockWriter {
        fn get_mut_slice(&mut self) -> &mut [u8] {
            &mut self.bytes
        }
    }

    impl Reader for MockWriter {
        fn get_slice(&self) -> &[u8] {
            &self.bytes
        }
    }

    mod get_sized_mut_slice {
        use super::*;

        #[test]
        fn should_return_mut_slice() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let slice = writer
                .get_sized_mut_slice(2, 4)
                .expect("Should have succeeded");

            let expected_result = [3, 4, 5, 6];
            assert_eq!(slice, expected_result);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let error = writer
                .get_sized_mut_slice(2, 100)
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 100,
                    available_size: 6
                }
            );
        }
    }

    mod get_type_sized_mut_slice {
        use super::*;

        #[test]
        fn should_return_mut_slice() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let slice = writer
                .get_type_sized_mut_slice::<u32>(2)
                .expect("Should have succeeded");

            let expected_result = [3, 4, 5, 6];
            assert_eq!(slice, expected_result);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let error = writer
                .get_type_sized_mut_slice::<u32>(6)
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

    mod write_bytes {
        use super::*;

        #[test]
        fn should_write_bytes() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let bytes = [0xaa, 0xbb, 0xcc, 0xdd];
            let written_length = writer
                .write_bytes(2, &bytes)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

            let inner = writer.get_bytes();
            assert_eq!(inner, [1, 2, 0xaa, 0xbb, 0xcc, 0xdd, 7, 8]);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let bytes = [0xaa, 0xbb, 0xcc, 0xdd];
            let error = writer
                .write_bytes(6, &bytes)
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

    mod write {
        use super::*;

        #[test]
        fn should_write_value() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            let written_length = writer
                .write(4, &value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

            let result = writer.read::<u32>(4).expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            let error = writer
                .write(6, &value)
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

    mod write_le {
        use super::*;

        #[test]
        fn should_write_value() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            writer
                .write_le(2, &value)
                .expect("Write should have succeeded");

            let result = writer
                .read_le::<u32>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            let error = writer
                .write_le(6, &value)
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

    mod write_be {
        use super::*;

        #[test]
        fn should_write_value() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            writer
                .write_be(2, &value)
                .expect("Write should have succeeded");

            let result = writer
                .read_be::<u32>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            let error = writer
                .write_be(6, &value)
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
}
