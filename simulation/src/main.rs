use oganesson::unit::Unit;
use oganesson::Vector;

fn main() {
    let v1 = Vector([20.0, 20.0, 21.0], Unit::NONE);
    let mut v2: Vector<2> = v1.truncated();
    println!("{:?} {:?}", v2, v1);
    v2.0[1] = 0f32;
    println!("{:?} {:?}", v2, v1);
}
