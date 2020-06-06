use std::mem;

pub struct Writer {
    data: Vec<u8>,
}

impl Writer {
    pub fn new() -> Writer {
        Writer { data: Vec::new() }
    }

    pub fn write_bytes(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data)
    }

    pub fn write_bytes_at(&mut self, data: &[u8], offset: usize) {
        while self.data.len() < data.len() + offset {
            self.data.push(0);
        }

        for i in 0..data.len() {
            self.data[offset + i] = data[i];
        }
    }

    pub fn write_at<T>(&mut self, data: T, offset: usize)
    where
        T: Encode,
    {
        let mut writer = Writer { data: Vec::new() };
        writer.write(data);

        self.write_bytes_at(&writer.data, offset);
    }

    pub fn write<T>(&mut self, data: T)
    where
        T: Encode,
    {
        data.encode(self);
    }

    pub fn data(self) -> Vec<u8> {
        self.data
    }
}

pub trait Encode: Sized {
    fn encode(self, writer: &mut Writer);

    fn encode_in_place(self) -> Vec<u8> {
        let mut writer = Writer::new();
        writer.write(self);

        writer.data()
    }
}

macro_rules! impl_encode_integer {
    ($($ty:ty)*) => {
        $(impl Encode for $ty {
            fn encode(self, writer: &mut Writer) {
            	writer.write_bytes(&self.to_le_bytes());
            }
        })*
    };
}

impl_encode_integer!(i8 u8 i16 u16 i32 u32 i64 u64);

pub trait EncodedSize: Sized {
    const SIZE: usize;
}

macro_rules! impl_encoded_size {
	($($ty:ty)*) => {
        $(impl EncodedSize for $ty {
            const SIZE: usize = mem::size_of::<$ty>();
        })*
    };
}

impl_encoded_size!(i8 u8 i16 u16 i32 u32 i64 u64);

macro_rules! impl_encode_array {
    ($($length:expr)*) => {
        $(impl<T> Encode for [T; $length]
        where
            T: Encode + Clone + Copy,
        {
            fn encode(self, writer: &mut Writer)   {
                for elem in self.iter() {
                    writer.write(*elem);
                }
            }
        })*
    };
}

impl_encode_array!(1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);

macro_rules! impl_encoded_size_array {
    ($($length:expr)*) => {
        $(impl<T> EncodedSize for [T; $length]
        where
            T: EncodedSize,
        {
            const SIZE: usize = T::SIZE * $length;
        })*
    };
}

impl_encoded_size_array!(1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);
