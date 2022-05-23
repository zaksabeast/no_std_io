use crate::Error;
use core::mem;

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

impl EndianWrite for u8 {
    #[inline(always)]
    fn get_size(&self) -> usize {
        mem::size_of::<Self>()
    }

    #[inline(always)]
    fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error> {
        let byte_count = mem::size_of::<u8>();

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
        let byte_count = mem::size_of::<u8>();

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

impl EndianWrite for i8 {
    #[inline(always)]
    fn get_size(&self) -> usize {
        mem::size_of::<Self>()
    }

    #[inline(always)]
    fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error> {
        let byte_count = mem::size_of::<i8>();

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
        let byte_count = mem::size_of::<i8>();

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

impl EndianWrite for u16 {
    #[inline(always)]
    fn get_size(&self) -> usize {
        mem::size_of::<Self>()
    }

    #[inline(always)]
    fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error> {
        let byte_count = mem::size_of::<u16>();

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
        let byte_count = mem::size_of::<u16>();

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

impl EndianWrite for i16 {
    #[inline(always)]
    fn get_size(&self) -> usize {
        mem::size_of::<Self>()
    }

    #[inline(always)]
    fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error> {
        let byte_count = mem::size_of::<i16>();

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
        let byte_count = mem::size_of::<i16>();

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

impl EndianWrite for u32 {
    #[inline(always)]
    fn get_size(&self) -> usize {
        mem::size_of::<Self>()
    }

    #[inline(always)]
    fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error> {
        let byte_count = mem::size_of::<u32>();

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
        let byte_count = mem::size_of::<u32>();

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

impl EndianWrite for i32 {
    #[inline(always)]
    fn get_size(&self) -> usize {
        mem::size_of::<Self>()
    }

    #[inline(always)]
    fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error> {
        let byte_count = mem::size_of::<i32>();

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
        let byte_count = mem::size_of::<i32>();

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

impl EndianWrite for u64 {
    #[inline(always)]
    fn get_size(&self) -> usize {
        mem::size_of::<Self>()
    }

    #[inline(always)]
    fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error> {
        let byte_count = mem::size_of::<u64>();

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
        let byte_count = mem::size_of::<u64>();

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

impl EndianWrite for i64 {
    #[inline(always)]
    fn get_size(&self) -> usize {
        mem::size_of::<Self>()
    }

    #[inline(always)]
    fn try_write_le(&self, dst: &mut [u8]) -> Result<usize, Error> {
        let byte_count = mem::size_of::<i64>();

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
        let byte_count = mem::size_of::<i64>();

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

impl EndianWrite for () {
    fn get_size(&self) -> usize {
        0
    }

    fn try_write_le(&self, _dst: &mut [u8]) -> Result<usize, Error> {
        Ok(0)
    }

    fn try_write_be(&self, _dst: &mut [u8]) -> Result<usize, Error> {
        Ok(0)
    }
}
