use crate::{EndianRead, StreamReader};
use core::marker::PhantomData;

/// An iterator for the little endian representation of an [EndianRead] type from a [StreamReader].
pub struct LeIter<Item: EndianRead, Stream: StreamReader> {
    data: PhantomData<Item>,
    stream: Stream,
}

impl<Item: EndianRead, Stream: StreamReader> LeIter<Item, Stream> {
    pub fn new(stream: Stream) -> Self {
        Self {
            data: PhantomData,
            stream,
        }
    }
}

impl<Item: EndianRead, Stream: StreamReader> Iterator for LeIter<Item, Stream> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream.read_stream_le().ok()
    }
}

/// An iterator for the big endian representation of an [EndianRead] type from a [StreamReader].
pub struct BeIter<Item: EndianRead, Stream: StreamReader> {
    data: PhantomData<Item>,
    stream: Stream,
}

impl<Item: EndianRead, Stream: StreamReader> BeIter<Item, Stream> {
    pub fn new(stream: Stream) -> Self {
        Self {
            data: PhantomData,
            stream,
        }
    }
}

impl<Item: EndianRead, Stream: StreamReader> Iterator for BeIter<Item, Stream> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream.read_stream_be().ok()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::StreamContainer;
    use alloc::vec::Vec;

    mod le_iter {
        use super::*;

        #[test]
        fn should_iterate() {
            let bytes: [u8; 8] = [0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x22, 0x33, 0x44];
            let stream = StreamContainer::new(bytes);
            let result: Vec<u32> = LeIter::new(stream).collect();
            assert_eq!(result, [0xddccbbaa, 0x44332211])
        }
    }

    mod be_iter {
        use super::*;

        #[test]
        fn should_iterate() {
            let bytes: [u8; 8] = [0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x22, 0x33, 0x44];
            let stream = StreamContainer::new(bytes);
            let result: Vec<u32> = BeIter::new(stream).collect();
            assert_eq!(result, [0xaabbccdd, 0x11223344])
        }
    }
}
