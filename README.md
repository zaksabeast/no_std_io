# no_std_io

This is a set of tools to make reading and writing data easy.

Benefits:

- Works with no_std
- Optional alloc feature
- Traits are provided so data can come from any source
- Padding attributes for aligned data

## Examples

```rs
let raw: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];

// Read available data
let u32_result = raw.read::<u32>(0);
assert_eq!(u32_result, Ok(0xddccbbaa));

// Type inference
let u16_result: u16 = raw.default_read(0);
assert_eq!(u16_result, 0xbbaa);

// Endianness
let le_result: u16 = raw.default_read_le(2);
assert_eq!(le_result, 0xddcc);
let be_result: u16 = raw.default_read_be(2);
assert_eq!(be_result, 0xccdd);

// Vectors
let bytes = raw.default_read_byte_vec(1, 2);
assert_eq!(bytes, vec![0xbb, 0xcc]);

// Streams
let mut stream = StreamContainer::new(raw);
let first = stream.read_stream::<u16>();
let second = stream.read_stream::<u8>();
assert_eq!(first, Ok(0xbbaa));
assert_eq!(second, Ok(0xcc));

// Iterators from streams
let collected = stream.into_le_iter().collect::<Vec<u8>>();
assert_eq!(collected, [0xdd]);

// Derive macros for EndianRead and EndianWrite traits
#[derive(Debug, PartialEq, EndianRead, EndianWrite)]
struct Test {
    first: u16,
    second: u16,
}

let derived: Test = raw.read_le(0).unwrap();
assert_eq!(derived, Test { first: 0xbbaa, second: 0xddcc });
```
