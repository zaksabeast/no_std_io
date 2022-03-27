/// Defines a shared interface to write data from a source that is endian specific.
///
/// This should only be used when handling an external data source, such as a remote API or file.
/// Usually you'll want code to be endian agnostic.
pub trait EndianWrite: Sized {
    /// Write the value from its little endian representation.
    fn write_le(src: &Self, dst: &mut [u8]);
    /// Write the value from its big endian representation.
    fn write_be(src: &Self, dst: &mut [u8]);
}

impl EndianWrite for u8 {
    fn write_le(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_le_bytes());
    }

    fn write_be(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_be_bytes());
    }
}

impl EndianWrite for u16 {
    fn write_le(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_le_bytes());
    }

    fn write_be(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_be_bytes());
    }
}

impl EndianWrite for i16 {
    fn write_le(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_le_bytes());
    }

    fn write_be(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_be_bytes());
    }
}

impl EndianWrite for u32 {
    fn write_le(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_le_bytes());
    }

    fn write_be(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_be_bytes());
    }
}

impl EndianWrite for i32 {
    fn write_le(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_le_bytes());
    }

    fn write_be(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_be_bytes());
    }
}

impl EndianWrite for u64 {
    fn write_le(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_le_bytes());
    }

    fn write_be(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_be_bytes());
    }
}

impl EndianWrite for i64 {
    fn write_le(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_le_bytes());
    }

    fn write_be(src: &Self, dst: &mut [u8]) {
        dst.copy_from_slice(&src.to_be_bytes());
    }
}
