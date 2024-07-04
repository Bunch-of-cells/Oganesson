use macroquad::prelude::*;
use oganesson::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Oganesson"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut universe = Universe::<2>::new();

    universe.add_object(
        ObjectBuilder::new_at([1260.0 / 2.0, 768.0 / 2.0] * units::m)
            .with_size(20.0 * units::m)
            .with_mass(1e6 * units::kg)
            .with_color(RED)
            .build()
            .unwrap(),
    );

    universe.add_object(
        ObjectBuilder::new_at([500.0, 768.0 / 2.0] * units::m)
            .with_size(10.0 * units::m)
            .with_mass(1e3 * units::kg)
            .with_velocity([0.0, 180.0] * units::m / units::s)
            .with_color(GREEN)
            .build()
            .unwrap(),
    );

    universe.add_object(
        ObjectBuilder::new_at([800.0, 768.0 / 2.0] * units::m)
            .with_size(50.0 * units::m)
            .with_mass(500.0 * units::kg)
            .with_velocity([0.0, -1800.0] * units::m / units::s)
            .with_color(BLUE)
            .build()
            .unwrap(),
    );

    let mut last_update = get_time() as Float;
    clear_background(GRAY);

    // let mut pos: Vec<Vector<2>> = vec![];

    loop {
        let dt = get_time() as Float - last_update;
        last_update = get_time() as Float;
        universe.step(dt);

        // let mut iter = pos.iter();
        // if let Some(last) = iter.next() {
        //     let mut last = last;
        //     let mut now = iter.next();
        //     while let Some(nows) = now {
        //         draw_line(last[0], last[1], nows[0], nows[1], 1.0, GREEN);
        //         last = nows;
        //         now = iter.next();
        //     }
        // }

        for obj in universe.objects() {
            draw_poly(
                obj.position()[0],
                obj.position()[1],
                50,
                obj.size().into(),
                0.,
                obj.color(),
            );
        }

        // draw_poly(
        //     universe.objects()[0].position()[0],
        //     universe.objects()[0].position()[1],
        //     50,
        //     universe.objects()[0].size().into(),
        //     0.,
        //     universe.objects()[0].color(),
        // );

        // pos.push(universe.objects()[1].position());

        next_frame().await
    }
}
