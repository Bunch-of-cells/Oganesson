#![allow(confusable_idents)]
#![allow(uncommon_codepoints)]

#[cfg(feature = "simulation")]
extern crate piston_window;

pub mod collision;
pub mod object;
pub mod quantity;
#[cfg(feature = "simulation")]
pub mod simulation;
pub mod transform;
pub mod universe;

pub use collision::Collider;
pub use object::{IntrinsicProperty, Object, ObjectAttributes, ObjectID};
pub use quantity::*;
pub use transform::{ObjectShape, Transform};
pub use universe::Universe;
