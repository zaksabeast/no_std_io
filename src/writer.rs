#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use super::{add_error_context, EndianWrite, Error};
use core::mem;
use safe_transmute::{transmute_one_to_bytes, TriviallyTransmutable};

pub type WriterResult<T> = Result<T, Error>;

/// An interface to safely write values to a source.
///
/// Blanket implementations are provided for byte slices and vectors.
/// Vectors will grow if there isn't enough space.  If this isn't desirable, use a slice from a vector as the writer.
///
/// To forward [Writer] methods to containers with vectors, implement both
/// [Writer::get_mut_slice] and [Writer::get_sized_mut_slice] instead of only [Writer::get_mut_slice].
pub trait Writer {
    /// Returns the data to be read from.
    fn get_mut_slice(&mut self) -> &mut [u8];

    /// Returns a slice from the given offset.
    /// Returns an empty slice if the offset is greater than the slice size.
    #[inline(always)]
    fn get_mut_slice_at_offset(&mut self, offset: usize) -> &mut [u8] {
        let data = self.get_mut_slice();

        if offset >= data.len() {
            return &mut [];
        }

        &mut data[offset..]
    }

    /// Gets a slice of bytes with a specified length from an offset of a source.
    ///
    /// An error should be returned if the size is invalid.
    #[inline(always)]
    fn get_sized_mut_slice(&mut self, offset: usize, length: usize) -> WriterResult<&mut [u8]> {
        let data = self.get_mut_slice();
        let offset_end = offset + length;

        if data.len() < offset_end {
            return Err(Error::InvalidSize {
                wanted_size: length,
                data_len: data.len(),
                offset,
            });
        }

        Ok(&mut data[offset..offset_end])
    }

    /// Same as [Writer::get_sized_mut_slice], except the length comes from `T.len()`.
    #[inline(always)]
    fn get_type_sized_mut_slice<T: Sized>(&mut self, offset: usize) -> WriterResult<&mut [u8]> {
        let length = mem::size_of::<T>();
        self.get_sized_mut_slice(offset, length)
    }

    /// Writes bytes to an offset and returns the number of bytes written.
    ///
    /// Errors if the byte slice length will not fit at the offset.
    #[inline(always)]
    fn write_bytes(&mut self, offset: usize, bytes: &[u8]) -> WriterResult<usize> {
        let length = bytes.len();
        let slice = self.get_sized_mut_slice(offset, length)?;

        slice.copy_from_slice(bytes);
        Ok(length)
    }

    /// Same as [Writer::write_bytes], but checks to make sure the bytes can safely be written to the offset.
    /// Returns 0 as the write size if the bytes won't fit into the offset.
    #[inline(always)]
    fn checked_write_bytes(&mut self, offset: usize, bytes: &[u8]) -> usize {
        self.write_bytes(offset, bytes).unwrap_or(0)
    }

    /// Same as [Writer::write_bytes], but writes a [TriviallyTransmutable] type by converting it to bytes.
    #[inline(always)]
    fn write<T: TriviallyTransmutable>(&mut self, offset: usize, value: &T) -> WriterResult<usize> {
        let bytes = transmute_one_to_bytes(value);
        self.write_bytes(offset, bytes)
    }

    /// Same as [Writer::checked_write_bytes], but writes a [TriviallyTransmutable] type by converting it to bytes.
    #[inline(always)]
    fn checked_write<T: TriviallyTransmutable>(&mut self, offset: usize, value: &T) -> usize {
        self.write(offset, value).unwrap_or(0)
    }

    /// Writes a value in its little endian representation.
    ///
    /// Prefer endian agnostic methods when possible.
    /// This should only be used when reading data from a format or protocol
    /// that explicitly defines little endian.
    #[inline(always)]
    fn write_le<T: EndianWrite>(&mut self, offset: usize, value: &T) -> WriterResult<usize> {
        let bytes = self.get_mut_slice_at_offset(offset);
        add_error_context(
            value.try_write_le(bytes),
            offset,
            self.get_mut_slice().len(),
        )
    }

