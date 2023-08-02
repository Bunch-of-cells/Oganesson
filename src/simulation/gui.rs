use std::{
    ops::{Deref, DerefMut},
    time::Instant,
};

use piston_window::*;

use super::color::*;
use crate::{field::VectorField, object::ObjectBuilder, units, ObjectShape, Scalar, Vector};

#[derive(Default)]
pub struct Universe {
    universe: crate::Universe<2>,
    paused: bool,
    mouse_pos: [f64; 2],
}

impl Universe {
    pub fn new() -> Universe {
        Self {
            universe: crate::Universe::new(),
            paused: false,
            mouse_pos: [0.0, 0.0],
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut window = WindowSettings::new("Oganesson", [1000, 800])
            .exit_on_esc(true)
            .build::<PistonWindow>()?
            .max_fps(60);

        let mut last_event = Instant::now();

        while let Some(event) = window.next() {
            if let Some([cx, cy]) = event.mouse_cursor_args() {
                self.mouse_pos = [cx, cy];
            }
            let dt = last_event.elapsed();
            self.update(&event, dt.as_secs_f64());
            window.draw_2d(&event, |ctx, gfx, _device| self.draw(ctx, gfx));

            last_event += dt;
        }

        Ok(())
    }

    fn update(&mut self, event: &Event, dt: f64) {
        let c = event.press_args().and_then(|press| match press {
            Button::Keyboard(Key::Space) => {
                self.paused = !self.paused;
                None
            }
            Button::Mouse(MouseButton::Left) => Some(5e-3f64),
            Button::Mouse(MouseButton::Right) => Some(-5e-3),
            _ => None,
        });
        if !self.paused {
            self.universe.step(dt);
        }

        if let Some(c) = c {
            self.universe.add_object(
                ObjectBuilder::new_at(Vector::from(self.mouse_pos.map(|a| a)) * units::m)
                    .with_shape(ObjectShape::Sphere {
                        radius: Scalar(20.0, units::m),
                    })
                    .with_color(if c.is_sign_negative() { BLUE } else { RED })
                    .with_charge(c * units::C)
                    .build()
                    .unwrap(),
            );
        }
    }

    fn draw(&mut self, ctx: Context, gfx: &mut G2d) {
        clear([0.0; 4], gfx);

        self.draw_field(&ctx, gfx);

        for object in self.objects() {
            let color = object.color();
            let pos = object.position();
            let transform = object.transform();
            match &transform.shape {
                &ObjectShape::Sphere { radius } => {
                    let r = (radius * transform.size).value();
                    let rect = [pos[0] - r, pos[1] - r, r * 2.0, r * 2.0].map(|a| a);
                    ellipse(color, rect, ctx.transform, gfx)
                }
                ObjectShape::Polygon { points } => polygon(
                    color,
                    points
                        .iter()
                        .map(|&x| (transform.rotation.rotate_vec(x * transform.size) + pos).into())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    ctx.transform,
                    gfx,
                ),
                ObjectShape::Point => (),
            };
        }
    }

    fn draw_field(&self, ctx: &Context, gfx: &mut G2d) {
        let [w, h] = ctx.get_view_size();

        let field = self.universe.electric_field();
        // let field = -field.gradient();

        for i in (0..w as u32).step_by(50) {
            for j in (0..h as u32).step_by(50) {
                self.draw_field_arrow(ctx, gfx, &field, i as f64, j as f64);
            }
        }
    }

    fn draw_field_arrow(
        &self,
        ctx: &Context,
        gfx: &mut G2d,
        field: &VectorField<'_, 2>,
        x: f64,
        y: f64,
    ) {
        let g = field.at(Vector([x, y], units::m)).unwrap();

        let p = if g.magnitude().is_zero() || g.0.iter().any(|x| x.is_nan()) {
            Vector([x, y], g.unit())
        } else {
            g.normalized() * (g.magnitude()).atan() * 10.0 + Vector::from([x, y])
        };

        line(WHITE, 1.0, [x, y, p[0], p[1]], ctx.transform, gfx);
        ellipse(
            WHITE,
            [p[0] - 5.0, p[1] - 5.0, 10.0, 10.0],
            ctx.transform,
            gfx,
        );
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
