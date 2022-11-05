pub trait CopyFrom<T: Sized + Copy, const N: usize> {
    fn copy_from(&mut self, src: &[T; N]);
}

impl<T: Sized + Copy, const N: usize> CopyFrom<T, N> for [T; N] {
    #[inline]
    fn copy_from(&mut self, src: &Self) {
        self.copy_from_slice(src);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn copy_from() {
        let mut buf = [0u8; 4];
        let val1 = [1u8; 4];

        buf.copy_from(&val1);
        assert_eq!(&buf, &val1);
    }
}
