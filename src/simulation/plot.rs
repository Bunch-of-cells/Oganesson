use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters::prelude::*;
use plotters_piston::draw_piston_window;

const FPS: u32 = 10;
pub fn main() {
    let mut window: PistonWindow = WindowSettings::new("Real Time CPU Usage", [450, 300])
        .samples(4)
        .build()
        .unwrap();
    window.set_max_fps(FPS as u64);
    while let Some(_) = draw_piston_window(&mut window, |b| {
        let root = b.into_drawing_area();
        root.fill(&WHITE)?;

        let mut cc = ChartBuilder::on(&root)
            .margin(10)
            .caption("Sin", ("sans-serif", 30))
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(-std::f64::consts::PI..std::f64::consts::PI, -1.2..1.2)?;

        cc.configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        cc.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        Ok(())
    }) {}
}
