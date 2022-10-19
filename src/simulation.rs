use std::ops::{Deref, DerefMut};

use ggez::graphics::{self, Color, DrawMode, Mesh};
use ggez::timer::delta;
use ggez::{event::EventHandler, Context, GameResult};

use crate::Collider;

#[derive(Default)]
pub struct Universe {
    universe: crate::Universe<2>,
}

impl Universe {
    pub fn new() -> Universe {
        Self {
            universe: crate::Universe::new(),
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
        self.universe.step(delta(ctx).as_secs_f32());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        for object in self.objects() {
            let color = object.color();
            let mesh = match object.collider() {
                &Collider::Sphere { radius } => {
                    Mesh::new_circle(ctx, DrawMode::fill(), [0.0, 0.0], *radius, 0.5, color)?
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
