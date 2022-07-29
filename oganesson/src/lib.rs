#![allow(confusable_idents)]

mod collision;
mod quantity;
mod world;

pub use collision::{Collider, Quaternion, Transform};
pub use quantity::*;
pub use world::{Object, PhysicsWorld};

type Float = f32;

// struct Solver {
//     f: fn(f32, f32) -> f32,
//     t: f32,
//     y: f32,
//     h: f32,
// }

// impl Solver {
//     fn solve_rk4(&mut self) -> f32 {
//         let k1 = (self.f)(self.t, self.y);
//         let k2 = (self.f)(self.t + self.h / 2.0, self.y + self.h * k1 / 2.0);
//         let k3 = (self.f)(self.t + self.h / 2.0, self.y + self.h * k2 / 2.0);
//         let k4 = (self.f)(self.t + self.h, self.y + self.h * k3);
//         self.y = (k1 + 2.0 * k2 + 3.0 * k3 + k4) * self.h / 6.0;
//         self.t += self.h;
//         self.round_if_near();
//         self.y
//     }

//     fn round_if_near(&mut self) {
//         if self.y % 1.0 < self.h || self.y % 1.0 > 1.0 - self.h {
//             self.y = self.y.round();
//         }
//     }
// }
