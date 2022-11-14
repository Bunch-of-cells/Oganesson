use std::ops::{Deref, DerefMut};

use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder};
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
        self.paused = ctx.keyboard.is_key_pressed(VirtualKeyCode::Space);
        if !self.paused {
            self.universe.step(ctx.time.delta().as_secs_f32());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.paused {
            return Ok(());
        }

        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        let mb = &mut MeshBuilder::new();

        for object in self.objects() {
            // println!("{:?}", object);
            let color = object.color();
            match object.collider() {
                &Collider::Sphere { radius } => {
                    mb.circle(DrawMode::fill(), object.position(), *radius, 0.1, color)?
                }
                &Collider::Triangle { a, b, c } => mb.triangles(&[a, b, c], color)?,
                Collider::Plane { .. } => todo!(),
                Collider::Polygon { points } => mb.polygon(DrawMode::fill(), points, color)?,
            };
        }

        let mesh = Mesh::from_data(ctx, mb.build());
        canvas.draw(&mesh, DrawParam::new());
        canvas.finish(ctx)
    }
}
