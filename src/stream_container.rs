use crate::{Cursor, Reader, Writer};

pub struct StreamContainer<T: Reader> {
    raw: T,
    cursor: usize,
}

impl<T: Reader> StreamContainer<T> {
    pub fn new(raw: T) -> Self {
        Self { raw, cursor: 0 }
    }

    pub fn into_raw(self) -> T {
        self.raw
    }
}

impl<T: Reader> Reader for StreamContainer<T> {
    fn get_slice(&self) -> &[u8] {
        self.raw.get_slice()
    }
}

impl<T: Reader + Writer> Writer for StreamContainer<T> {
    fn get_mut_slice(&mut self) -> &mut [u8] {
        self.raw.get_mut_slice()
    }
}

impl<T: Reader> Cursor for StreamContainer<T> {
    fn get_index(&self) -> usize {
        self.cursor
    }

    fn set_index(&mut self, index: usize) {
        self.cursor = index;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::StreamReader;
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
}
