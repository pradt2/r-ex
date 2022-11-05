pub trait FromArr<'a, T: 'a, const N: usize, U: 'a> {
    fn from_arr(_: &'a [T; N]) -> U;
}

impl<'a, T: 'a, const N: usize, U: From<&'a [T]> + 'a> FromArr<'a, T, N, U> for U {
    #[inline]
    fn from_arr(buf: &'a [T; N]) -> U {
        U::from(buf.as_slice())
    }
}

pub trait FromArrMut<'a, T: 'a, const N: usize, U: 'a> {
    fn from_arr_mut(_: &'a mut [T; N]) -> U;
}

impl<'a, T: 'a, const N: usize, U: From<&'a mut [T]> + 'a> FromArrMut<'a, T, N, U> for U {
    #[inline]
    fn from_arr_mut(buf: &'a mut [T; N]) -> U {
        U::from(buf.as_mut_slice())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn from_arr() {
        struct BufRef<'a> {
            #[allow(dead_code)]
            buf: &'a [u8],
        }

        impl<'a> From<&'a [u8]> for BufRef<'a> {
            fn from(buf: &'a [u8]) -> Self {
                Self {
                    buf
                }
            }
        }

        let _ = BufRef::from_arr(&[0u8; 2]);

        struct BufRefMut<'a> {
            #[allow(dead_code)]
            buf: &'a mut [u8],
        }

        impl<'a> From<&'a mut [u8]> for BufRefMut<'a> {
            fn from(buf: &'a mut [u8]) -> Self {
                Self {
                    buf
                }
            }
        }

        let mut buf = [0u8; 2];
        let _ = BufRefMut::from_arr_mut(&mut buf);
    }
}