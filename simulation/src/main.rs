use oganesson::{Vector, units::{m, s}, constants};

fn main() {
    let v = Vector([500.0], m / s);
    println!("{:?}", (1.0 - v.squared() / constants::c.powi(2)).powf(2f64.recip()));

    let mut solver = Solver {
        f: |t, x| t.cos() + x,
        t: 0.0,
        y: 0.0,
        h: 1e-5,
    };

    for _ in 0..10 {
        let actual = solver.y.sin();
        println!("{:?}", actual - solver.solve_rk4());
    }
}

struct Solver{
    f: fn(f32, f32) -> f32, t: f32, y: f32, h: f32,
}

impl Solver {
    fn solve_rk4(&mut self) -> f32 {
        let k1 = (self.f)(self.t, self.y);
        let k2 = (self.f)(self.t + self.h / 2.0, self.y + self.h * k1 / 2.0);
        let k3 = (self.f)(self.t + self.h / 2.0, self.y + self.h * k2 / 2.0);
        let k4 = (self.f)(self.t + self.h, self.y + self.h * k3);
        self.y = (k1 + 2.0 * k2 + 3.0 * k3 + k4) * self.h / 6.0;
        self.t += self.h;
        self.y
    }
}
