use coffee::{
    graphics::{Color, Frame, Mesh, Point, Shape, Window},
    load::Task,
    Error, Game, Timer,
};
use oganesson::{units::*, Collider, Object, PhysicsWorld, Scalar, Vector};

fn main() -> Result<(), Error> {
    Simulation::run(coffee::graphics::WindowSettings {
        title: "Oganesson".to_string(),
        size: (1200, 800),
        resizable: false,
        fullscreen: false,
        maximized: false,
    })
}

struct Simulation {
    world: PhysicsWorld<2>,
}

impl Game for Simulation {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Self>
    where
        Self: Sized,
    {
        let mut world = PhysicsWorld::new(Vector::zero() * of_acceleration).unwrap();
        world.add_object(
            Object::new(
                Vector([0.0, 400.0], m),
                Vector([100.0, 0.0], m / s),
                Scalar(50.0, kg),
                Collider::Sphere {
                    radius: Scalar(100.0, m),
                },
            )
            .unwrap(),
        );
        world.add_object(
            Object::new(
                Vector([1200.0, 400.0], m),
                Vector([-100.0, 0.0], m / s),
                Scalar(500.0, kg),
                Collider::Sphere {
                    radius: Scalar(100.0, m),
                },
            )
            .unwrap(),
        );
        Task::succeed(|| Simulation { world })
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _timer: &Timer) {
        frame.clear(Color::BLACK);
        let mut target = frame.as_target();
        let mut mesh = Mesh::new();
        for object in self.world.objects() {
            match object.collider() {
                Collider::Sphere { radius } => {
                    mesh.fill(
                        Shape::Circle {
                            center: Point::from_slice(object.transform().position().as_slice()),
                            radius: radius.value(),
                        },
                        Color::GREEN,
                    );
                }
                Collider::Quad { .. } => {
                    todo!()
                }
                Collider::Polyline { .. } => {
                    todo!()
                }
            }
        }
        mesh.draw(&mut target);
    }

    fn update(&mut self, _window: &Window) {
        self.world.step((Self::TICKS_PER_SECOND as f32).recip());
    }
}
