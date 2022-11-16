use std::ops::{Deref, DerefMut};

use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::winit::event::VirtualKeyCode;
use ggez::{event::EventHandler, Context, GameResult};

use crate::field::VectorField;
use crate::{units, Collider, Vector};

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

    fn draw_bodies(&self, mb: &mut MeshBuilder) -> GameResult {
        for object in self.objects() {
            // println!("{:?}", object);
            let color = object.color();
            match object.collider() {
                &Collider::Sphere { radius } => {
                    mb.circle(DrawMode::fill(), object.position(), *radius, 0.1, color)?;
                }
                &Collider::Triangle { a, b, c } => {
                    mb.triangles(&[a, b, c], color)?;
                }
                Collider::Plane { .. } => todo!(),
                Collider::Polygon { points } => {
                    mb.polygon(DrawMode::fill(), points, color)?;
                }
            };
        }
        Ok(())
    }

    fn draw_field(&self, mb: &mut MeshBuilder, ctx: &mut Context) -> GameResult {
        let (w, h) = ctx.gfx.size();

        let field = self.universe.electric_field();

        for i in (0..w as u32).step_by(50) {
            for j in (0..h as u32).step_by(50) {
                self.draw_field_arrow(mb, &field, i as f32, j as f32, Color::WHITE, 5000.0)?;
            }
        }
        Ok(())
    }

    fn draw_field_arrow(
        &self,
        mb: &mut MeshBuilder,
        field: &VectorField<'_, 2>,
        x: f32,
        y: f32,
        color: Color,
        factor: f32,
    ) -> GameResult {
        let g = field.at(Vector([x, y], units::m)).unwrap();

        let p = if g.magnitude().is_zero() || g.0.iter().any(|x| x.is_nan()) {
            Vector([x, y], g.unit())
        } else {
            g.normalized() * (g.magnitude() / factor).log2().atan() * 25.0 + Vector::from([x, y])
        };

        mb.line(&[[x, y].into(), p], 1.0, color)?;
        mb.circle(DrawMode::fill(), p, 5.0, 1.0, color)?;
        Ok(())
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
        self.paused = !ctx.keyboard.is_key_pressed(VirtualKeyCode::Space);
        if !self.paused {
            self.universe.step(ctx.time.delta().as_secs_f32());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        let mb = &mut MeshBuilder::new();
        self.draw_field(mb, ctx)?;
        self.draw_bodies(mb)?;

        let mesh = Mesh::from_data(ctx, mb.build());
        canvas.draw(&mesh, DrawParam::new());
        canvas.finish(ctx)
    }
}
