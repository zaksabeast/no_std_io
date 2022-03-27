use core::convert::TryInto;

/// Defines a shared interface to read data from a source that is endian specific.
///
/// This should only be used when handling an external data source, such as a remote API or file.
/// Usually you'll want code to be endian agnostic.
pub trait EndianRead: Sized {
    /// Read the value from its little endian representation.
    fn read_le(bytes: &[u8]) -> Self;
    /// Read the value from its big endian representation.
    fn read_be(bytes: &[u8]) -> Self;
}

impl EndianRead for u8 {
    fn read_le(bytes: &[u8]) -> Self {
        u8::from_le_bytes(bytes.try_into().unwrap())
    }

    fn read_be(bytes: &[u8]) -> Self {
        u8::from_be_bytes(bytes.try_into().unwrap())
    }
}

impl EndianRead for u16 {
    fn read_le(bytes: &[u8]) -> Self {
        u16::from_le_bytes(bytes.try_into().unwrap())
    }

    fn read_be(bytes: &[u8]) -> Self {
        u16::from_be_bytes(bytes.try_into().unwrap())
    }
}

impl EndianRead for i16 {
    fn read_le(bytes: &[u8]) -> Self {
        i16::from_le_bytes(bytes.try_into().unwrap())
    }

    fn read_be(bytes: &[u8]) -> Self {
        i16::from_be_bytes(bytes.try_into().unwrap())
    }
}

impl EndianRead for u32 {
    fn read_le(bytes: &[u8]) -> Self {
        u32::from_le_bytes(bytes.try_into().unwrap())
    }

    fn read_be(bytes: &[u8]) -> Self {
        u32::from_be_bytes(bytes.try_into().unwrap())
    }
}

impl EndianRead for i32 {
    fn read_le(bytes: &[u8]) -> Self {
        i32::from_le_bytes(bytes.try_into().unwrap())
    }

    fn read_be(bytes: &[u8]) -> Self {
        i32::from_be_bytes(bytes.try_into().unwrap())
    }
}

impl EndianRead for u64 {
    fn read_le(bytes: &[u8]) -> Self {
        u64::from_le_bytes(bytes.try_into().unwrap())
    }

    fn read_be(bytes: &[u8]) -> Self {
        u64::from_be_bytes(bytes.try_into().unwrap())
    }
}

impl EndianRead for i64 {
    fn read_le(bytes: &[u8]) -> Self {
        i64::from_le_bytes(bytes.try_into().unwrap())
    }

    fn read_be(bytes: &[u8]) -> Self {
        i64::from_be_bytes(bytes.try_into().unwrap())
    }
}
