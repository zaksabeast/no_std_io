use core::mem::size_of;

/// An interface for working with cursors by getting and setting an index.
pub trait Cursor {
    fn get_index(&self) -> usize;
    fn set_index(&mut self, index: usize);

    /// Increments the index by the given amount.
    #[inline(always)]
    fn increment_by(&mut self, count: usize) {
        self.set_index(self.get_index() + count);
    }

    /// Returns the current index and replaces it with the provided size.
    #[inline(always)]
    fn swap_incremented_index(&mut self, size: usize) -> usize {
        let index = self.get_index();
        self.increment_by(size);
        index
    }

    /// Returns the current index and replaces it
    /// with the size of the provided type added to the index.
    #[inline(always)]
    fn swap_incremented_index_for_type<T: Sized>(&mut self) -> usize {
        let size = size_of::<T>();
        self.swap_incremented_index(size)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct MockCursor {
        index: usize,
    }

    impl MockCursor {
        fn new(index: usize) -> Self {
            Self { index }
        }
    }

    impl Cursor for MockCursor {
        fn get_index(&self) -> usize {
            self.index
        }
        fn set_index(&mut self, index: usize) {
            self.index = index;
        }
    }

    #[test]
    fn should_get_the_current_index() {
        let cursor = MockCursor::new(3);
        assert_eq!(cursor.get_index(), 3)
    }

    #[test]
    fn should_set_the_current_index() {
        let mut cursor = MockCursor::new(3);
        cursor.set_index(5);
        assert_eq!(cursor.get_index(), 5)
    }

    #[test]
    fn should_swap_incremented_index() {
        let mut cursor = MockCursor::new(3);
        let previous_index = cursor.swap_incremented_index(5);
        let current_index = cursor.get_index();

        assert_eq!(previous_index, 3);
        assert_eq!(current_index, 8);
    }

    #[test]
    fn should_swap_incremented_index_for_type() {
        let mut cursor = MockCursor::new(3);
        let previous_index = cursor.swap_incremented_index_for_type::<u32>();
        let current_index = cursor.get_index();

        assert_eq!(previous_index, 3);
        assert_eq!(current_index, 7);
    }
}
