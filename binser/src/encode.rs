use std::mem;

/// A trait representing values that can be read from a `Reader`.
pub struct Writer {
    data: Vec<u8>,
}

impl Writer {
    /// Creates a new `Writer` instance with an empty buffer.
    pub fn new() -> Writer {
        Writer { data: Vec::new() }
    }

    /// Creates a new `Writer` instance with an empty buffer and a capacity.
    pub fn with_capacity(capacity: usize) -> Writer {
        Writer {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Writes bytes at the end of the buffer.
    pub fn write_bytes(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data)
    }

    /// Writes bytes at a specified offset in the buffer.
    ///
    /// If offset is larger than the buffers size, the empty space will be padded with zeroes.
    pub fn write_bytes_at(&mut self, data: &[u8], offset: usize) {
        while self.data.len() < data.len() + offset {
            self.data.push(0);
        }

        for i in 0..data.len() {
            self.data[offset + i] = data[i];
        }
    }

    /// Writes a value implementing `Encode` at a specified offset in the buffer.
    ///
    /// If offset is larger than the buffers size, the empty space will be padded with zeroes.
    pub fn write_at<T>(&mut self, data: T, offset: usize)
    where
        T: Encode,
    {
        let mut writer = Writer { data: Vec::new() };
        writer.write(data);

        self.write_bytes_at(&writer.data, offset);
    }

    /// Writes a value implementing `Encode` at the end of the buffer.
    pub fn write<T>(&mut self, data: T)
    where
        T: Encode,
    {
        data.encode(self);
    }

    /// Returns the encoded data.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Transfers ownership of the encoded data.
    pub fn finish(self) -> Vec<u8> {
        self.data
    }

    /// Clears the writers internal buffer while retaining capacity to allow for more efficient memory reuse.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

/// A trait representing values that can be written to a `Writer`.
pub trait Encode: Sized {
    fn encode(self, writer: &mut Writer);

    /// Creates a temporary `Writer`, writes itself to it and returns its data.
    ///
    /// Convenience method.
    fn encode_in_place(self) -> Vec<u8> {
        let mut writer = Writer::new();
        writer.write(self);

        writer.finish()
    }
}

macro_rules! impl_encode_number {
    ($($ty:ty)*) => {
        $(impl Encode for $ty {
            fn encode(self, writer: &mut Writer) {
            	writer.write_bytes(&self.to_le_bytes());
            }
        })*
    };
}

impl_encode_number!(i8 u8 i16 u16 i32 u32 i64 u64 f32 f64);

/// A trait for values with a known encoded size at compile time.
///
/// For all types implementin this trait, the following invariant must always be true:
/// ```
/// let mut writer = Writer::new();
/// writer.write(Self);
/// assert_eq!(writer.data().len(), Self::SIZE);
/// ```
pub trait EncodedSize: Sized {
    const SIZE: usize;
}

macro_rules! impl_encoded_size_number {
	($($ty:ty)*) => {
        $(impl EncodedSize for $ty {
            const SIZE: usize = mem::size_of::<$ty>();
        })*
    };
}

impl_encoded_size_number!(i8 u8 i16 u16 i32 u32 i64 u64 f32 f64);

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
