pub trait FromRef<T>: Sized {
    fn from_ref(_: &T) -> Self;
}

impl<T: Clone, U: From<T>> FromRef<T> for U {
    #[inline]
    fn from_ref(val_ref: &T) -> Self {
        let val = val_ref.clone();
        Self::from(val)
    }
}

pub trait RefInto<T: Sized>: Clone {
    fn ref_into(&self) -> T;
}

impl<T, U: Into<T> + Clone> RefInto<T> for U {
    #[inline]
    fn ref_into(&self) -> T {
        let val = self.clone();
        val.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    #[allow(unused_must_use)]
    fn from_ref() {
        let a = 0u8;

        u16::from(a);

        u16::from_ref(&a);

        let mut a = 0u8;

        u16::from_ref(&mut a);
    }

    // TODO more tests
}
