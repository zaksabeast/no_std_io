use super::cursor::Cursor;
use crate::{EndianWrite, Writer, WriterResult};
use safe_transmute::TriviallyTransmutable;

/// An interface to write values as a stream.
pub trait StreamWriter: Writer + Cursor {
    /// Same as [Writer::write], but uses the current stream instead of an offset.
    #[inline(always)]
    fn write_stream<T: TriviallyTransmutable>(&mut self, value: &T) -> WriterResult<usize> {
        let index = self.swap_incremented_index_for_type::<T>();
        self.write(index, value)
    }

    /// Same as [StreamWriter::write_stream], but does not write if there is not enough space.
    #[inline(always)]
    fn checked_write_stream<T: TriviallyTransmutable>(&mut self, value: &T) -> usize {
        let index = self.swap_incremented_index_for_type::<T>();
        self.checked_write(index, value)
    }

    /// Same as [Writer::write_le], but uses the current stream instead of an offset.
    #[inline(always)]
    fn write_stream_le<T: EndianWrite>(&mut self, value: &T) -> WriterResult<usize> {
        let index = self.get_index();
        let bytes_written = self.write_le(index, value)?;
        self.increment_by(bytes_written);
        Ok(bytes_written)
    }

    /// Same as [StreamWriter::write_stream_le], but does not write if there is not enough space.
    #[inline(always)]
    fn checked_write_stream_le<T: EndianWrite>(&mut self, value: &T) -> usize {
        let index = self.swap_incremented_index_for_type::<T>();
        self.checked_write_le(index, value)
    }

    /// Same as [Writer::write_array_le], but uses the current stream instead of an offset.
    #[inline(always)]
    fn write_array_stream_le<const SIZE: usize, T: EndianWrite>(
        &mut self,
        value: &[T; SIZE],
    ) -> WriterResult<usize> {
        let index = self.get_index();
        let mut write_size = 0;

        for val in value {
            let bytes_written = self.write_le(index + write_size, val)?;
            self.increment_by(bytes_written);
            write_size += bytes_written;
        }

        Ok(write_size)
    }

    /// Same as [StreamWriter::write_stream_le], but does not write if there is not enough space.
    #[inline(always)]
    fn checked_write_array_stream_le<const SIZE: usize, T: EndianWrite>(
        &mut self,
        value: &[T; SIZE],
    ) -> usize {
        let index = self.get_index();
        if value.is_empty() {
            return 0;
        }

        let size = value[0].get_size() * SIZE;
        let len = self.get_mut_slice().len();
        if index + size > len {
            return 0;
        }

        self.write_array_stream_le(value).unwrap_or(0)
    }

    /// Same as [Writer::write_be], but uses the current stream instead of an offset.
    #[inline(always)]
    fn write_stream_be<T: EndianWrite>(&mut self, value: &T) -> WriterResult<usize> {
        let index = self.get_index();
        let bytes_written = self.write_be(index, value)?;
        self.increment_by(bytes_written);
        Ok(bytes_written)
    }

    /// Same as [StreamWriter::write_stream_be], but does not write if there is not enough space.
    #[inline(always)]
    fn checked_write_stream_be<T: EndianWrite>(&mut self, value: &T) -> usize {
        let index = self.swap_incremented_index_for_type::<T>();
        self.checked_write_be(index, value)
    }

    /// Same as [Writer::write_array_be], but uses the current stream instead of an offset.
    #[inline(always)]
    fn write_array_stream_be<const SIZE: usize, T: EndianWrite>(
        &mut self,
        value: &[T; SIZE],
    ) -> WriterResult<usize> {
        let index = self.get_index();
        let mut write_size = 0;

        for val in value {
            self.write_be(index + write_size, val)?;
            self.increment_by(val.get_size());
            write_size += val.get_size();
        }

        Ok(write_size)
    }

    /// Same as [StreamWriter::write_stream_be], but does not write if there is not enough space.
    #[inline(always)]
    fn checked_write_array_stream_be<const SIZE: usize, T: EndianWrite>(
        &mut self,
        value: &[T; SIZE],
    ) -> usize {
        let index = self.get_index();
        if value.is_empty() {
            return 0;
        }

        let size = value[0].get_size() * SIZE;
        let len = self.get_mut_slice().len();
        if index + size > len {
            return 0;
        }

        self.write_array_stream_be(value).unwrap_or(0)
    }

