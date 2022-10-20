use std::ops::{Deref, DerefMut};

use ggez::graphics::{self, Color, DrawMode, Mesh};
use ggez::input::keyboard::is_key_pressed;
use ggez::timer::delta;
use ggez::winit::event::VirtualKeyCode;
use ggez::{event::EventHandler, Context, GameResult};

use crate::Collider;

#[derive(Default)]
pub struct Universe {
    universe: crate::Universe<2>,
    paused: bool,
}

impl Universe {
    pub fn new() -> Universe {
        Self {
            universe: crate::Universe::new(),
            paused: false,
        }
    }
}

impl Deref for Universe {
    type Target = crate::Universe<2>;
    fn deref(&self) -> &Self::Target {
        &self.universe
    }
}

impl DerefMut for Universe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.universe
    }
}

impl EventHandler for Universe {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.paused = is_key_pressed(ctx, VirtualKeyCode::Space);
        if !self.paused {
            self.universe.step(delta(ctx).as_secs_f32());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.paused {
            return Ok(());
        }
        graphics::clear(ctx, Color::WHITE);

        for object in self.objects() {
            println!("{:?}", object);
            let color = object.color();
            let mesh = match object.collider() {
                &Collider::Sphere { radius } => {
                    Mesh::new_circle(ctx, DrawMode::fill(), [0.0, 0.0], *radius, 0.1, color)?
                }
                &Collider::Triangle { a, b, c } => Mesh::from_triangles(ctx, &[a, b, c], color)?,
                Collider::Plane { .. } => todo!(),
                Collider::Polygon { points } => {
                    Mesh::new_polygon(ctx, DrawMode::fill(), points, color)?
                }
            };
            graphics::draw(ctx, &mesh, (object.position(),))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
