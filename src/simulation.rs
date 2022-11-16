use std::ops::{Deref, DerefMut};

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::MouseButton;
pub use ggez::graphics::Color;
use ggez::graphics::{Canvas, DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::winit::event::VirtualKeyCode;
use ggez::{event, ContextBuilder};
use ggez::{event::EventHandler, Context, GameResult};

use crate::field::VectorField;
use crate::{units, Collider, Float, IntrinsicProperty, Object, Scalar, Vector};

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

    pub fn run(self) -> GameResult<()> {
        let (ctx, event_loop) = ContextBuilder::new("oganesson", "Bunch-of-cells")
            .window_setup(WindowSetup::default().title("Oganesson"))
            .window_mode(WindowMode {
                height: 800.0,
                width: 1000.0,
                ..Default::default()
            })
            .build()?;

        event::run(ctx, event_loop, self)
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

        let c: Option<Float> = if ctx.mouse.button_just_pressed(MouseButton::Left) {
            Some(5e-3)
        } else if ctx.mouse.button_just_pressed(MouseButton::Right) {
            Some(-5e-3)
        } else {
            None
        };

        if let Some(c) = c {
            self.universe.add_object(
                Object::new(
                    Vector::from(ctx.mouse.position()) * units::m,
                    Vector([0.0, 0.0], units::m / units::s),
                    IntrinsicProperty::new(
                        Scalar(1.0, units::kg),
                        Collider::Sphere {
                            radius: Scalar(20.0, units::m),
                        },
                        if c.is_sign_negative() {
                            Color::RED
                        } else {
                            Color::BLUE
                        },
                    )
                    .unwrap()
                    .with_charge(Scalar(c, units::C))
                    .unwrap(),
                )
                .unwrap(),
            );
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        let mb = &mut MeshBuilder::new();

        self.draw_bodies(mb)?;
        self.draw_field(mb, ctx)?;

        let mesh = Mesh::from_data(ctx, mb.build());
        canvas.draw(&mesh, DrawParam::new());
        canvas.finish(ctx)
    }
}
