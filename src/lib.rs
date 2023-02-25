#![allow(confusable_idents)]
#![allow(uncommon_codepoints)]

#[cfg(feature = "simulation")]
extern crate piston_window;

pub mod collision;
pub mod object;
pub mod quantity;
#[cfg(feature = "simulation")]
pub mod simulation;
pub mod universe;

pub use collision::{Collider, Quaternion, Transform};
pub use object::{IntrinsicProperty, Object, ObjectAttributes};
pub use quantity::*;
pub use universe::Universe;
