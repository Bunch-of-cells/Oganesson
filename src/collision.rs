use crate::{Object, Scalar, Vector};

#[derive(Debug, Clone)]
pub struct Collider<const N: usize> {
    pub size: Scalar,
    pub position: Vector<N>,
}

impl<const N: usize> Collider<N> {
    pub fn collides(&self, other: &Collider<N>) -> Option<Vector<N>> {
        let r1 = self.size;
        let r2 = other.size;
        let distance = self.position - other.position;
        let direction = distance.normalized();
        let distance = distance.magnitude().abs();
        if distance >= r1 + r2 {
            None
        } else {
            Some(direction * (r1 + r2 - distance))
        }
    }

    pub fn is_collision(&self, other: &Collider<N>) -> bool {
        self.collides(other).is_some()
    }
}

pub fn possible_collisions<const N: usize>(objects: &[Object<N>]) -> Vec<(usize, usize)> {
    if objects.len() < 2 {
        return Vec::new();
    }

    let mut objects = objects
        .iter()
        .enumerate()
        .map(|(n, obj)| (n, obj.collider()))
        .collect::<Vec<_>>();

    possible_collisions_recursive(&mut objects, 0, 0)
}

fn possible_collisions_recursive<const N: usize>(
    objects: &mut [(usize, Collider<N>)],
    n: usize,
    n_not: usize,
) -> Vec<(usize, usize)> {
    let mut possible_collisions = Vec::new();

    objects.sort_by(|(_, collider1), (_, collider2)| {
        collider1.position[n]
            .partial_cmp(&collider2.position[n])
            .unwrap()
    });

    let median = match objects.len() {
        0 => return Vec::new(),
        x if x % 2 == 0 => (objects[x / 2].1.position[n] + objects[x / 2 - 1].1.position[n]) / 2.0,
        x => objects[(x - 1) / 2].1.position[n],
    };

    let mut a: Vec<_> = objects
        .iter()
        .filter(|(_, collider)| median > collider.position[n] - collider.size)
        .cloned()
        .collect();

    if a.len() == objects.len() {
        if n_not >= N {
            for (i, (obj_a, collider_a)) in a.iter().enumerate() {
                for (obj_b, collider_b) in a.iter().skip(i + 1) {
                    if collider_a.is_collision(collider_b) {
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
        .filter(|(_, collider)| median < collider.position[n] - collider.size)
        .cloned()
        .collect();

    if b.len() == objects.len() {
        if n_not >= N {
            for (i, (obj_a, collider_a)) in b.iter().enumerate() {
                for (obj_b, collider_b) in b.iter().skip(i + 1) {
                    if collider_a.is_collision(collider_b) {
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
        [(a, collider_a), (b, collider_b)] => {
            if collider_a.is_collision(collider_b) {
                possible_collisions.push((*a, *b));
            }
        }
        a => possible_collisions.append(&mut possible_collisions_recursive(a, (n + 1) % N, 0)),
    }

    match b.as_mut_slice() {
        [(a, collider_a), (b, collider_b)] => {
            if collider_a.is_collision(collider_b) {
                possible_collisions.push((*a, *b));
            }
        }
        b => possible_collisions.append(&mut possible_collisions_recursive(b, (n + 1) % N, 0)),
    }

    possible_collisions
}
