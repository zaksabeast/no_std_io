use crate::Error;
use core::{convert::TryInto, mem};

/// The result of a read, including the value that was
/// read and the number of bytes it consumed.
#[derive(Debug, PartialEq)]
pub struct ReadOutput<T: Sized> {
    data: T,
    read_bytes: usize,
}

impl<T: Sized> ReadOutput<T> {
    pub fn new(data: T, read_bytes: usize) -> Self {
        Self { data, read_bytes }
    }

    /// Consumes the read output and returns the inner data.
    pub fn into_data(self) -> T {
        self.data
    }

    /// Returns the number of bytes used to read the data.
    pub fn get_read_bytes(&self) -> usize {
        self.read_bytes
    }

    /// Converts the data of ReadOutput into a new type,
    /// and retains the read bytes.
    pub fn into_other<U: From<T>>(self) -> ReadOutput<U> {
        let read_bytes = self.get_read_bytes();
        let data = self.into_data().into();
        ReadOutput { data, read_bytes }
    }
}

/// Defines a shared interface to read data from a source that is endian specific.
///
/// This should only be used when handling an external data source, such as a remote API or file.
/// Usually you'll want code to be endian agnostic.
pub trait EndianRead: Sized {
    /// Tries to read the value from its little endian representation.
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error>;
    /// Tries to read the value from its big endian representation.
    fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error>;
}

impl EndianRead for u8 {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<u8>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: u8::from_le_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }

    fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<u8>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: u8::from_be_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }
}

impl EndianRead for i8 {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<i8>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: i8::from_le_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }

    fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<i8>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: i8::from_be_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }
}

impl EndianRead for u16 {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<u16>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: u16::from_le_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }

    fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<u16>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: u16::from_be_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }
}

impl EndianRead for i16 {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<i16>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: i16::from_le_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }

    fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<i16>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: i16::from_be_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }
}

impl EndianRead for u32 {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<u32>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: u32::from_le_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }

    fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<u32>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: u32::from_be_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }
}

impl EndianRead for i32 {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<i32>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: i32::from_le_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }

    fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<i32>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: i32::from_be_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }
}

impl EndianRead for u64 {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<u64>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: u64::from_le_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }

    fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<u64>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: u64::from_be_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }
}

impl EndianRead for i64 {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<i64>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: i64::from_le_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }

    fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let byte_count = mem::size_of::<i64>();

        if byte_count > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: byte_count,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: i64::from_be_bytes(bytes[..byte_count].try_into().unwrap()),
            read_bytes: byte_count,
        })
    }
}