    /// Same as [Writer::write_bytes], but uses the current stream instead of an offset.
    #[inline(always)]
    fn write_stream_bytes(&mut self, bytes: &[u8]) -> WriterResult<usize> {
        let index = self.swap_incremented_index(bytes.len());
        self.write_bytes(index, bytes)
    }

    /// Same as [Writer::checked_write_bytes], but does not write if there is not enough space.
    #[inline(always)]
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

        #[derive(Debug, PartialEq)]
        struct Repeat(u8);

        impl EndianWrite for Repeat {
            fn get_size(&self) -> usize {
                3
            }

            fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error> {
                let bytes: [u8; 3] = [self.0, self.0, self.0];
                dst[0..3].copy_from_slice(&bytes);
                Ok(bytes.len())
            }

            fn try_write_be(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                unimplemented!()
            }
        }

        #[test]
        fn should_write_values_with_dynamic_read_lengths() {
            let mut writer = MockStream::new([0x11, 0x22, 0xaa, 0xbb, 0x88, 0x99, 0x01, 0x02]);
            let written_bytes = writer
                .write_stream_le(&Repeat(0x50))
                .expect("Should have been written successfully");
            assert_eq!(written_bytes, 3);

            let result = writer.get_bytes();
            let expected = [0x50, 0x50, 0x50, 0xbb, 0x88, 0x99, 0x01, 0x02];
            assert_eq!(result, expected);
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

    mod write_array_stream_le {
        use super::*;
        use crate::Error;

        #[test]
        fn should_write_value() {
            let mut writer = MockStream::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = [0x1122u16, 0x3344];
            writer.set_index(2);
            let written_length = writer
                .write_array_stream_le(&value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

            let result = writer
                .read_array_le::<2, u16>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, [0x1122u16, 0x3344]);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut writer = MockStream::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = [0x1122u16, 0x3344];
            writer.set_index(6);
            let error = writer
                .write_array_stream_le(&value)
                .expect_err("Length should have been too large");

            assert_eq!(
                error,
                Error::InvalidSize {
                    wanted_size: 2,
                    offset: 8,
                    data_len: 8,
                }
            );
        }

        #[derive(Debug, PartialEq)]
        struct Repeat(u8);

        impl EndianWrite for Repeat {
            fn get_size(&self) -> usize {
                3
            }

            fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error> {
                let bytes: [u8; 3] = [self.0, self.0, self.0];
                dst[0..3].copy_from_slice(&bytes);
                Ok(bytes.len())
            }

            fn try_write_be(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                unimplemented!()
            }
        }

        #[test]
        fn should_write_values_with_dynamic_read_lengths() {
            let mut writer = MockStream::new([0x11, 0x22, 0xaa, 0xbb, 0x88, 0x99, 0x01, 0x02]);
            let written_bytes = writer
                .write_array_stream_le(&[Repeat(0x50), Repeat(0x50)])
                .expect("Should have been written successfully");
            assert_eq!(written_bytes, 6);

            let result = writer.get_bytes();
            let expected = [0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x01, 0x02];
            assert_eq!(result, expected);
        }
    }

    mod checked_write_array_stream_le {
        use super::*;

        #[test]
        fn should_write_value() {
            let mut writer = MockStream::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = [0x1122u16, 0x3344];
            writer.set_index(2);
            let written_length = writer.checked_write_array_stream_le(&value);

            assert_eq!(written_length, 4);

            let result = writer
                .read_array_le::<2, u16>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, [0x1122u16, 0x3344]);
        }

        #[test]
        fn should_return_0_if_size_is_too_large_for_offset() {
            let bytes = [1, 2, 3, 4, 5, 6, 7, 8];
            let mut writer = MockStream::new(bytes.clone());
            let value = [0x1122u16, 0x3344];
            writer.set_index(6);
            let written_length = writer.checked_write_array_stream_le(&value);

            assert_eq!(written_length, 0);
            assert_eq!(writer.get_bytes(), bytes);
        }
    }
}
