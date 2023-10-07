#![allow(confusable_idents)]
#![allow(uncommon_codepoints)]

extern crate piston_window;

pub mod object;
pub mod quantity;
pub mod simulation;
pub mod universe;

pub use object::{IntrinsicProperty, Object, ObjectAttributes, ObjectBuilder, ObjectID};
pub use quantity::*;
pub use universe::Universe;
