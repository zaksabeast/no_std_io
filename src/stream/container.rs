use crate::{Cursor, EndianWrite, Reader, Writer, WriterResult};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// A convenience container that allows streaming anything that implements [Reader].
/// The container can also write to anything that implements [Writer], but only [Reader] is needed
/// to use the container.
///
/// To forward streamed [Writer] methods to containers with vectors, implement both [StreamContainer::get_mut_slice]
/// and [StreamContainer::get_sized_mut_slice] instead of only [StreamContainer::get_mut_slice].
pub struct StreamContainer<T: Reader> {
    raw: T,
    cursor: usize,
}

impl<T: Reader> StreamContainer<T> {
    #[inline(always)]
    pub fn new(raw: T) -> Self {
        Self { raw, cursor: 0 }
    }

    #[inline(always)]
    pub fn into_raw(self) -> T {
        self.raw
    }
}

impl<T: Reader> Reader for StreamContainer<T> {
    #[inline(always)]
    fn get_slice(&self) -> &[u8] {
        self.raw.get_slice()
    }
}

impl<T: Reader + Writer> Writer for StreamContainer<T> {
    #[inline(always)]
    fn get_mut_slice(&mut self) -> &mut [u8] {
        self.raw.get_mut_slice()
    }

    #[inline(always)]
    fn get_sized_mut_slice(&mut self, offset: usize, length: usize) -> WriterResult<&mut [u8]> {
        self.raw.get_sized_mut_slice(offset, length)
    }

    #[inline(always)]
    fn write_le<U: EndianWrite>(&mut self, offset: usize, value: &U) -> WriterResult<usize> {
        self.raw.write_le(offset, value)
    }

    #[inline(always)]
    fn write_be<U: EndianWrite>(&mut self, offset: usize, value: &U) -> WriterResult<usize> {
        self.raw.write_be(offset, value)
    }
}

impl<T: Reader> Cursor for StreamContainer<T> {
    #[inline(always)]
    fn get_index(&self) -> usize {
        self.cursor
    }

    #[inline(always)]
    fn set_index(&mut self, index: usize) {
        self.cursor = index;
    }
}

impl<'a> From<StreamContainer<&'a mut [u8]>> for &'a mut [u8] {
    #[inline(always)]
    fn from(stream: StreamContainer<&'a mut [u8]>) -> Self {
        stream.into_raw()
    }
}

impl<'a> From<StreamContainer<&'a [u8]>> for &'a [u8] {
    #[inline(always)]
    fn from(stream: StreamContainer<&'a [u8]>) -> Self {
        stream.into_raw()
    }
}

impl<const SIZE: usize> From<StreamContainer<[u8; SIZE]>> for [u8; SIZE] {
    #[inline(always)]
    fn from(stream: StreamContainer<[u8; SIZE]>) -> Self {
        stream.into_raw()
    }
}

#[cfg(feature = "alloc")]
impl From<StreamContainer<Vec<u8>>> for Vec<u8> {
    #[inline(always)]
    fn from(stream: StreamContainer<Vec<u8>>) -> Self {
        stream.into_raw()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{StreamReader, StreamWriter};
    use alloc::vec;

    #[test]
    fn should_work_with_vectors() {
        let data = vec![0xaa, 0xbb, 0xcc, 0xdd];
        StreamContainer::new(data).default_read_stream::<u32>();
    }

    #[test]
    fn should_work_with_slices() {
        let data: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
        StreamContainer::new(data.as_slice()).default_read_stream::<u32>();
    }

    #[test]
    fn should_work_with_mut_slices() {
        let mut data: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
        StreamContainer::new(data.as_mut_slice()).default_read_stream::<u32>();
    }

    #[test]
    fn should_grow_a_vector_if_needed() {
        let data = vec![];
        let mut stream = StreamContainer::new(data);
        stream.checked_write_stream_bytes(&[0xaa, 0xbb, 0xcc, 0xdd]);
        assert_eq!(stream.into_raw(), [0xaa, 0xbb, 0xcc, 0xdd]);
    }

    #[test]
    fn should_grow_a_vector_if_needed_with_le() {
        let data = vec![];
        let mut stream = StreamContainer::new(data);
        stream.write_stream_le(&0xaabbccddu32).unwrap();
        assert_eq!(stream.into_raw(), [0xdd, 0xcc, 0xbb, 0xaa]);
    }

    #[test]
    fn should_grow_a_vector_if_needed_with_be() {
        let data = vec![];
        let mut stream = StreamContainer::new(data);
        stream.checked_write_stream_be(&0xaabbccddu32);
        assert_eq!(stream.into_raw(), [0xaa, 0xbb, 0xcc, 0xdd]);
    }

    #[test]
    fn should_not_grow_a_vector_if_not_needed() {
        let data = vec![0; 4];
        let mut stream = StreamContainer::new(data);
        stream.checked_write_stream_bytes(&[0xaa, 0xbb, 0xcc, 0xdd]);
        assert_eq!(stream.into_raw(), [0xaa, 0xbb, 0xcc, 0xdd]);
    }

    #[test]
    fn should_grow_a_vector_if_needed_and_written_to_twice() {
        let data = vec![0; 4];
        let mut stream = StreamContainer::new(data);
        stream.checked_write_stream_bytes(&[0xaa, 0xbb, 0xcc, 0xdd]);
        stream.checked_write_stream_bytes(&[0xaa, 0xbb, 0xcc, 0xdd]);
        assert_eq!(
            stream.into_raw(),
            [0xaa, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0xdd]
        );
    }
}
