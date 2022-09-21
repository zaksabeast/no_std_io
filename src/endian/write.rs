use crate::Error;
use core::{marker::PhantomData, mem};

/// Defines a shared interface to write data to a source that is endian specific.
///
/// This should only be used when handling an external data source, such as a remote API or file.
/// Usually you'll want code to be endian agnostic.
pub trait EndianWrite {
    /// Returns the size of the data that is to be written.
    fn get_size(&self) -> usize;
    /// Tries to write the value from its little endian representation.
    /// Returns the number of bytes written.
    fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error>;
    /// Tries to write the value from its big endian representation.
    /// Returns the number of bytes written.
    fn try_write_be(&self, dst: &mut [u8]) -> Result<usize, Error>;
}

macro_rules! impl_endian_write {
    ($($i:ident),*) => {
        $(
            impl EndianWrite for $i {
                #[inline(always)]
                fn get_size(&self) -> usize {
                    mem::size_of::<$i>()
                }

                #[inline(always)]
                fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error> {
                    let byte_count = mem::size_of::<$i>();

                    if byte_count > dst.len() {
                        return Err(Error::InvalidSize {
                            wanted_size: byte_count,
                            offset: 0,
                            data_len: dst.len(),
                        });
                    }

                    let bytes = self.to_le_bytes();
                    dst[..byte_count].copy_from_slice(&bytes);
                    Ok(bytes.len())
                }

                #[inline(always)]
                fn try_write_be(&self, dst: &mut [u8]) -> Result<usize, Error> {
                    let byte_count = mem::size_of::<$i>();

                    if byte_count > dst.len() {
                        return Err(Error::InvalidSize {
                            wanted_size: byte_count,
                            offset: 0,
                            data_len: dst.len(),
                        });
                    }

                    let bytes = self.to_be_bytes();
                    dst[..byte_count].copy_from_slice(&bytes);
                    Ok(bytes.len())
                }
            }
        )*
    };
}

impl_endian_write!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);

impl EndianWrite for bool {
    #[inline(always)]
    fn get_size(&self) -> usize {
        mem::size_of::<Self>()
    }

    #[inline(always)]
    fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error> {
        let byte_count = mem::size_of::<bool>();

        if byte_count > dst.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: dst.len(),
            });
        }

        let bytes = [*self as u8];
        dst[..byte_count].copy_from_slice(&bytes);
        Ok(bytes.len())
    }

    #[inline(always)]
    fn try_write_be(&self, dst: &mut [u8]) -> Result<usize, Error> {
        let byte_count = mem::size_of::<bool>();

        if byte_count > dst.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: dst.len(),
            });
        }

        let bytes = [*self as u8];
        dst[..byte_count].copy_from_slice(&bytes);
        Ok(bytes.len())
    }
}

impl<const SIZE: usize> EndianWrite for [u8; SIZE] {
    #[inline(always)]
    fn get_size(&self) -> usize {
        SIZE
    }

    #[inline(always)]
    fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error> {
        if SIZE > dst.len() {
            return Err(Error::InvalidSize {
                wanted_size: SIZE,
                offset: 0,
                data_len: dst.len(),
            });
        }

        dst[..SIZE].copy_from_slice(self);
        Ok(SIZE)
    }

    #[inline(always)]
    fn try_write_be(&self, dst: &mut [u8]) -> Result<usize, Error> {
        if SIZE > dst.len() {
            return Err(Error::InvalidSize {
                wanted_size: SIZE,
                offset: 0,
                data_len: dst.len(),
            });
        }

        dst[..SIZE].copy_from_slice(self);
        Ok(SIZE)
    }
}

impl EndianWrite for () {
    #[inline(always)]
    fn get_size(&self) -> usize {
        0
    }

    #[inline(always)]
    fn try_write_le(&self, _dst: &mut [u8]) -> Result<usize, Error> {
        Ok(0)
    }

    #[inline(always)]
    fn try_write_be(&self, _dst: &mut [u8]) -> Result<usize, Error> {
        Ok(0)
    }
}

impl<T: EndianWrite> EndianWrite for PhantomData<T> {
    #[inline(always)]
    fn get_size(&self) -> usize {
        0
    }

    #[inline(always)]
    fn try_write_le(&self, _dst: &mut [u8]) -> Result<usize, Error> {
        Ok(0)
    }

    #[inline(always)]
    fn try_write_be(&self, _dst: &mut [u8]) -> Result<usize, Error> {
        Ok(0)
    }
}