    /// Same as [Writer::write_le], but checks to make sure the bytes can safely be written to the offset.
    /// Returns 0 as the write size if the bytes won't fit into the offset.
    #[inline(always)]
    fn checked_write_le<T: EndianWrite>(&mut self, offset: usize, value: &T) -> usize {
        self.write_le(offset, value).unwrap_or(0)
    }

    /// Writes a value in its big endian representation.
    ///
    /// Prefer endian agnostic methods when possible.
    /// This should only be used when reading data from a format or protocol
    /// that explicitly defines big endian.
    #[inline(always)]
    fn write_be<T: EndianWrite>(&mut self, offset: usize, value: &T) -> WriterResult<usize> {
        let bytes = self.get_mut_slice_at_offset(offset);
        add_error_context(
            value.try_write_be(bytes),
            offset,
            self.get_mut_slice().len(),
        )
    }

    /// Same as [Writer::write_be], but checks to make sure the bytes can safely be written to the offset.
    /// Returns 0 as the write size if the bytes won't fit into the offset.
    #[inline(always)]
    fn checked_write_be<T: EndianWrite>(&mut self, offset: usize, value: &T) -> usize {
        self.write_be(offset, value).unwrap_or(0)
    }

    /// Writes an array in its little endian representation.
    ///
    /// The array will be written fully or until an error is encountered. The error will contain
    /// the offset at which the error was encountered while writing.
    ///
    /// This should only be used when reading data from a format or protocol
    /// that explicitly defines little endian.
    #[inline(always)]
    fn write_array_le<const SIZE: usize, T: EndianWrite>(
        &mut self,
        offset: usize,
        value: &[T; SIZE],
    ) -> WriterResult<usize> {
        let mut write_size = 0;

        for val in value {
            self.write_le(offset + write_size, val)?;
            write_size += val.get_size();
        }

        Ok(write_size)
    }

    /// Same as [Writer::write_array_le], but checks to make sure the bytes can safely be written to the offset.
    /// Returns 0 as the write size if the bytes won't fit into the offset.
    #[inline(always)]
    fn checked_write_array_le<const SIZE: usize, T: EndianWrite>(
        &mut self,
        offset: usize,
        value: &[T; SIZE],
    ) -> usize {
        if value.is_empty() {
            return 0;
        }

        let size = value.iter().map(|val| val.get_size()).sum::<usize>();
        let len = self.get_mut_slice().len();
        if offset + size > len {
            return 0;
        }

        self.write_array_le(offset, value).unwrap_or(0)
    }

    /// Writes an array in its big endian representation.
    ///
    /// The array will be written fully or until an error is encountered. The error will contain
    /// the offset at which the error was encountered while writing.
    ///
    /// This should only be used when reading data from a format or protocol
    /// that explicitly defines big endian.
    #[inline(always)]
    fn write_array_be<const SIZE: usize, T: EndianWrite>(
        &mut self,
        offset: usize,
        value: &[T; SIZE],
    ) -> WriterResult<usize> {
        let mut write_size = 0;

        for val in value {
            self.write_be(offset + write_size, val)?;
            write_size += val.get_size();
        }

        Ok(write_size)
    }

    /// Same as [Writer::write_array_be], but checks to make sure the bytes can safely be written to the offset.
    /// Returns 0 as the write size if the bytes won't fit into the offset.
    #[inline(always)]
    fn checked_write_array_be<const SIZE: usize, T: EndianWrite>(
        &mut self,
        offset: usize,
        value: &[T; SIZE],
    ) -> usize {
        if value.is_empty() {
            return 0;
        }

        let size = value.iter().map(|val| val.get_size()).sum::<usize>();
        let len = self.get_mut_slice().len();
        if offset + size > len {
            return 0;
        }

        self.write_array_be(offset, value).unwrap_or(0)
    }
}

impl<const SIZE: usize> Writer for [u8; SIZE] {
    #[inline(always)]
    fn get_mut_slice(&mut self) -> &mut [u8] {
        self
    }
}

