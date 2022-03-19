use super::cursor::Cursor;
use crate::{EndianRead, Reader, ReaderResult};
use alloc::vec::Vec;
use safe_transmute::TriviallyTransmutable;

/// An interface to read values as a stream.
pub trait StreamReader: Reader + Cursor {
    /// Same as [Reader::read], but uses the current stream instead of an offset.
    fn read_stream<T: TriviallyTransmutable + Default>(&mut self) -> ReaderResult<T> {
        let index = self.swap_incremented_index_for_type::<T>();
        self.read(index)
    }

    /// Same as [StreamReader::read_stream], but returns a default value if the read is invalid.
    fn default_read_stream<T: TriviallyTransmutable + Default>(&mut self) -> T {
        let index = self.swap_incremented_index_for_type::<T>();
        self.default_read(index)
    }

    /// Same as [Reader::read_le], but uses the current stream instead of an offset.
    fn read_stream_le<T: EndianRead + Default>(&mut self) -> ReaderResult<T> {
        let index = self.swap_incremented_index_for_type::<T>();
        self.read_le(index)
    }

    /// Same as [StreamReader::read_stream_le], but returns a default value if the read is invalid.
    fn default_read_stream_le<T: EndianRead + Default>(&mut self) -> T {
        let index = self.swap_incremented_index_for_type::<T>();
        self.default_read_le(index)
    }

    /// Same as [Reader::read_byte_vec], but uses the current stream instead of an offset.
    fn read_byte_stream(&mut self, size: usize) -> ReaderResult<Vec<u8>> {
        let index = self.swap_incremented_index(size);
        self.read_byte_vec(index, size)
    }

    /// Same as [StreamReader::default_read_byte_vec], but returns a default value if the read is invalid.
    fn default_read_byte_stream(&mut self, size: usize) -> Vec<u8> {
        let index = self.swap_incremented_index(size);
        self.default_read_byte_vec(index, size)
    }
}

impl<T> StreamReader for T where T: Reader + Cursor {}

#[cfg(test)]
mod test {
    use super::*;

    pub struct MockStream {
        bytes: [u8; 8],
        index: usize,
    }

    impl MockStream {
        fn new(bytes: [u8; 8]) -> Self {
            Self { bytes, index: 0 }
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

    mod read_stream {
        use super::*;
        use crate::Error;

        #[test]
        fn should_return_a_value() {
            let mut reader = MockStream::new(u64::to_ne_bytes(0x1122334411223344));
            let value = reader
                .read_stream::<u32>()
                .expect("Read should have been successful.");

            assert_eq!(value, 0x11223344);
            assert_eq!(reader.get_index(), 4);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut reader = MockStream::new(u64::to_ne_bytes(0x1122334411223344));
            reader.set_index(8);
            let error = reader
                .read_stream::<u32>()
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 4,
                    available_size: 0
                }
            );
        }

        #[test]
        fn should_return_error_if_alignment_is_invalid() {
            let mut reader = MockStream::new(u64::to_ne_bytes(0x1122334411223344));
            reader.set_index(3);
            let error = reader
                .read_stream::<u32>()
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

    mod default_read_stream {
        use super::*;

        #[test]
        fn should_return_a_value() {
            let mut reader = MockStream::new(u64::to_ne_bytes(0x11223344aabbccdd));
            reader.set_index(4);
            let value = reader.default_read_stream::<u32>();
            assert_eq!(value, 0x11223344);
        }

        #[test]
        fn should_return_default_if_size_is_too_large_for_offset() {
            let mut reader = MockStream::new(u64::to_ne_bytes(0x11223344aabbccdd));
            reader.set_index(8);
            let value = reader.default_read_stream::<u32>();
            assert_eq!(value, u32::default());
        }

        #[test]
        fn should_return_default_if_alignment_is_invalid() {
            let mut reader = MockStream::new(u64::to_ne_bytes(0x11223344aabbccdd));
            reader.set_index(2);
            let value = reader.default_read_stream::<u32>();
            assert_eq!(value, u32::default());
        }
    }

    mod read_stream_le {
        use super::*;
        use crate::Error;

        #[test]
        fn should_return_a_value() {
            let mut reader = MockStream::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            reader.set_index(4);
            let value = reader
                .read_stream_le::<u32>()
                .expect("Read should have been successful.");

            assert_eq!(value, 0xddccbbaa);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut reader = MockStream::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            reader.set_index(8);
            let error = reader
                .read_stream_le::<u32>()
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 4,
                    available_size: 0
                }
            );
        }
    }

    mod default_read_stream_le {
        use super::*;

        #[test]
        fn should_return_a_value() {
            let mut reader = MockStream::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            reader.set_index(4);
            let value = reader.default_read_stream_le::<u32>();
            assert_eq!(value, 0xddccbbaa);
        }

        #[test]
        fn should_return_default_if_size_is_too_large_for_offset() {
            let mut reader = MockStream::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            reader.set_index(6);
            let value = reader.default_read_stream_le::<u32>();
            assert_eq!(value, u32::default());
        }
    }

    mod read_byte_stream {
        use super::*;
        use crate::Error;
        use alloc::vec;

        #[test]
        fn should_return_a_value() {
            let mut reader = MockStream::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            reader.set_index(4);
            let value = reader
                .read_byte_stream(3)
                .expect("Read should have been successful.");

            assert_eq!(value, vec![0xaa, 0xbb, 0xcc]);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut reader = MockStream::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            reader.set_index(6);
            let error = reader
                .read_byte_stream(4)
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

    mod default_read_byte_stream {
        use super::*;
        use alloc::vec;

        #[test]
        fn should_return_a_value() {
            let mut reader = MockStream::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            reader.set_index(4);
            let value = reader.default_read_byte_stream(3);
            assert_eq!(value, vec![0xaa, 0xbb, 0xcc]);
        }

        #[test]
        fn should_return_default_if_size_is_too_large_for_offset() {
            let mut reader = MockStream::new([0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd]);
            reader.set_index(6);
            let value = reader.default_read_byte_stream(4);
            assert_eq!(value, vec![0, 0, 0, 0]);
        }
    }
}