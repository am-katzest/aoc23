use itertools::Itertools;
use std::fs::read_to_string;
use std::iter;
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

type Coord = (isize, isize);

fn opposite(x: Dir) -> Dir {
    match x {
        Dir::Left => Dir::Right,
        Dir::Right => Dir::Left,
        Dir::Down => Dir::Up,
        Dir::Up => Dir::Down,
    }
}

fn clockwise(x: Dir) -> Dir {
    match x {
        Dir::Left => Dir::Up,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Up => Dir::Right,
    }
}

fn other(t: Tile, x: Dir) -> Option<Dir> {
    match t {
        Tile::Pipe(a, b) if a == x => Some(b),
        Tile::Pipe(a, b) if b == x => Some(a),
        _ => None,
    }
}

fn is_start(t: Tile) -> bool {
    match t {
        Tile::Starting => true,
        _ => false,
    }
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


#[derive(Clone, Copy, Debug, PartialEq)]
struct Walker {
    dir: Dir,
    coord:  Coord
}

fn proceed(m :Map, w: Walker)  -> Option<Walker> {
    let target = walk(w.dir, w.coord);
    if target == m.start {
        None
    } else {
        Some(Walker {coord:target, dir: other(m[target], opposite(w.dir)).unwrap()})
    }
}

fn create_loop(m: Map, d: Dir) -> impl Iterator<Item = Walker> {
    let starting = Walker {coord: m.start, dir: d};
    iter::successors(Some(starting), move |x| {
        proceed(m.to_owned(), x.to_owned())
    })
}

fn part1(m: Map, d : Dir) -> usize {
    create_loop(m, d).count() / 2
}

fn main() {
    println!("part 1: {:?}", part1(parse("inputs/10a"), Dir::Down));
    println!("part 1: {:?}", part1(parse("inputs/10b"), Dir::Up));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part1_test() {
        assert_eq!(8, part1(parse("inputs/10a"), Dir::Down));
        assert_eq!(6927, part1(parse("inputs/10b"), Dir::Up));
    }
}