impl Writer for &mut [u8] {
    #[inline(always)]
    fn get_mut_slice(&mut self) -> &mut [u8] {
        self
    }
}

#[cfg(feature = "alloc")]
impl Writer for Vec<u8> {
    #[inline(always)]
    fn get_mut_slice(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }

    #[inline(always)]
    fn write_le<T: EndianWrite>(&mut self, offset: usize, value: &T) -> WriterResult<usize> {
        let offset_end = offset + value.get_size();
        let self_len = self.len();

        if offset_end > self_len {
            self.resize(offset_end, 0);
        }

        add_error_context(
            value.try_write_le(&mut self[offset..]),
            offset,
            self.get_mut_slice().len(),
        )
    }

    #[inline(always)]
    fn write_be<T: EndianWrite>(&mut self, offset: usize, value: &T) -> WriterResult<usize> {
        let offset_end = offset + value.get_size();
        let self_len = self.len();

        if offset_end > self_len {
            self.resize(offset_end, 0);
        }

        add_error_context(
            value.try_write_be(&mut self[offset..]),
            offset,
            self.get_mut_slice().len(),
        )
    }

    #[inline(always)]
    fn write_array_le<const SIZE: usize, T: EndianWrite>(
        &mut self,
        offset: usize,
        value: &[T; SIZE],
    ) -> WriterResult<usize> {
        if value.is_empty() {
            return Ok(0);
        }
        let offset_end = value.iter().map(|val| val.get_size()).sum::<usize>() + offset;
        let self_len = self.len();

        if offset_end > self_len {
            self.resize(offset_end, 0);
        }

        let mut write_size = 0;

        for val in value {
            self.write_le(offset + write_size, val)?;
            write_size += val.get_size();
        }

        Ok(write_size)
    }

    #[inline(always)]
    fn write_array_be<const SIZE: usize, T: EndianWrite>(
        &mut self,
        offset: usize,
        value: &[T; SIZE],
    ) -> WriterResult<usize> {
        if value.is_empty() {
            return Ok(0);
        }
        let offset_end = value.iter().map(|val| val.get_size()).sum::<usize>() + offset;
        let self_len = self.len();

        if offset_end > self_len {
            self.resize(offset_end, 0);
        }

        let mut write_size = 0;

        for val in value {
            self.write_be(offset + write_size, val)?;
            write_size += val.get_size();
        }

        Ok(write_size)
    }

