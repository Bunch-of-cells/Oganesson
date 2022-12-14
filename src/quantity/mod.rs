pub mod consts;
pub mod field;
pub mod scalar;
pub mod unit;
pub mod vector;

pub use consts::*;
pub use scalar::Scalar;
pub use vector::Vector;

pub type Float = f32;
pub const STEP: Float = 1e-3;

#[macro_export]
macro_rules! c {
    ($(#[$attr:meta])* ($($vis:tt)*) const $N:ident : $T:ty = $e:expr;) => {
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[inline(always)]
        $(#[$attr])*
        $($vis)* fn $N() -> $T {
            $e
        }
    };
    ($(#[$attr:meta])* pub const $N:ident : $T:ty = $e:expr;) => {
        c!($($attr)* (pub) const $N : $T = $e)
    };
    ($(#[$attr:meta])* const $N:ident : $T:ty = $e:expr;) => {
        c!($($attr)* () const $N : $T = $e)
    };
    ($($(#[$attr:meta])* pub const $N:ident : $T:ty = $e:expr;)*) => {
        $($crate::c!($(#[$attr])* (pub) const $N : $T = $e;);)*
    };
    () => ()
}
