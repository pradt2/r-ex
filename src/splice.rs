pub trait Splice<T: Sized> {
    fn splice<const N: usize>(&self) -> Option<(&[T; N], &[T])>;
    fn splice_mut<const N: usize>(&mut self) -> Option<(&mut [T; N], &mut [T])>;
}

impl<T: Sized> Splice<T> for [T] {
    #[inline]
    fn splice<const N: usize>(&self) -> Option<(&[T; N], &[T])> {
        if self.len() < N { return None; }
        let (head, tail) = self.split_at(N);
        Some((head.try_into().ok()?, tail))
    }

    #[inline]
    fn splice_mut<const N: usize>(&mut self) -> Option<(&mut [T; N], &mut [T])> {
        if self.len() < N { return None; }
        let (head, tail) = self.split_at_mut(N);
        Some((head.try_into().ok()?, tail))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn splice() {
        let mut buf = [0u8; 2];

        let (head, tail) = buf.splice::<0>().unwrap();
        assert_eq!(0, head.len());
        assert_eq!(2, tail.len());

        let (head, tail) = buf.splice::<2>().unwrap();
        assert_eq!(2, head.len());
        assert_eq!(0, tail.len());

        let x = buf.splice::<4>();
        assert_eq!(None, x);

        let (head, tail) = buf.splice_mut::<0>().unwrap();
        assert_eq!(0, head.len());
        assert_eq!(2, tail.len());

        let (head, tail) = buf.splice_mut::<2>().unwrap();
        assert_eq!(2, head.len());
        assert_eq!(0, tail.len());

        let x = buf.splice_mut::<4>();
        assert_eq!(None, x);
    }
}