    #[inline(always)]
    fn get_sized_mut_slice(&mut self, offset: usize, length: usize) -> WriterResult<&mut [u8]> {
        let offset_end = offset + length;
        let self_len = self.len();

        if offset_end > self_len {
            self.resize(offset_end, 0);
        }

        let slice = self.get_mut_slice();
        Ok(&mut slice[offset..offset_end])
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

        use alloc::{vec, vec::Vec};

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
                    offset: 2,
                    data_len: 8,
                }
            );
        }

        #[test]
        fn should_grow_a_vector_if_needed() {
            let mut writer: Vec<u8> = vec![];

            let result = writer
                .get_sized_mut_slice(2, 4)
                .expect("Should have succeeded");

            let expected_result = [0; 4];
            assert_eq!(result, expected_result);
            assert_eq!(writer.len(), 6);
        }

        #[test]
        fn should_not_grow_a_vector_if_not_needed() {
            let mut writer: Vec<u8> = vec![0; 4];

            let result = writer
                .get_sized_mut_slice(0, 4)
                .expect("Should have succeeded");

            let expected_result = [0; 4];
            assert_eq!(result, expected_result);
            assert_eq!(writer.len(), 4);
        }

        #[test]
        fn should_not_not_error_if_vector_size_is_larger_than_write_size() {
            let mut writer: Vec<u8> = vec![0; 10];

            let result = writer
                .get_sized_mut_slice(2, 4)
                .expect("Should have succeeded");

            let expected_result = [0; 4];
            assert_eq!(result, expected_result);
            assert_eq!(writer.len(), 10);
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
                    offset: 6,
                    data_len: 8,
                }
            );
        }
    }

    mod write_bytes {
        use super::*;
        use alloc::vec;

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
                    offset: 6,
                    data_len: 8,
                }
            );
        }

        #[test]
        fn should_grow_a_vector_if_needed() {
            let mut writer = vec![];
            let bytes = [0xaa, 0xbb, 0xcc, 0xdd];
            let written_length = writer
                .write_bytes(2, &bytes)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

            assert_eq!(writer, vec![0, 0, 0xaa, 0xbb, 0xcc, 0xdd]);
            assert_eq!(writer.len(), 6);
        }

        #[test]
        fn should_not_grow_a_vector_if_not_needed() {
            let mut writer = vec![0; 4];
            let bytes = [0xaa, 0xbb, 0xcc, 0xdd];
            let written_length = writer
                .write_bytes(0, &bytes)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

            assert_eq!(writer, vec![0xaa, 0xbb, 0xcc, 0xdd]);
            assert_eq!(writer.len(), 4);
        }

        #[test]
        fn should_grow_a_vector_if_needed_and_written_to_twice() {
            let mut writer = vec![];
            let bytes = [0xaa, 0xbb, 0xcc, 0xdd];
            let written_length = writer
                .write_bytes(0, &bytes)
                .expect("Write should have succeeded");
            assert_eq!(written_length, 4);

            let written_length = writer
                .write_bytes(4, &bytes)
                .expect("Write should have succeeded");
            assert_eq!(written_length, 4);

            assert_eq!(writer, vec![0xaa, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0xdd]);
            assert_eq!(writer.len(), 8);
        }

        #[test]
        fn should_grow_a_vector_if_needed_with_le() {
            let mut writer = vec![];
            let written_length = writer
                .write_le(0, &0xaabbccddu32)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);
            assert_eq!(writer, vec![0xdd, 0xcc, 0xbb, 0xaa]);
            assert_eq!(writer.len(), 4);
        }

        #[test]
        fn should_grow_a_vector_if_needed_with_be() {
            let mut writer = vec![];
            let written_length = writer
                .write_be(0, &0xaabbccddu32)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);
            assert_eq!(writer, vec![0xaa, 0xbb, 0xcc, 0xdd]);
            assert_eq!(writer.len(), 4);
        }
    }

    mod checked_write_bytes {
        use super::*;

        #[test]
        fn should_write_bytes() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let bytes = [0xaa, 0xbb, 0xcc, 0xdd];
            let written_length = writer.checked_write_bytes(2, &bytes);

            assert_eq!(written_length, 4);

            let inner = writer.get_bytes();
            assert_eq!(inner, [1, 2, 0xaa, 0xbb, 0xcc, 0xdd, 7, 8]);
        }

        #[test]
        fn should_return_0_if_size_is_too_large_for_offset() {
            let initial_bytes = [1, 2, 3, 4, 5, 6, 7, 8];
            let mut writer = MockWriter::new(initial_bytes.clone());
            let bytes_to_write = [0xaa, 0xbb, 0xcc, 0xdd];
            let written_length = writer.checked_write_bytes(6, &bytes_to_write);

            assert_eq!(written_length, 0);
            assert_eq!(writer.get_bytes(), initial_bytes);
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
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            let written_length = writer.checked_write(4, &value);

            assert_eq!(written_length, 4);

            let result = writer.read::<u32>(4).expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
        }

        #[test]
        fn should_return_0_if_size_is_too_large_for_offset() {
            let bytes = [1, 2, 3, 4, 5, 6, 7, 8];
            let mut writer = MockWriter::new(bytes.clone());
            let value = 0xaabbccddu32;
            let written_length = writer.checked_write(6, &value);

            assert_eq!(written_length, 0);
            assert_eq!(writer.get_bytes(), bytes);
        }
    }

    mod write_le {
        use super::*;
        use alloc::vec;

        #[test]
        fn should_write_value() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            let written_length = writer
                .write_le(2, &value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

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
                    offset: 6,
                    data_len: 8,
                }
            );
        }

        #[test]
        fn should_grow_a_vector_if_needed() {
            let mut writer = vec![];
            let value = 0xaabbccddu32;
            let written_length = writer
                .write_le(2, &value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

            let result = writer
                .read_le::<u32>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
            assert_eq!(writer.len(), 6);
        }

        #[test]
        fn should_not_grow_a_vector_if_not_needed() {
            let mut writer = vec![0; 4];
            let value = 0xaabbccddu32;
            let written_length = writer
                .write_le(0, &value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

            let result = writer
                .read_le::<u32>(0)
                .expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
            assert_eq!(writer.len(), 4);
        }

        #[derive(Debug)]
        struct CustomErrorTest(u32);

        impl EndianWrite for CustomErrorTest {
            fn get_size(&self) -> usize {
                0
            }
            fn try_write_le(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                Err(Error::InvalidRead {
                    message: "Custom error!",
                })
            }
            fn try_write_be(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                unimplemented!()
            }
        }

        #[test]
        fn should_bubble_up_custom_errors_for_vec() {
            let value = CustomErrorTest(0);
            let mut bytes = vec![];
            let result = bytes.write_le(0, &value).unwrap_err();
            let expected = Error::InvalidRead {
                message: "Custom error!",
            };
            assert_eq!(result, expected)
        }

        #[test]
        fn should_bubble_up_custom_errors_for_slice() {
            let value = CustomErrorTest(0);
            let bytes = &mut [];
            let result = bytes.write_le(0, &value).unwrap_err();
            let expected = Error::InvalidRead {
                message: "Custom error!",
            };
            assert_eq!(result, expected)
        }

        #[derive(Debug)]
        struct OffsetErrorTest(u32);

        impl EndianWrite for OffsetErrorTest {
            fn get_size(&self) -> usize {
                0
            }
            fn try_write_le(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                Err(Error::InvalidSize {
                    wanted_size: 8,
                    offset: 1,
                    data_len: 0,
                })
            }
            fn try_write_be(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                unimplemented!()
            }
        }

        #[test]
        fn should_bubble_up_error_offsets_for_vec() {
            let value = OffsetErrorTest(0);
            let mut bytes = vec![];
            let result = bytes.write_le(2, &value).unwrap_err();
            let expected = Error::InvalidSize {
                wanted_size: 8,
                offset: 3,
                data_len: 2,
            };
            assert_eq!(result, expected)
        }

        #[test]
        fn should_bubble_up_error_offsets_for_slice() {
            let value = OffsetErrorTest(0);
            let bytes = &mut [];
            let result = bytes.write_le(2, &value).unwrap_err();
            let expected = Error::InvalidSize {
                wanted_size: 8,
                offset: 3,
                data_len: 0,
            };
            assert_eq!(result, expected)
        }
    }

    mod checked_write_le {
        use super::*;

        #[test]
        fn should_write_value() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            let written_length = writer.checked_write_le(2, &value);

            assert_eq!(written_length, 4);

            let result = writer
                .read_le::<u32>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
        }

        #[test]
        fn should_return_0_if_size_is_too_large_for_offset() {
            let bytes = [1, 2, 3, 4, 5, 6, 7, 8];
            let mut writer = MockWriter::new(bytes.clone());
            let value = 0xaabbccddu32;
            let written_length = writer.checked_write_le(6, &value);

            assert_eq!(written_length, 0);
            assert_eq!(writer.get_bytes(), bytes);
        }
    }

    mod write_be {
        use super::*;
        use alloc::vec;

        #[test]
        fn should_write_value() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            let written_length = writer
                .write_be(2, &value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

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
                    offset: 6,
                    data_len: 8,
                }
            );
        }

        #[test]
        fn should_grow_a_vector_if_needed() {
            let mut writer = vec![];
            let value = 0xaabbccddu32;
            let written_length = writer
                .write_be(2, &value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

            let result = writer
                .read_be::<u32>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
            assert_eq!(writer.len(), 6);
        }

        #[test]
        fn should_not_grow_a_vector_if_not_needed() {
            let mut writer = vec![0; 4];
            let value = 0xaabbccddu32;
            let written_length = writer
                .write_be(0, &value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 4);

            let result = writer
                .read_be::<u32>(0)
                .expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
            assert_eq!(writer.len(), 4);
        }

        #[derive(Debug)]
        struct CustomErrorTest(u32);

        impl EndianWrite for CustomErrorTest {
            fn get_size(&self) -> usize {
                0
            }
            fn try_write_le(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                unimplemented!()
            }
            fn try_write_be(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                Err(Error::InvalidRead {
                    message: "Custom error!",
                })
            }
        }

        #[test]
        fn should_bubble_up_custom_errors_for_vec() {
            let value = CustomErrorTest(0);
            let mut bytes = vec![];
            let result = bytes.write_be(0, &value).unwrap_err();
            let expected = Error::InvalidRead {
                message: "Custom error!",
            };
            assert_eq!(result, expected)
        }

        #[test]
        fn should_bubble_up_custom_errors_for_slice() {
            let value = CustomErrorTest(0);
            let bytes = &mut [];
            let result = bytes.write_be(0, &value).unwrap_err();
            let expected = Error::InvalidRead {
                message: "Custom error!",
            };
            assert_eq!(result, expected)
        }

        #[derive(Debug)]
        struct OffsetErrorTest(u32);

        impl EndianWrite for OffsetErrorTest {
            fn get_size(&self) -> usize {
                0
            }
            fn try_write_le(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                unimplemented!()
            }
            fn try_write_be(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                Err(Error::InvalidSize {
                    wanted_size: 8,
                    offset: 1,
                    data_len: 0,
                })
            }
        }

        #[test]
        fn should_bubble_up_error_offsets_for_vec() {
            let value = OffsetErrorTest(0);
            let mut bytes = vec![];
            let result = bytes.write_be(2, &value).unwrap_err();
            let expected = Error::InvalidSize {
                wanted_size: 8,
                offset: 3,
                data_len: 2,
            };
            assert_eq!(result, expected)
        }

        #[test]
        fn should_bubble_up_error_offsets_for_slice() {
            let value = OffsetErrorTest(0);
            let bytes = &mut [];
            let result = bytes.write_be(2, &value).unwrap_err();
            let expected = Error::InvalidSize {
                wanted_size: 8,
                offset: 3,
                data_len: 0,
            };
            assert_eq!(result, expected)
        }
    }

    mod checked_write_be {
        use super::*;

        #[test]
        fn should_write_value() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = 0xaabbccddu32;
            let written_length = writer.checked_write_be(2, &value);

            assert_eq!(written_length, 4);

            let result = writer
                .read_be::<u32>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, 0xaabbccddu32);
        }

        #[test]
        fn should_return_0_if_size_is_too_large_for_offset() {
            let bytes = [1, 2, 3, 4, 5, 6, 7, 8];
            let mut writer = MockWriter::new(bytes.clone());
            let value = 0xaabbccddu32;
            let written_length = writer.checked_write_be(6, &value);

            assert_eq!(written_length, 0);
            assert_eq!(writer.get_bytes(), bytes);
        }
    }

    mod write_array_le {
        use super::*;
        use alloc::vec;

        #[test]
        fn should_write_value() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = [0x1122u16, 0x3344, 0x5566];
            let written_length = writer
                .write_array_le(2, &value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 6);

            let result = writer
                .read_array_le::<3, u16>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, [0x1122u16, 0x3344, 0x5566]);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = [0x1122u16, 0x3344, 0x5566];
            let error = writer
                .write_array_le(6, &value)
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

        #[test]
        fn should_grow_a_vector_if_needed() {
            let mut writer = vec![];
            let value = [0x1122u16, 0x3344, 0x5566];
            let written_length = writer
                .write_array_le(2, &value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 6);

            let result = writer
                .read_array_le::<3, u16>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, [0x1122u16, 0x3344, 0x5566]);
            assert_eq!(writer.len(), 8);
        }

        #[test]
        fn should_not_grow_a_vector_if_not_needed() {
            let mut writer = vec![0; 6];
            let value = [0x1122u16, 0x3344, 0x5566];
            let written_length = writer
                .write_array_le(0, &value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 6);

            let result = writer
                .read_array_le::<3, u16>(0)
                .expect("Read should have succeeded");
            assert_eq!(result, [0x1122u16, 0x3344, 0x5566]);
            assert_eq!(writer.len(), 6);
        }

        #[derive(Debug)]
        struct CustomErrorTest(u32);

        impl EndianWrite for CustomErrorTest {
            fn get_size(&self) -> usize {
                0
            }
            fn try_write_le(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                Err(Error::InvalidRead {
                    message: "Custom error!",
                })
            }
            fn try_write_be(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                unimplemented!()
            }
        }

        #[test]
        fn should_bubble_up_custom_errors_for_vec() {
            let value = [CustomErrorTest(0)];
            let mut bytes = vec![];
            let result = bytes.write_array_le(0, &value).unwrap_err();
            let expected = Error::InvalidRead {
                message: "Custom error!",
            };
            assert_eq!(result, expected)
        }

        #[test]
        fn should_bubble_up_custom_errors_for_slice() {
            let value = [CustomErrorTest(0)];
            let bytes = &mut [];
            let result = bytes.write_array_le(0, &value).unwrap_err();
            let expected = Error::InvalidRead {
                message: "Custom error!",
            };
            assert_eq!(result, expected)
        }

        #[derive(Debug)]
        struct OffsetErrorTest(u32);

        impl EndianWrite for OffsetErrorTest {
            fn get_size(&self) -> usize {
                0
            }
            fn try_write_le(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                Err(Error::InvalidSize {
                    wanted_size: 8,
                    offset: 1,
                    data_len: 0,
                })
            }
            fn try_write_be(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                unimplemented!()
            }
        }

        #[test]
        fn should_bubble_up_error_offsets_for_vec() {
            let value = [OffsetErrorTest(0)];
            let mut bytes = vec![];
            let result = bytes.write_array_le(2, &value).unwrap_err();
            let expected = Error::InvalidSize {
                wanted_size: 8,
                offset: 3,
                data_len: 2,
            };
            assert_eq!(result, expected)
        }

        #[test]
        fn should_bubble_up_error_offsets_for_slice() {
            let value = [OffsetErrorTest(0)];
            let bytes = &mut [];
            let result = bytes.write_array_le(2, &value).unwrap_err();
            let expected = Error::InvalidSize {
                wanted_size: 8,
                offset: 3,
                data_len: 0,
            };
            assert_eq!(result, expected)
        }
    }

    mod checked_write_array_le {
        use super::*;

        #[test]
        fn should_write_value() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = [0x1122u16, 0x3344, 0x5566];
            let written_length = writer.checked_write_array_le(2, &value);

            assert_eq!(written_length, 6);

            let result = writer
                .read_array_le::<3, u16>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, [0x1122u16, 0x3344, 0x5566]);
        }

        #[test]
        fn should_return_0_if_size_is_too_large_for_offset() {
            let bytes = [1, 2, 3, 4, 5, 6, 7, 8];
            let mut writer = MockWriter::new(bytes.clone());
            let value = [0x1122u16, 0x3344, 0x5566];
            let written_length = writer.checked_write_array_le(6, &value);

            assert_eq!(written_length, 0);
            assert_eq!(writer.get_bytes(), bytes);
        }
    }

    mod write_array_be {
        use super::*;
        use alloc::vec;

        #[test]
        fn should_write_value() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = [0x1122u16, 0x3344, 0x5566];
            let written_length = writer
                .write_array_be(2, &value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 6);

            let result = writer
                .read_array_be::<3, u16>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, [0x1122u16, 0x3344, 0x5566]);
        }

        #[test]
        fn should_return_error_if_size_is_too_large_for_offset() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = [0x1122u16, 0x3344, 0x5566];
            let error = writer
                .write_array_be(6, &value)
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

        #[test]
        fn should_grow_a_vector_if_needed() {
            let mut writer = vec![];
            let value = [0x1122u16, 0x3344, 0x5566];
            let written_length = writer
                .write_array_be(2, &value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 6);

            let result = writer
                .read_array_be::<3, u16>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, [0x1122u16, 0x3344, 0x5566]);
            assert_eq!(writer.len(), 8);
        }

        #[test]
        fn should_not_grow_a_vector_if_not_needed() {
            let mut writer = vec![0; 6];
            let value = [0x1122u16, 0x3344, 0x5566];
            let written_length = writer
                .write_array_be(0, &value)
                .expect("Write should have succeeded");

            assert_eq!(written_length, 6);

            let result = writer
                .read_array_be::<3, u16>(0)
                .expect("Read should have succeeded");
            assert_eq!(result, [0x1122u16, 0x3344, 0x5566]);
            assert_eq!(writer.len(), 6);
        }

        #[derive(Debug)]
        struct CustomErrorTest(u32);

        impl EndianWrite for CustomErrorTest {
            fn get_size(&self) -> usize {
                0
            }
            fn try_write_le(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                unimplemented!()
            }
            fn try_write_be(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                Err(Error::InvalidRead {
                    message: "Custom error!",
                })
            }
        }

        #[test]
        fn should_bubble_up_custom_errors_for_vec() {
            let value = [CustomErrorTest(0)];
            let mut bytes = vec![];
            let result = bytes.write_array_be(0, &value).unwrap_err();
            let expected = Error::InvalidRead {
                message: "Custom error!",
            };
            assert_eq!(result, expected)
        }

        #[test]
        fn should_bubble_up_custom_errors_for_slice() {
            let value = [CustomErrorTest(0)];
            let bytes = &mut [];
            let result = bytes.write_array_be(0, &value).unwrap_err();
            let expected = Error::InvalidRead {
                message: "Custom error!",
            };
            assert_eq!(result, expected)
        }

        #[derive(Debug)]
        struct OffsetErrorTest(u32);

        impl EndianWrite for OffsetErrorTest {
            fn get_size(&self) -> usize {
                0
            }
            fn try_write_le(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                unimplemented!()
            }
            fn try_write_be(&self, _dst: &mut [u8]) -> Result<usize, Error> {
                Err(Error::InvalidSize {
                    wanted_size: 8,
                    offset: 1,
                    data_len: 0,
                })
            }
        }

        #[test]
        fn should_bubble_up_error_offsets_for_vec() {
            let value = [OffsetErrorTest(0)];
            let mut bytes = vec![];
            let result = bytes.write_array_be(2, &value).unwrap_err();
            let expected = Error::InvalidSize {
                wanted_size: 8,
                offset: 3,
                data_len: 2,
            };
            assert_eq!(result, expected)
        }

        #[test]
        fn should_bubble_up_error_offsets_for_slice() {
            let value = [OffsetErrorTest(0)];
            let bytes = &mut [];
            let result = bytes.write_array_be(2, &value).unwrap_err();
            let expected = Error::InvalidSize {
                wanted_size: 8,
                offset: 3,
                data_len: 0,
            };
            assert_eq!(result, expected)
        }
    }

    mod checked_write_array_be {
        use super::*;

        #[test]
        fn should_write_value() {
            let mut writer = MockWriter::new([1, 2, 3, 4, 5, 6, 7, 8]);
            let value = [0x1122u16, 0x3344, 0x5566];
            let written_length = writer.checked_write_array_be(2, &value);

            assert_eq!(written_length, 6);

            let result = writer
                .read_array_be::<3, u16>(2)
                .expect("Read should have succeeded");
            assert_eq!(result, [0x1122u16, 0x3344, 0x5566]);
        }

        #[test]
        fn should_return_0_if_size_is_too_large_for_offset() {
            let bytes = [1, 2, 3, 4, 5, 6, 7, 8];
            let mut writer = MockWriter::new(bytes.clone());
            let value = [0x1122u16, 0x3344, 0x5566];
            let written_length = writer.checked_write_array_be(6, &value);

            assert_eq!(written_length, 0);
            assert_eq!(writer.get_bytes(), bytes);
        }
    }
}
