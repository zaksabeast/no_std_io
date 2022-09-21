use crate::Error;
use core::{convert::TryInto, marker::PhantomData, mem};

/// The result of a read, including the value that was
/// read and the number of bytes it consumed.
#[derive(Debug, PartialEq)]
pub struct ReadOutput<T: Sized> {
    data: T,
    read_bytes: usize,
}

impl<T: Sized> ReadOutput<T> {
    #[inline(always)]
    pub fn new(data: T, read_bytes: usize) -> Self {
        Self { data, read_bytes }
    }

    /// Consumes the read output and returns the inner data.
    #[inline(always)]
    pub fn into_data(self) -> T {
        self.data
    }

    /// Returns the number of bytes used to read the data.
    #[inline(always)]
    pub fn get_read_bytes(&self) -> usize {
        self.read_bytes
    }

    /// Converts the data of ReadOutput into a new type,
    /// and retains the read bytes.
    #[inline(always)]
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

macro_rules! impl_endian_read {
    ($($i:ty),*) => {
        $(
            impl EndianRead for $i {
                #[inline(always)]
                fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
                    let byte_count = mem::size_of::<$i>();

                    if byte_count > bytes.len() {
                        return Err(Error::InvalidSize {
                            wanted_size: byte_count,
                            offset: 0,
                            data_len: bytes.len(),
                        });
                    }

                    Ok(ReadOutput {
                        data: <$i>::from_le_bytes(bytes[..byte_count].try_into().unwrap()),
                        read_bytes: byte_count,
                    })
                }

                #[inline(always)]
                fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
                    let byte_count = mem::size_of::<$i>();

                    if byte_count > bytes.len() {
                        return Err(Error::InvalidSize {
                            wanted_size: byte_count,
                            offset: 0,
                            data_len: bytes.len(),
                        });
                    }

                    Ok(ReadOutput {
                        data: <$i>::from_be_bytes(bytes[..byte_count].try_into().unwrap()),
                        read_bytes: byte_count,
                    })
                }
            }
        )*
    };
}

impl_endian_read!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);

impl EndianRead for bool {
    #[inline(always)]
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let result = u8::try_read_le(bytes)?;
        Ok(ReadOutput {
            read_bytes: result.get_read_bytes(),
            data: result.into_data() != 0,
        })
    }

    #[inline(always)]
    fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let result = u8::try_read_le(bytes)?;
        Ok(ReadOutput {
            read_bytes: result.get_read_bytes(),
            data: result.into_data() != 0,
        })
    }
}

impl<const SIZE: usize> EndianRead for [u8; SIZE] {
    #[inline(always)]
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        if SIZE > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: SIZE,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: bytes[..SIZE].try_into().unwrap(),
            read_bytes: SIZE,
        })
    }

    #[inline(always)]
    fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        if SIZE > bytes.len() {
            return Err(Error::InvalidSize {
                wanted_size: SIZE,
                offset: 0,
                data_len: bytes.len(),
            });
        }

        Ok(ReadOutput {
            data: bytes[..SIZE].try_into().unwrap(),
            read_bytes: SIZE,
        })
    }
}

impl EndianRead for () {
    #[inline(always)]
    fn try_read_le(_bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        Ok(ReadOutput::new((), 0))
    }

    #[inline(always)]
    fn try_read_be(_bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        Ok(ReadOutput::new((), 0))
    }
}

impl<T: EndianRead> EndianRead for PhantomData<T> {
    #[inline(always)]
    fn try_read_le(_bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        Ok(ReadOutput::new(PhantomData, 0))
    }

    #[inline(always)]
    fn try_read_be(_bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        Ok(ReadOutput::new(PhantomData, 0))
    }
}
