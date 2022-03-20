use super::cursor::Cursor;
use crate::{EndianWrite, Writer, WriterResult};
use safe_transmute::TriviallyTransmutable;

/// An interface to write values as a stream.
pub trait StreamWriter: Writer + Cursor {
    /// Same as [Writer::write], but uses the current stream instead of an offset.
    fn write_stream<T: TriviallyTransmutable + Default>(
        &mut self,
        value: &T,
    ) -> WriterResult<usize> {
        let index = self.swap_incremented_index_for_type::<T>();
        self.write(index, value)
    }

    /// Same as [StreamWriter::write_stream], but does not write if there is not enough space.
    fn checked_write_stream<T: TriviallyTransmutable + Default>(&mut self, value: &T) -> usize {
        let index = self.swap_incremented_index_for_type::<T>();
        self.checked_write(index, value)
    }

    /// Same as [Writer::write_le], but uses the current stream instead of an offset.
    fn write_stream_le<T: EndianWrite + Default>(&mut self, value: &T) -> WriterResult<usize> {
        let index = self.swap_incremented_index_for_type::<T>();
        self.write_le(index, value)
    }

    /// Same as [StreamWriter::write_stream_le], but does not write if there is not enough space.
    fn checked_write_stream_le<T: EndianWrite + Default>(&mut self, value: &T) -> usize {
        let index = self.swap_incremented_index_for_type::<T>();
        self.checked_write_le(index, value)
    }

    /// Same as [Writer::write_bytes], but uses the current stream instead of an offset.
    fn write_stream_bytes(&mut self, bytes: &[u8]) -> WriterResult<usize> {
        let index = self.swap_incremented_index(bytes.len());
        self.write_bytes(index, bytes)
    }

    /// Same as [Writer::checked_write_bytes], but does not write if there is not enough space.
    fn checked_write_stream_bytes(&mut self, bytes: &[u8]) -> usize {
        let index = self.swap_incremented_index(bytes.len());
        self.checked_write_bytes(index, bytes)
    }
}

impl<T> StreamWriter for T where T: Writer + Cursor {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Reader;

    pub struct MockStream {
        bytes: [u8; 8],
        index: usize,
    }

    impl MockStream {
        fn new(bytes: [u8; 8]) -> Self {
            Self { bytes, index: 0 }
        }

        fn get_bytes(&self) -> [u8; 8] {
            self.bytes.clone()
        }
    }

    impl Writer for MockStream {
        fn get_mut_slice(&mut self) -> &mut [u8] {
            &mut self.bytes
        }
    }

    impl Reader for MockStream {
        fn get_slice(&self) -> &[u8] {
            &self.bytes
        }
    }

    impl Cursor for MockStream {
        fn get_index(&self) -> usize {
            self.index
        }

        fn set_index(&mut self, index: usize) {
            self.index = index;
        }
    }

    mod write_stream_bytes {
        use super::*;
        use crate::Error;

        #[test]
        fn should_write_bytes() {
            let mut writer = MockStream::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let bytes = [0xaa, 0xbb, 0xcc, 0xdd];
            writer.set_index(2);
            let written_length = writer
                .write_stream_bytes(&bytes)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

            let inner = writer.get_bytes();
            assert_eq!(inner, [1, 2, 0xaa, 0xbb, 0xcc, 0xdd, 7, 8]);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut writer = MockStream::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let bytes = [0xaa, 0xbb, 0xcc, 0xdd];
            writer.set_index(6);
            let error = writer
                .write_stream_bytes(&bytes)
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 4,
                    offset: 6,
                    data_len: 8,
                }
            );
        }
    }

    mod checked_write_stream_bytes {
        use super::*;

        #[test]
        fn should_write_bytes() {
            let mut writer = MockStream::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let bytes = [0xaa, 0xbb, 0xcc, 0xdd];
            writer.set_index(2);
            let written_length = writer.checked_write_stream_bytes(&bytes);

            assert_eq!(written_length, 4);

            let inner = writer.get_bytes();
            assert_eq!(inner, [1, 2, 0xaa, 0xbb, 0xcc, 0xdd, 7, 8]);
        }

        #[test]
        fn should_return_0_if_size_is_too_large_for_offset() {
            let initial_bytes = [1, 2, 3, 4, 5, 6, 7, 8];
            let mut writer = MockStream::new(initial_bytes.clone());
            let bytes_to_write = [0xaa, 0xbb, 0xcc, 0xdd];
            writer.set_index(6);
            let written_length = writer.checked_write_stream_bytes(&bytes_to_write);

            assert_eq!(written_length, 0);
            assert_eq!(writer.get_bytes(), initial_bytes);
        }
    }

    mod write_stream {
        use super::*;
        use crate::Error;

        #[test]
        fn should_write_value() {
            let mut writer = MockStream::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            writer.set_index(4);
            let written_length = writer
                .write_stream(&value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

            let result = writer.read::<u32>(4).expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut writer = MockStream::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            writer.set_index(6);
            let error = writer
                .write_stream(&value)
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 4,
                    offset: 6,
                    data_len: 8,
                }
            );
        }
    }

    mod checked_write {
        use super::*;

        #[test]
        fn should_write_value() {
            let mut writer = MockStream::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            let written_length = writer.checked_write(4, &value);

            assert_eq!(written_length, 4);

            let result = writer.read::<u32>(4).expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
        }

        #[test]
        fn should_return_0_if_size_is_too_large_for_offset() {
            let bytes = [1, 2, 3, 4, 5, 6, 7, 8];
            let mut writer = MockStream::new(bytes.clone());
            let value = 0xaabbccddu32;
            let written_length = writer.checked_write(6, &value);

            assert_eq!(written_length, 0);
            assert_eq!(writer.get_bytes(), bytes);
        }
    }

    mod write_stream_le {
        use super::*;
        use crate::Error;

        #[test]
        fn should_write_value() {
            let mut writer = MockStream::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            writer.set_index(2);
            let written_length = writer
                .write_stream_le(&value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

            let result = writer
                .read_le::<u32>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut writer = MockStream::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            writer.set_index(6);
            let error = writer
                .write_stream_le(&value)
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 4,
                    offset: 6,
                    data_len: 8,
                }
            );
        }
    }

    mod checked_write_stream_le {
        use super::*;

        #[test]
        fn should_write_value() {
            let mut writer = MockStream::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            writer.set_index(2);
            let written_length = writer.checked_write_stream_le(&value);

            assert_eq!(written_length, 4);

            let result = writer
                .read_le::<u32>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
        }

        #[test]
        fn should_return_0_if_size_is_too_large_for_offset() {
            let bytes = [1, 2, 3, 4, 5, 6, 7, 8];
            let mut writer = MockStream::new(bytes.clone());
            let value = 0xaabbccddu32;
            writer.set_index(6);
            let written_length = writer.checked_write_stream_le(&value);

            assert_eq!(written_length, 0);
            assert_eq!(writer.get_bytes(), bytes);
        }
    }
}
