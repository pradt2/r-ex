pub trait EndianAs<T> {
    fn as_be(&self) -> T;
    fn as_le(&self) -> T;
}

pub trait EndianFromRef<T, U> {
    fn from_be_ref(buf: &U) -> T;
    fn from_le_ref(buf: &U) -> T;
}

pub trait EndianSet<T> {
    fn set_be(&mut self, val: T);
    fn set_le(&mut self, val: T);
}

macro_rules! impl_endian_traits {
    ($typ:ty) => {
        impl EndianAs<$typ> for [u8; core::mem::size_of::<$typ>()] {
            #[inline]
            fn as_be(&self) -> $typ { <$typ>::from_be_bytes(*self) }

            #[inline]
            fn as_le(&self) -> $typ { <$typ>::from_le_bytes(*self) }
        }

        impl EndianFromRef<$typ, [u8; core::mem::size_of::<$typ>()]> for $typ {
            #[inline]
            fn from_be_ref(buf: &[u8; core::mem::size_of::<$typ>()]) -> $typ { <$typ>::from_be_bytes(*buf) }

            #[inline]
            fn from_le_ref(buf: &[u8; core::mem::size_of::<$typ>()]) -> $typ { <$typ>::from_le_bytes(*buf) }
        }

        impl EndianSet<$typ> for [u8; core::mem::size_of::<$typ>()] {
            #[inline]
            fn set_be(&mut self, val: $typ) { self.copy_from_slice(&val.to_be_bytes()); }

            #[inline]
            fn set_le(&mut self, val: $typ) { self.copy_from_slice(&val.to_le_bytes()); }
        }
    };
}

impl_endian_traits!(u16);
impl_endian_traits!(u32);
impl_endian_traits!(u64);
impl_endian_traits!(u128);

impl_endian_traits!(i16);
impl_endian_traits!(i32);
impl_endian_traits!(i64);
impl_endian_traits!(i128);

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    macro_rules! test_endian_traits {
        ($typ:ty, $typ_ident:ident) => {

            #[test]
            fn $typ_ident() {
                let mut buf = [0u8; core::mem::size_of::<$typ>()];

                buf.set_be(1 as $typ);
                assert_eq!((1 as $typ).to_be_bytes(), buf);
                assert_eq!((1 as $typ), buf.as_be());
                assert_eq!((1 as $typ), <$typ>::from_be_ref(&buf));
                assert_eq!((1 as $typ), <$typ>::from_be_ref(&mut buf));

                buf.set_le(1 as $typ);
                assert_eq!((1 as $typ).to_le_bytes(), buf);
                assert_eq!((1 as $typ), buf.as_le());
                assert_eq!((1 as $typ), <$typ>::from_le_ref(&buf));
                assert_eq!((1 as $typ), <$typ>::from_le_ref(&mut buf));
            }
        }
    }

    test_endian_traits!(u16, u16);
    test_endian_traits!(u32, u32);
    test_endian_traits!(u64, u64);
    test_endian_traits!(u128, u128);

    test_endian_traits!(i16, i16);
    test_endian_traits!(i32, i32);
    test_endian_traits!(i64, i64);
    test_endian_traits!(i128, i128);
}
