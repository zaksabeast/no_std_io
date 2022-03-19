use crate::{Cursor, Reader, Writer};

pub struct StreamContainer<T: Reader + Writer> {
    raw: T,
    cursor: usize,
}

impl<T: Reader + Writer> StreamContainer<T> {
    pub fn new(raw: T) -> Self {
        Self { raw, cursor: 0 }
    }

    pub fn into_raw(self) -> T {
        self.raw
    }
}

impl<T: Reader + Writer> Reader for StreamContainer<T> {
    fn get_slice(&self) -> &[u8] {
        self.raw.get_slice()
    }
}

impl<T: Reader + Writer> Writer for StreamContainer<T> {
    fn get_mut_slice(&mut self) -> &mut [u8] {
        self.raw.get_mut_slice()
    }
}

impl<T: Reader + Writer> Cursor for StreamContainer<T> {
    fn get_index(&self) -> usize {
        self.cursor
    }

    fn set_index(&mut self, index: usize) {
        self.cursor = index;
    }
}
