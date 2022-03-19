use crate::{Cursor, Reader, Writer};

pub struct StreamContainer<const SIZE: usize> {
    raw: [u8; SIZE],
    cursor: usize,
}

impl<const SIZE: usize> StreamContainer<SIZE> {
    pub fn new(raw: [u8; SIZE]) -> Self {
        Self { raw, cursor: 0 }
    }

    pub fn into_raw(self) -> [u8; SIZE] {
        self.raw
    }
}

impl<const SIZE: usize> Reader for StreamContainer<SIZE> {
    fn get_slice(&self) -> &[u8] {
        &self.raw
    }
}

impl<const SIZE: usize> Writer for StreamContainer<SIZE> {
    fn get_mut_slice(&mut self) -> &mut [u8] {
        &mut self.raw
    }
}

impl<const SIZE: usize> Cursor for StreamContainer<SIZE> {
    fn get_index(&self) -> usize {
        self.cursor
    }

    fn set_index(&mut self, index: usize) {
        self.cursor = index;
    }
}
