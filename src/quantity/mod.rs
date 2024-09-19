pub mod consts;
pub mod dimension;
pub mod field;
pub mod scalar;
// pub mod tensor;
pub mod vector;

pub use consts::*;
pub use field::{ScalarField, VectorField};
pub use scalar::Scalar;
// pub use tensor::Tensor;
pub use vector::Vector;

pub type Float = f32;
pub use std::f32::{
    consts::{E, PI},
    EPSILON,
};

use super::STEP;

pub struct DiffSolver<I: Fn(Float, Float, Float) -> Float> {
    a: I,
    t: Float,
    x: Float,
    v: Float,
}

impl<I: Fn(Float, Float, Float) -> Float> DiffSolver<I> {
    pub fn new(a: I, t: Float, x: Float, v: Float) -> Self {
        Self { a, t, x, v }
    }
}

impl<I: Fn(Float, Float, Float) -> Float> Iterator for DiffSolver<I> {
    type Item = (Float, Float, Float);

    fn next(&mut self) -> Option<Self::Item> {
        let (t, x, v) = (self.t, self.x, self.v);
        let k0 = STEP * v;
        let l0 = STEP * (self.a)(t, x, v);
        let k1 = STEP * (v + l0 / 2.0);
        let l1 = STEP * (self.a)(t + STEP / 2.0, x + k0 / 2.0, v + l0 / 2.0);
        let k2 = STEP * (v + l1 / 2.0);
        let l2 = STEP * (self.a)(t + STEP / 2.0, x + k1 / 2.0, v + l1 / 2.0);
        let k3 = STEP * (v + l2);
        let l3 = STEP * (self.a)(t + STEP, x + k2, v + l2);
        self.t = t + STEP;
        self.x = x + (k0 + 2.0 * k1 + 2.0 * k2 + k3) / 6.0;
        self.v = v + (l0 + 2.0 * l1 + 2.0 * l2 + l3) / 6.0;
        Some((t, x, v))
    }
}

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
        $crate::c!($(#[$attr])* (pub) const $N : $T = $e;);
    };
    ($(#[$attr:meta])* const $N:ident : $T:ty = $e:expr;) => {
        $crate::c!($(#[$attr])* () const $N : $T = $e;);
    };
    ($($(#[$attr:meta])* pub const $N:ident : $T:ty = $e:expr;)*) => {
        $($crate::c!($(#[$attr])* (pub) const $N : $T = $e;);)*
    };
    () => ()
}
