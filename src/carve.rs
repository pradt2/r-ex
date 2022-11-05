pub trait Carve<T: Sized + Copy> {
    fn carve<const N: usize>(&self, offset: usize) -> Option<&[T; N]>;
    fn carve_mut<const N: usize>(&mut self, offset: usize) -> Option<&mut [T; N]>;
}

impl<T: Sized + Copy> Carve<T> for [T] {
    #[inline]
    fn carve<const N: usize>(&self, offset: usize) -> Option<&[T; N]> {
        self.get(offset..offset + N)?.try_into().ok()
    }

    #[inline]
    fn carve_mut<const N: usize>(&mut self, offset: usize) -> Option<&mut [T; N]> {
        self.get_mut(offset..offset + N)?.try_into().ok()
    }
}

pub trait Carved<T: Sized + Copy> {
    fn carved<const N: usize>(&self) -> Option<&[T; N]>;
    fn carved_mut<const N: usize>(&mut self) -> Option<&mut [T; N]>;
}

impl<T: Sized + Copy> Carved<T> for [T] {
    #[inline]
    fn carved<const N: usize>(&self) -> Option<&[T; N]> {
        self.carve(0)
    }

    #[inline]
    fn carved_mut<const N: usize>(&mut self) -> Option<&mut [T; N]> {
        self.carve_mut(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn carve() {
        let mut buf = [1u8, 2, 3, 4];

        assert_eq!(&[1, 2, 3, 4], buf.carve(0).unwrap());
        assert_eq!(&[3, 4], buf.carve(2).unwrap());
        assert_eq!(&[] as &[u8; 0], buf.carve(4).unwrap());
        assert!(buf.carve::<5>(0).is_none());

        assert_eq!(&[1, 2, 3, 4], buf.carve_mut(0).unwrap());
        assert_eq!(&[3, 4], buf.carve_mut(2).unwrap());
        assert_eq!(&[] as &[u8; 0], buf.carve_mut(4).unwrap());
        assert!(buf.carve_mut::<5>(0).is_none());
    }

    #[test]
    fn carved() {
        let mut buf = [1u8, 2, 3, 4];

        assert_eq!(&[1, 2, 3, 4], buf.carved().unwrap());
        assert_eq!(&[1, 2], buf.carved().unwrap());
        assert_eq!(&[] as &[u8; 0], buf.carved().unwrap());
        assert!(buf.carved::<5>().is_none());

        assert_eq!(&[1, 2, 3, 4], buf.carved_mut().unwrap());
        assert_eq!(&[1, 2], buf.carved_mut().unwrap());
        assert_eq!(&[] as &[u8; 0], buf.carved_mut().unwrap());
        assert!(buf.carved_mut::<5>().is_none());
    }
}
