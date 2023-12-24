use std::collections::{HashMap, HashSet, VecDeque};

use itertools::{Itertools, Position};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn parse_vector(l: &str) -> Vector {
    let (x, y, z) = l
        .split(|x| x == ' ' || x == ',')
        .filter_map(|x| x.parse::<f64>().ok())
        .collect_tuple()
        .unwrap();
    Vector { x, y, z }
}

fn add(a: Vector, b: Vector) -> Vector {
    Vector {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

fn on_the_line_xy(location: Vector, hail: Hail) -> bool {
    assert!(hail.velocity.x != 0.); // TODO
    let diff = location.x - hail.position.x;
    if diff % hail.velocity.x != 0. {
        return false;
    }
    let time = diff / hail.velocity.x;
    if time < 0. {
        return false;
    }
    let expected_y_pos = time * hail.velocity.y + hail.position.y;
    if expected_y_pos != location.y {
        return false;
    }
    return true;
}
type MM = (f64, f64);

fn within(a: f64, (min, max): MM) -> bool {
    min >= a && a <= max
}

fn within_borders_xy(loc: Vector, mm: MM) -> bool {
    within(loc.x, mm) && within(loc.y, mm)
}

fn intersect_xy(a: Hail, b: Hail, mm: MM) -> bool {
    match intersection_xy(a, b) {
        Some(point) => on_the_line_xy(point, a) && on_the_line_xy(point, b) && within_borders_xy(point, mm),
        None => false,
    }
}

fn line_to_points(h: Hail) -> (Vector, Vector) {
    (h.position, add(h.position, h.velocity))
}

// would rather not do slopes, from https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
fn intersection_xy(a: Hail, b: Hail) -> Option<Vector> {
    // TODO: switch to simpler method
    let (p1, p2) = line_to_points(a);
    let (p3, p4) = line_to_points(b);
    let pxnom = (p1.x * p2.y - p1.y * p2.x) * (p3.x - p4.x) - (p1.x - p2.x) * (p3.x * p4.y - p3.y - p4.y);
    let pynom = (p1.x * p2.y - p1.y * p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x * p4.y - p3.y - p4.y);
    let denom = ((p1.x - p2.x) * (p3.y - p4.y)) - ((p1.y - p2.y) * (p3.x - p4.x));
    if denom == 0. {
        // check some smarter way
        return None;
    }
    Some(Vector {
        x: pxnom / denom,
        y: pynom / denom,
        z: 0.,
    })
}

fn parse_hail(h: &str) -> Hail {
    let (position, velocity) = h.split('@').map(parse_vector).collect_tuple().unwrap();
    Hail { position, velocity }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct Hail {
    position: Vector,
    velocity: Vector,
}

fn parse(f: &str) -> Vec<Hail> {
    std::fs::read_to_string(f).unwrap().lines().map(parse_hail).collect()
}

fn part1(hails: &Vec<Hail>, mm: MM) -> usize {
    hails
        .iter()
        .cartesian_product(hails.iter())
        .filter(|(a, b)| a != b && intersect_xy(**a, **b, mm))
        .count()
}

fn main() {
    println!("{:?}", part1(&parse("inputs/24a"), (7., 27.)));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn linification_test() {
        let mkxy = |x, y| Vector { x, y, z: 0. };
        assert_eq!(
            Some(mkxy(1., 0.)),
            intersection_xy(
                Hail {
                    velocity: mkxy(0., -1.),
                    position: mkxy(1., 1.)
                },
                Hail {
                    velocity: mkxy(1., 0.),
                    position: mkxy(0., 0.)
                }
            )
        );
    }
}
