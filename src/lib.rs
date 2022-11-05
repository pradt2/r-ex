#![no_std]

mod carve;
mod copy_from;
mod endian;
mod from_arr;
mod from_ref;
mod splice;

pub mod prelude {
    #[cfg(feature = "carve")]
    pub use crate::carve::*;

    #[cfg(feature = "copy-from")]
    pub use crate::copy_from::*;

    #[cfg(feature = "endian")]
    pub use crate::endian::*;

    #[cfg(feature = "from-arr")]
    pub use crate::from_arr::*;

    #[cfg(feature = "from-ref")]
    pub use crate::from_ref::*;

    #[cfg(feature = "splice")]
    pub use crate::splice::*;
}
