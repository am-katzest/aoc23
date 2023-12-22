use std::collections::HashMap;

use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

impl std::ops::Index<Axis> for Coord {
    type Output = isize;
    fn index(&self, k: Axis) -> &isize {
        match k {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

impl std::ops::IndexMut<Axis> for Coord {
    fn index_mut(&mut self, k: Axis) -> &mut isize {
        match k {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Brick {
    id: usize,
    start: Coord,
    end: Coord,
}

fn intersects(xs: isize, xe: isize, ys: isize, ye: isize) -> bool {
    xs <= ys && ys <= xe || xs <= ye && ye <= xe || ys <= xs && xe <= ye
    // y start inside x || y end inside x || x inside y
    // i think that's all
}

fn intersects_axis(a: Brick, b: Brick, k: Axis) -> bool {
    intersects(a.start[k], a.end[k], b.start[k], b.end[k])
}

fn collides(a: Brick, b: Brick) -> bool {
    if a.id == b.id {
        return false;
    }
    intersects_axis(a, b, Axis::X) && intersects_axis(a, b, Axis::Y) && intersects_axis(a, b, Axis::Z)
}

// it's amusing, because given how many functions like that are there, it would probably better to store brick as three axes
fn normalize_axis(b: &mut Brick, a: Axis) {
    let s = b.start[a];
    let e = b.end[a];
    if e >= s {
        return;
    }
    println!("{:?} was cringe", b);
    b.start[a] = e;
    b.end[a] = s;
}

fn normalize(b: Brick) -> Brick {
    let mut b = b.to_owned();
    normalize_axis(&mut b, Axis::X);
    normalize_axis(&mut b, Axis::Y);
    normalize_axis(&mut b, Axis::Z);
    b
}

fn inside_grass(x: Brick) -> bool {
    x.end.z < 0
}

fn move_vertically(x: Brick, alt: isize) -> Brick {
    let mut r = x.to_owned();
    r.start.z += alt;
    r.end.z += alt;
    r
}

fn fall(x: Vec<Brick>) -> Vec<Brick> {
    let mut bricks = x.to_owned();
    loop {
        let mut moved = false;
        for i in 0..bricks.len() {
            let current = bricks[i];
            let down = move_vertically(current, -1);
            if !(inside_grass(down) || collides_with_any(&bricks, down)) {
                continue;
            }
            println!("moved {:?} to {:?}", current, down);
            moved = true;
            bricks[i] = down;
        }
        println!("iterated",);
        if !moved {
            return bricks;
        }
    }
}

fn collides_with_any(bricks: &Vec<Brick>, down: Brick) -> bool {
    bricks.iter().any(|other| collides(down, *other))
}

fn parse_coord(l: &str) -> Coord {
    let (x, y, z) = l.split(',').map(|x| x.parse::<isize>().unwrap()).collect_tuple().unwrap();
    Coord { x, y, z }
}

fn parse_brick((id, l): (usize, &str)) -> Brick {
    let (start, end) = l.split('~').map(parse_coord).collect_tuple().unwrap();
    normalize(Brick { start, end, id })
}

fn parse(f: &str) -> Vec<Brick> {
    std::fs::read_to_string(f).unwrap().lines().enumerate().map(parse_brick).collect_vec()
}

fn collisions(bricks: &Vec<Brick>, down: Brick) -> Vec<usize> {
    bricks.iter().filter(|other| collides(down, **other)).map(|x| x.id).collect_vec()
}

fn part1(bricks: Vec<Brick>) -> usize {
    let bricks = fall(bricks);
    let mut removable: HashMap<usize, bool> = bricks.iter().map(|x| (x.id, true)).collect();
    for brick in &bricks {
        let supports = collisions(&bricks, move_vertically(*brick, -1));
        println!("{:?}, {:?}", supports, brick);

        if supports.len() == 1 {
            removable.insert(supports[0], false);
        }
    }
    println!("{:?}", removable);
    3
}

fn main() {
    println!("{:?}", part1(parse("inputs/22a")));
    //println!("{:?}", part1(parse("inputs/22a")));
    //println!("{:?}", fall(parse("inputs/22a")));
    //println!("{:?}", parse("inputs/22b"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn intersection_test() {
        assert_eq!(true, intersects(1, 1, 1, 1));
        assert_eq!(false, intersects(1, 1, 2, 2));
        assert_eq!(false, intersects(2, 2, 1, 1));
        assert_eq!(true, intersects(1, 5, 2, 3));
        assert_eq!(true, intersects(2, 3, 1, 5));
        assert_eq!(true, intersects(1, 5, 2, 7));
        assert_eq!(true, intersects(2, 7, 1, 5));
        assert_eq!(true, intersects(1, 2, 2, 7));
        assert_eq!(true, intersects(2, 7, 1, 2));
    }
    #[test]
    fn collision_test() {
        assert_eq!(true, collides(parse_brick((0, "1,0,1~1,2,1")), parse_brick((1, "1,0,1~1,2,1"))));
        assert_eq!(false, collides(parse_brick((0, "1,0,1~1,2,1")), parse_brick((0, "1,0,1~1,2,1"))));
        assert_eq!(true, collides(parse_brick((0, "1,2,1~1,3,1")), parse_brick((1, "1,0,1~1,2,1"))));
        assert_eq!(false, collides(parse_brick((0, "1,3,1~1,4,1")), parse_brick((1, "1,0,1~1,2,1"))));
    }
    use crate::*;
}
