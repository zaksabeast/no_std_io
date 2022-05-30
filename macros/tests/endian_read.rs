use macros::EndianRead;
use no_std_io::{Cursor, Error, ReadOutput, Reader, StreamContainer, StreamReader};

#[derive(Debug, Default, PartialEq, EndianRead)]
struct Test {
    first: u8,
    second: u32,
}

#[derive(Debug, Default, PartialEq)]
struct ListContainer<T: no_std_io::EndianRead>(Vec<T>);

impl<T: no_std_io::EndianRead> no_std_io::EndianRead for ListContainer<T> {
    #[inline(always)]
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        if bytes.is_empty() {
            return Err(Error::InvalidSize {
                wanted_size: 1,
                offset: 0,
                data_len: 0,
            });
        }

        let count = bytes[0] as usize;
        let mut stream = StreamContainer::new(&bytes[1..]);

        let mut list = vec![];

        for _ in 0..count {
            let item = stream.read_stream_le()?;
            list.push(item);
        }

        let result = ListContainer(list);
        let read_bytes = stream.get_index() + 1;

        Ok(ReadOutput::new(result, read_bytes))
    }

    #[inline(always)]
    fn try_read_be(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        if bytes.is_empty() {
            return Err(Error::InvalidSize {
                wanted_size: 1,
                offset: 0,
                data_len: 0,
            });
        }

        let count = bytes[0] as usize;
        let mut stream = StreamContainer::new(&bytes[1..]);

        let mut list = vec![];

        for _ in 0..count {
            let item = stream.read_stream_be()?;
            list.push(item);
        }

        let result = ListContainer(list);
        let read_bytes = stream.get_index() + 1;

        Ok(ReadOutput::new(result, read_bytes))
    }
}

#[derive(Debug, Default, PartialEq, EndianRead)]
struct TestContainer {
    test: Test,
    list: ListContainer<u32>,
}

#[test]
fn should_read_le() {
    let bytes: [u8; 5] = [0xaa, 0xbb, 0xcc, 0xdd, 0xee];
    let result: Test = bytes.read_le(0).expect("Read should have worked");
    let expected = Test {
        first: 0xaa,
        second: 0xeeddccbb,
    };

    assert_eq!(result, expected);
}

#[test]
fn should_read_be() {
    let bytes: [u8; 5] = [0xaa, 0xbb, 0xcc, 0xdd, 0xee];
    let result: Test = bytes.read_be(0).expect("Read should have worked");
    let expected = Test {
        first: 0xaa,
        second: 0xbbccddee,
    };

    assert_eq!(result, expected);
}

#[test]
fn should_error_if_there_are_not_enough_bytes() {
    let bytes = vec![0xaa, 0xbb, 0xcc, 0xdd];
    let result = bytes
        .read_le::<Test>(0)
        .expect_err("This should have failed");
    assert_eq!(
        result,
        Error::InvalidSize {
            wanted_size: 4,
            offset: 1,
            data_len: 4
        }
    );
}

#[test]
fn should_read_dynamic_size_le() {
    let bytes = vec![0x02, 0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd];
    let result: ListContainer<u32> = bytes.read_le(0).expect("Read should have worked");
    let expected = ListContainer(vec![0x44332211, 0xddccbbaa]);

    assert_eq!(result, expected);
}

#[test]
fn should_read_dynamic_size_be() {
    let bytes = vec![0x02, 0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb, 0xcc, 0xdd];
    let result: ListContainer<u32> = bytes.read_be(0).expect("Read should have worked");
    let expected = ListContainer(vec![0x11223344, 0xaabbccdd]);

    assert_eq!(result, expected);
}

#[test]
fn should_read_nested_le() {
    let bytes = vec![
        0x00, 0x11, 0x22, 0x33, 0x44, 0x02, 0xaa, 0xbb, 0xcc, 0xdd, 0x55, 0x66, 0x77, 0x88,
    ];
    let result: TestContainer = bytes.read_le(0).expect("Read should have worked");
    let expected = TestContainer {
        test: Test {
            first: 0x00,
            second: 0x44332211,
        },
        list: ListContainer(vec![0xddccbbaa, 0x88776655]),
    };

    assert_eq!(result, expected);
}

