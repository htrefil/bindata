use std::mem;

/// A trait for values with a known encoded size at compile time.
///
/// For all types implementin this trait, the following invariant must always be true:
/// ```
/// let mut writer = Writer::new();
/// writer.write(Self);
/// assert_eq!(writer.data().len(), Self::SIZE);
/// ```
pub trait Size: Sized {
    const SIZE: usize;
}

macro_rules! impl_size_number {
	($($ty:ty)*) => {
        $(impl Size for $ty {
            const SIZE: usize = mem::size_of::<$ty>();
        })*
    };
}

impl_size_number!(i8 u8 i16 u16 i32 u32 i64 u64 f32 f64);

macro_rules! impl_size_array {
    ($($length:expr)*) => {
        $(impl<T> Size for [T; $length]
        where
            T: Size,
        {
            const SIZE: usize = T::SIZE * $length;
        })*
    };
}

impl_size_array!(1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);
