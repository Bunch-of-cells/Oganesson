#![allow(confusable_idents)]

mod collision;
mod object;
mod quantity;
mod world;

pub use collision::{Collider, Quaternion, Transform};
pub use object::{Object, ObjectAttributes};
pub use quantity::*;
pub use world::PhysicsWorld;

type Float = f32;