#[test]
fn should_read_nested_be() {
    let bytes = vec![
        0x00, 0x11, 0x22, 0x33, 0x44, 0x02, 0xaa, 0xbb, 0xcc, 0xdd, 0x55, 0x66, 0x77, 0x88,
    ];
    let result: TestContainer = bytes.read_be(0).expect("Read should have worked");
    let expected = TestContainer {
        test: Test {
            first: 0x00,
            second: 0x11223344,
        },
        list: ListContainer(vec![0xaabbccdd, 0x55667788]),
    };

    assert_eq!(result, expected);
}

mod padding {
    use super::*;

    #[derive(Debug, Default, PartialEq, no_std_io::EndianRead, no_std_io::EndianWrite)]
    struct NestedContainer<T: no_std_io::EndianRead + no_std_io::EndianWrite> {
        first: u32,
        data: T,
    }

    #[derive(Debug, PartialEq, no_std_io::EndianRead, no_std_io::EndianWrite)]
    struct PaddedTest {
        #[no_std_io(pad_before = 1)]
        first: u8,
        #[no_std_io(pad_before = 2)]
        second: u32,
    }

    #[test]
    fn should_read_le() {
        let bytes = vec![0x00, 0xaa, 0x00, 0x00, 0xee, 0xdd, 0xcc, 0xbb];
        let result: PaddedTest = bytes.read_le(0).expect("Read should have worked");
        let expected = PaddedTest {
            first: 0xaa,
            second: 0xbbccddee,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn should_read_be() {
        let bytes = vec![0x00, 0xaa, 0x00, 0x00, 0xbb, 0xcc, 0xdd, 0xee];
        let result: PaddedTest = bytes.read_be(0).expect("Read should have worked");
        let expected = PaddedTest {
            first: 0xaa,
            second: 0xbbccddee,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn should_read_nested_le() {
        let bytes = vec![
            0x44, 0x33, 0x22, 0x11, 0x00, 0xaa, 0x00, 0x00, 0xee, 0xdd, 0xcc, 0xbb,
        ];
        let result: NestedContainer<PaddedTest> =
            bytes.read_le(0).expect("Read should have worked");
        let expected = NestedContainer {
            first: 0x11223344,
            data: PaddedTest {
                first: 0xaa,
                second: 0xbbccddee,
            },
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn should_read_nested_be() {
        let bytes = vec![
            0x11, 0x22, 0x33, 0x44, 0x00, 0xaa, 0x00, 0x00, 0xbb, 0xcc, 0xdd, 0xee,
        ];
        let result: NestedContainer<PaddedTest> =
            bytes.read_be(0).expect("Read should have worked");
        let expected = NestedContainer {
            first: 0x11223344,
            data: PaddedTest {
                first: 0xaa,
                second: 0xbbccddee,
            },
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn should_read_dynamic_size_le() {
        let bytes = vec![
            0x02, 0x00, 0xaa, 0x00, 0x00, 0xee, 0xdd, 0xcc, 0xbb, 0x00, 0x11, 0x00, 0x00, 0x55,
            0x44, 0x33, 0x22,
        ];
        let result: ListContainer<PaddedTest> = bytes.read_le(0).expect("Write should have worked");
        let expected = ListContainer::<PaddedTest>(vec![
            PaddedTest {
                first: 0xaa,
                second: 0xbbccddee,
            },
            PaddedTest {
                first: 0x11,
                second: 0x22334455,
            },
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn should_read_dynamic_size_be() {
        let bytes = vec![
            0x02, 0x00, 0xaa, 0x00, 0x00, 0xbb, 0xcc, 0xdd, 0xee, 0x00, 0x11, 0x00, 0x00, 0x22,
            0x33, 0x44, 0x55,
        ];
        let result: ListContainer<PaddedTest> = bytes.read_be(0).expect("Write should have worked");
        let expected = ListContainer::<PaddedTest>(vec![
            PaddedTest {
                first: 0xaa,
                second: 0xbbccddee,
            },
            PaddedTest {
                first: 0x11,
                second: 0x22334455,
            },
        ]);
        assert_eq!(result, expected);
    }
}
