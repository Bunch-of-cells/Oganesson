use macroquad::prelude::*;
use oganesson::*;

#[macroquad::main("Oganesson")]
async fn main() {
    let mut universe = Universe::<2>::new();

    universe.add_object(
        ObjectBuilder::new_at([200.0, 200.0] * units::m)
            .with_size(20.0 * units::m)
            .with_mass(500.0 * units::kg)
            .with_color(RED)
            .build()
            .unwrap(),
    );

    universe.add_object(
        ObjectBuilder::new_at([500.0, 200.0] * units::m)
            .with_size(20.0 * units::m)
            .with_mass(500.0 * units::kg)
            .with_color(BLUE)
            .build()
            .unwrap(),
    );

    let mut last_update = get_time() as Float;

    loop {
        clear_background(GRAY);

        let dt = get_time() as Float - last_update;
        last_update = get_time() as Float;
        universe.step(dt);

        for obj in universe.objects() {
            draw_circle(
                obj.position()[0],
                obj.position()[1],
                obj.size().into(),
                obj.color(),
            );
        }

        next_frame().await
    }
}
