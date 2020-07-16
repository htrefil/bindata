use binser::{Decode, Encode, Size};

#[derive(Encode, Decode, PartialEq, Debug, Size)]
struct UnnamedStruct(i8, u8, i16, u16, i32, u32, i64, u64);

#[derive(Encode, Decode, PartialEq, Debug, Size)]
struct NamedStruct {
    a: i8,
    b: u8,
    c: i16,
    d: u16,
    e: i32,
    f: u32,
    g: i64,
    h: u64,
}

#[derive(Encode, Decode, PartialEq, Debug, Size)]
struct UnitStruct;

#[derive(Encode, Decode, Debug, PartialEq, Size)]
#[repr(i8)]
enum Enum {
    First = 1,
    Second = 2,
}

#[derive(Encode, Decode, PartialEq, Debug, Size)]
struct Generic<T> {
    a: T,
}

#[cfg(test)]
mod tests {
    use super::*;
    use binser::{Reader, Writer};

    #[test]
    fn test_size() {
        const SIZE: usize = i8::SIZE
            + u8::SIZE
            + i16::SIZE
            + u16::SIZE
            + i32::SIZE
            + u32::SIZE
            + i64::SIZE
            + u64::SIZE;

        assert_eq!(UnnamedStruct::SIZE, SIZE);
        assert_eq!(NamedStruct::SIZE, SIZE);
        assert_eq!(UnitStruct::SIZE, 0);
        assert_eq!(Enum::SIZE, i8::SIZE);
        assert_eq!(Generic::<u32>::SIZE, u32::SIZE);
    }

    #[test]
    fn test_serialize() {
        let mut writer = Writer::new();
        writer.write(UnnamedStruct(0, 1, 2, 3, 4, 5, 6, 7));
        writer.write(NamedStruct {
            a: 0,
            b: 1,
            c: 2,
            d: 3,
            e: 4,
            f: 5,
            g: 6,
            h: 7,
        });
        writer.write(UnitStruct);
        writer.write(Enum::First);
        writer.write(Generic::<u32> { a: 0 });

        let data = writer.data();
        let mut reader = Reader::new(&data);
        assert_eq!(
            reader.read::<UnnamedStruct>().unwrap(),
            UnnamedStruct(0, 1, 2, 3, 4, 5, 6, 7)
        );
        assert_eq!(
            reader.read::<NamedStruct>().unwrap(),
            NamedStruct {
                a: 0,
                b: 1,
                c: 2,
                d: 3,
                e: 4,
                f: 5,
                g: 6,
                h: 7,
            }
        );
        assert_eq!(reader.read::<UnitStruct>().unwrap(), UnitStruct);
        assert_eq!(reader.read::<Enum>().unwrap(), Enum::First);
        assert_eq!(reader.read::<Generic<u32>>().unwrap(), Generic { a: 0 });
    }
}
