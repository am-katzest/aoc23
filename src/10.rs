use itertools::Itertools;
use std::fs::read_to_string;
use std::ops::Index;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Ground,
    Starting,
    Pipe(Dir, Dir),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

static DIRECTIONS: &[Dir] = &[Dir::Left, Dir::Right, Dir::Up, Dir::Down];

type Coord = (isize, isize);

fn opposite(x: Dir) -> Dir {
    match x {
        Dir::Left => Dir::Right,
        Dir::Right => Dir::Left,
        Dir::Down => Dir::Up,
        Dir::Up => Dir::Down,
    }
}

fn other(t: Tile, x: Dir) -> Option<Dir> {
    match t {
        Tile::Pipe(a, b) if a == x => Some(b),
        Tile::Pipe(a, b) if b == x => Some(a),
        _ => None,
    }
}

fn has_direction(t: Tile, d: Dir) -> bool {
    match t {
        Tile::Ground => false,
        Tile::Starting => true,
        Tile::Pipe(a, b) => d == a || d == b,
    }
}

fn is_start(t: Tile) -> bool {
    match t {
        Tile::Starting => true,
        _ => false,
    }
}

fn can_move(from: Tile, to: Tile, dir: Dir) -> bool {
    has_direction(from, dir) && has_direction(to, opposite(dir))
}

fn walk(d: Dir, (x, y): Coord) -> Coord {
    match d {
        Dir::Left => (x - 1, y),
        Dir::Right => (x + 1, y),
        Dir::Down => (x, y + 1),
        Dir::Up => (x, y - 1),
    }
}

//type Map = Vec<Vec<Tile>>;
#[derive(Clone, Debug)]
struct Map {
    start: Coord,
    tiles: Vec<Vec<Tile>>,
    size_x: isize,
    size_y: isize,
}

impl Index<Coord> for Map {
    type Output = Tile;
    fn index(&self, (x, y): Coord) -> &Tile {
        if x < 0 || x >= self.size_x || y < 0 || y >= self.size_y {
            return &Tile::Ground; // i like to pretend stuff is infinite
        }
        &self.tiles[y as usize][x as usize]
    }
}

fn parse_tile(t: char) -> Tile {
    match t {
        'S' => Tile::Starting,
        '.' => Tile::Ground,
        '|' => Tile::Pipe(Dir::Up, Dir::Down),
        '-' => Tile::Pipe(Dir::Left, Dir::Right),
        'L' => Tile::Pipe(Dir::Up, Dir::Right),
        'F' => Tile::Pipe(Dir::Down, Dir::Right),
        '7' => Tile::Pipe(Dir::Down, Dir::Left),
        'J' => Tile::Pipe(Dir::Up, Dir::Left),
        _ => panic!("the fuck is {t}"),
    }
}

fn parse(f: &str) -> Map {
    let tiles = read_to_string(f)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(parse_tile).collect_vec())
        .collect_vec();
    let mut start = (0, 0);
    'outer: for (y, l) in tiles.iter().enumerate() {
        for (x, t) in l.iter().enumerate() {
            if is_start(*t) {
                start = (x as isize, y as isize);
                break 'outer;
            }
        }
    }
    let size_y = tiles.len() as isize;
    let size_x = tiles[0].len() as isize;
    Map {
        tiles,
        start,
        size_x,
        size_y,
    }
}

fn measure_loop(start: Coord, dir: Dir, m: Map) -> Option<isize> {
    let mut duration = 0;
    let mut current = start;
    let mut dir = dir;
    loop {
        println!("{:?} {:?} {:?}", current, dir, has_direction(m[current], dir));
        let current_tile = m[current];
        if !has_direction(current_tile, dir) {
            return None;
        }
        let target = walk(dir, current);
        let target_tile = m[target]; // i wonder if it'll get reused
        if !can_move(current_tile, target_tile, dir) {
            return None;
        }
        current = target;
        duration += 1;
        dir = match other(target_tile, opposite(dir)) {
            Some(x) => x,
            None => break,
        };
        // shouldn't be needed
        if current == start {
            break;
        }
    }
    Some(duration)
}

fn part1(m: Map) -> i32 {
    println!("{:?}", m);
    for d in DIRECTIONS {
        println!("{:?}", measure_loop(m.start.to_owned(), *d, m.to_owned()));
    }
    3
}

fn main() {
    println!("part 1: {:?}", part1(parse("inputs/10a")));
}

#[cfg(test)]
mod tests {
    use crate::*;
}
