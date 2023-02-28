use crate::{units, Object, ObjectID, Scalar, Transform, Vector};

#[derive(Debug, Clone, Default)]
pub enum Collider<const N: usize> {
    Sphere {
        radius: Scalar,
    },
    Polygon {
        points: Vec<Vector<N>>,
    },
    #[default]
    Point,
}

impl<const N: usize> Collider<N> {
    pub fn get_bounding_box(&self, transform: &Transform<N>) -> BoundingBox<N> {
        match self {
            &Collider::Sphere { radius } => {
                let position = transform.position;
                let mut min = position;
                min.0
                    .iter_mut()
                    .for_each(|a| *a -= (radius * transform.size).value());
                let mut max = position;
                max.0
                    .iter_mut()
                    .for_each(|a| *a += (radius * transform.size).value());
                BoundingBox { min, max }
            }
            Collider::Polygon { points } => {
                let mut dist_from_center = [0.0; N];
                for i in 0..N {
                    let mut min = points.first().unwrap()[i];
                    let mut max = points.first().unwrap()[i];
                    for &point in points {
                        let point = transform.rotation.rotate_vec(point * transform.size);
                        if point[i] > max {
                            max = point[i];
                        } else if point[i] < min {
                            min = point[i];
                        }
                    }
                    dist_from_center[i] = (max - min) / 2.0;
                }
                let position = transform.position;
                let dist_from_center = dist_from_center * units::m;
                BoundingBox {
                    min: position - dist_from_center,
                    max: position + dist_from_center,
                }
            }
            Collider::Point => BoundingBox {
                min: transform.position,
                max: transform.position,
            },
        }
    }

    pub fn is_collision(
        &self,
        transform: &Transform<N>,
        other: &Collider<N>,
        other_transform: &Transform<N>,
    ) -> Option<Vector<N>> {
        match (self, other) {
            (_, Collider::Point) | (Collider::Point, _) => None,
            (&Collider::Sphere { radius: r1 }, &Collider::Sphere { radius: r2 }) => {
                let distance = transform.position() - other_transform.position();
                let direction = distance.normalized();
                let distance = distance.magnitude().abs();
                if distance >= r1 + r2 {
                    None
                } else {
                    Some(direction * (r1 + r2 - distance))
                }
            }
            (Collider::Polygon { .. }, Collider::Sphere { .. }) => todo!(),
            (Collider::Sphere { .. }, Collider::Polygon { .. }) => other
                .is_collision(other_transform, self, transform)
                .map(|v| -v),
            (Collider::Polygon { points: a }, Collider::Polygon { points: b }) => {
                let (pos_a, pos_b) = (transform.position, other_transform.position);
                let mut d = (pos_a - pos_b).normalized();

                let support = |d| {
                    let s_a = *a
                        .iter()
                        .max_by(|&&v1, &&v2| (v1).dot(d).partial_cmp(&v2.dot(d)).unwrap())
                        .unwrap();
                    let s_b = *b
                        .iter()
                        .max_by(|v1, v2| v1.dot(d).partial_cmp(&v2.dot(d)).unwrap())
                        .unwrap();
                    s_a - s_b
                };

                let s = support(d);
                if s.is_zero() {
                    println!("CoLLisoN");
                    return None;
                }
                d = -s.normalized();
                let mut simplex = vec![s];

                loop {
                    let x = support(d);
                    if s.is_zero() {
                        println!("CoLLisoN");
                        return None;
                    }
                    if x.dot(d) < 0.0 {
                        break None;
                    }
                    simplex.push(x);
                    if simplex.len() == 2 {
                        let &[y, x, ..] = simplex.as_slice() else {unreachable!()};
                        let xy = y - x;
                        d = xy.triple_product(-x, xy).normalized();
                    } else {
                        let &[z, y, x, ..] = simplex.as_slice() else {unreachable!()};
                        let xy = y - x;
                        let xz = z - x;
                        let xy_perp = xz.triple_product(xy, xy);
                        let xz_perp = xy.triple_product(xz, xz);
                        if xy_perp.dot(-x) > 0.0 {
                            simplex.remove(0);
                            d = xy_perp.normalized();
                        } else if xz_perp.dot(-x) > 0.0 {
                            simplex.remove(1);
                            d = xz_perp.normalized();
                        } else {
                            println!("CoLLisoN");
                            return None;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Collision<const N: usize> {
    pub obj_a: ObjectID,
    pub obj_b: ObjectID,
    pub normal: Vector<N>,
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
        .map(|(n, obj)| (n, obj.collider().get_bounding_box(obj.transform())))
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

    let mut a: Vec<_> = objects
        .iter()
        .filter(|(_, bounds)| median > bounds.min[n])
        .cloned()
        .collect();

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

    let mut b: Vec<_> = objects
        .iter()
        .filter(|(_, bounds)| median < bounds.min[n])
        .cloned()
        .collect();

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
