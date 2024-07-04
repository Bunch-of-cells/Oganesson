#![allow(confusable_idents)]
#![allow(uncommon_codepoints)]

mod collision;
mod object;
mod quantity;
mod universe;

pub use collision::Collider;
pub use object::{IntrinsicProperty, Object, ObjectAttributes, ObjectBuilder, ObjectID};
pub use quantity::*;
pub use universe::Universe;

pub const STEP: Float = 1e-4;
