use macroquad::prelude::*;
use oganesson::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Oganesson"),
        window_width: 1200,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut universe = Universe::<3>::new();
    universe.add_magnetic_field([0.0, 0.0, -1e5] * units::T);

    universe.add_object(
        ObjectBuilder::new_at([600.0, 400.0, 0.0] * units::m)
            .with_size(5.0 * units::m)
            .with_mass(1e1 * units::kg)
            .with_velocity([0.0, -100.0, 0.0] * units::m / units::s)
            .with_charge(1e-3 * units::C)
            .with_color(BLUE)
            .build()
            .unwrap(),
    );

    universe.add_object(
        ObjectBuilder::new_at([300.0, 420.0, 0.0] * units::m)
            .with_size(5.0 * units::m)
            .with_mass(1e1 * units::kg)
            .with_velocity([0.0, 0.0, 0.0] * units::m / units::s)
            .with_charge(-1e-2 * units::C)
            .with_color(RED)
            .build()
            .unwrap(),
    );

    const TIME_SCALE: Float = 1.5;

    let mut last_update = get_time() as Float;
    clear_background(GRAY);

    let mut posis: Vec<Vec<Vector<3>>> = vec![];
    for _ in universe.objects() {
        posis.push(vec![]);
    }

    loop {
        let dt = (get_time() as Float - last_update) * TIME_SCALE;
        last_update = get_time() as Float;
        universe.step(dt);

        for (i, pos) in posis.iter().enumerate() {
            let mut iter = pos.iter();
            if let Some(last) = iter.next() {
                let mut last = last;
                let mut now = iter.next();
                while let Some(nows) = now {
                    draw_line(
                        last[0],
                        last[1],
                        nows[0],
                        nows[1],
                        1.0,
                        universe.objects()[i].color(),
                    );
                    last = nows;
                    now = iter.next();
                }
            }
        }

        for obj in universe.objects() {
            let z = obj.position()[2];
            draw_poly(
                obj.position()[0],
                obj.position()[1],
                50,
                obj.size().0 * (-z/10.0 + 1.0),
                0.,
                obj.color(),
            );
        }

        for (i, pos) in posis.iter_mut().enumerate() {
            // if pos.len() > 1000 {
            //     pos.remove(0);
            // }
            pos.push(universe.objects()[i].position());
        }

        next_frame().await
    }
}
