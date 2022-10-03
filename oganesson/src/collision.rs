use crate::{Object, Scalar, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct Transform<const N: usize> {
    pub(crate) position: Vector<N>,
    // pub(crate) scale: Vector<N>,
    // pub(crate) rotation: Quaternion,
}

impl<const N: usize> Transform<N> {
    pub fn new(position: Vector<N>) -> Transform<N> {
        Transform {
            position,
            // scale: Vector([1.0; N], units::Null),
            // rotation: Quaternion {
            //     w: 0.0,
            //     x: 0.0,
            //     y: 0.0,
            //     z: 0.0,
            // },
        }
    }

    pub fn position(&self) -> Vector<N> {
        self.position
    }

    // pub fn with_scale(mut self, scale: Vector<N>) -> Self {
    //     self.scale = scale;
    //     self
    // }

    // pub fn with_rotation(mut self, rotation: Quaternion) -> Self {
    //     self.rotation = rotation;
    //     self
    // }

    // pub fn scale(&self) -> Vector<N> {
    //     self.scale
    // }

    // pub fn rotation(&self) -> Quaternion {
    //     self.rotation
    // }
}

#[derive(Debug, Clone)]
pub enum Collider<const N: usize> {
    Sphere { radius: Scalar },
    Polygon { points: Vec<Vector<N>> },
}

impl<const N: usize> Collider<N> {
    pub fn is_collision(
        &self,
        transform: &Transform<N>,
        collider: &Collider<N>,
        collider_transform: &Transform<N>,
    ) -> Option<Vector<N>> {
        match (self, collider) {
            (&Collider::Sphere { radius: r1 }, &Collider::Sphere { radius: r2 }) => {
                let distance = transform.position - collider_transform.position;
                let direction = distance.normalized();
                let distance = distance.magnitude().abs();
                if distance >= r1 + r2 {
                    None
                } else {
                    Some(direction * (r1 + r2 - distance))
                }
            }

            (Collider::Polygon { .. }, &Collider::Sphere { .. }) => None,
            (Collider::Polygon { .. }, Collider::Polygon { .. }) => None,

            (Collider::Sphere { .. }, Collider::Polygon { .. }) => collider
                .is_collision(collider_transform, self, transform)
                .map(|v| -v),
        }
    }

    pub fn get_bounding_box(&self, transform: &Transform<N>) -> BoundingBox<N> {
        match self {
            Collider::Sphere { radius } => {
                let position = transform.position;
                let mut min = position;
                min.add_to_each(-*radius);
                let mut max = position;
                max.add_to_each(*radius);
                BoundingBox { min, max }
            }

            Collider::Polygon { points } =>  {
                let mut mins = [0.0; N];
                let mut maxs = [0.0; N];
                for i in 0..N {
                    let mut min = points.first().unwrap()[i];
                    let mut max = points.first().unwrap()[i];
                    for point in points {
                        if point[i] > max {
                            max = point[i];
                        } else if point[i] < min {
                            min = point[i];
                        }
                    }
                    mins[i] = min;
                    maxs[i] = max;
                }
                BoundingBox {
                    min: Vector::from(mins),
                    max: Vector::from(maxs),
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Collision<const N: usize> {
    pub a: usize,
    pub b: usize,
    pub direction: Vector<N>,
}

impl<const N: usize> Collision<N> {
    pub fn new(a: usize, b: usize, direction: Vector<N>) -> Collision<N> {
        Collision { a, b, direction }
    }
}

#[derive(Debug, Clone)]
pub struct BoundingBox<const N: usize> {
    pub min: Vector<N>,
    pub max: Vector<N>,
}

impl<const N: usize> BoundingBox<N> {
    pub fn overlaps(&self, other: &BoundingBox<N>) -> bool {
        (0..N).all(|n| self.min[n] <= other.max[n] && self.max[n] >= other.min[n])
    }

    pub fn center(&self) -> Vector<N> {
        (self.min + self.max) / 2.0
    }
}

pub fn possible_collisions<const N: usize>(objects: &[Object<N>]) -> Vec<(usize, usize)> {
    if objects.len() < 2 {
        return Vec::new();
    }

    let mut objects = objects
        .iter()
        .enumerate()
        .map(|(n, obj)| (n, obj.collider.get_bounding_box(&obj.transform)))
        .collect::<Vec<_>>();

    possible_collisions_recursive(&mut objects, 0, 0)
}

fn possible_collisions_recursive<const N: usize>(
    objects: &mut [(usize, BoundingBox<N>)],
    n: usize,
    n_not: usize,
) -> Vec<(usize, usize)> {
    let mut possible_collisions = Vec::new();

    objects.sort_by(|(_, bounds1), (_, bounds2)| {
        bounds1.center()[n]
            .partial_cmp(&bounds2.center()[n])
            .unwrap()
    });

    let median = match objects.len() {
        0 => return Vec::new(),
        x if x % 2 == 0 => (objects[x / 2].1.center()[n] + objects[x / 2 - 1].1.center()[n]) / 2.0,
        x => objects[(x - 1) / 2].1.center()[n],
    };

    let mut a = objects
        .iter()
        .filter(|(_, bounds)| median > bounds.min[n])
        .cloned()
        .collect::<Vec<_>>();

    if a.len() == objects.len() {
        if n_not >= N {
            for (i, (obj_a, bounds_a)) in a.iter().enumerate() {
                for (obj_b, bounds_b) in a.iter().skip(i + 1) {
                    if bounds_a.overlaps(bounds_b) {
                        possible_collisions.push((*obj_a, *obj_b));
                    }
                }
            }

            return possible_collisions;
        } else {
            return possible_collisions_recursive(a.as_mut_slice(), (n + 1) % N, n_not + 1);
        }
    }

    let mut b = objects
        .iter()
        .filter(|(_, bounds)| median < bounds.min[n])
        .cloned()
        .collect::<Vec<_>>();

    if b.len() == objects.len() {
        if n_not >= N {
            for (i, (obj_a, bounds_a)) in b.iter().enumerate() {
                for (obj_b, bounds_b) in b.iter().skip(i + 1) {
                    if bounds_a.overlaps(bounds_b) {
                        possible_collisions.push((*obj_a, *obj_b));
                    }
                }
            }

            return possible_collisions;
        } else {
            return possible_collisions_recursive(b.as_mut_slice(), (n + 1) % N, n_not + 1);
        }
    }

    match a.as_mut_slice() {
        [(a, bounds_a), (b, bounds_b)] => {
            if bounds_a.overlaps(bounds_b) {
                possible_collisions.push((*a, *b));
            }
        }
        a => possible_collisions.append(&mut possible_collisions_recursive(a, (n + 1) % N, 0)),
    }

    match b.as_mut_slice() {
        [(a, bounds_a), (b, bounds_b)] => {
            if bounds_a.overlaps(bounds_b) {
                possible_collisions.push((*a, *b));
            }
        }
        b => possible_collisions.append(&mut possible_collisions_recursive(b, (n + 1) % N, 0)),
    }

    possible_collisions
}
