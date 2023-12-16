use itertools::Itertools;
use std::fs::read_to_string;
use std::iter;
use std::ops::Index;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty,
    Splitter(Axis), // axis in which it lets light through
    Mirror(Axis),   // light on this axis gets clockwised
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

type Coord = (usize, usize);

fn clockwise(x: Dir) -> Dir {
    match x {
        Dir::Left => Dir::Up,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Up => Dir::Right,
    }
}

fn opposite(x: Dir) -> Dir {
    clockwise(clockwise(x))
}

fn counterclockwise(x: Dir) -> Dir {
    clockwise(opposite(x))
}

fn axis(d: Dir) -> Axis {
    match d {
        Dir::Up | Dir::Down => Axis::Vertical,
        Dir::Left | Dir::Right => Axis::Horizontal,
    }
}

fn distort(d: Dir, t: Tile) -> Vec<Dir> {
    match (t, axis(d)) {
        (Tile::Mirror(x), y) if x == y => vec![clockwise(d)],
        (Tile::Mirror(x), y) if x != y => vec![counterclockwise(d)],
        (Tile::Splitter(x), y) if x == y => vec![d],
        (Tile::Splitter(x), y) if x != y => vec![clockwise(d), counterclockwise(d)],
        _ => vec![d],
    }
}

fn step(d: Dir, (x, y): Coord, m: &Map) -> Option<Coord> {
    let (mx, my) = m.size;
    match d {
        Dir::Left if x > 0 => Some((x - 1, y)),
        Dir::Right if x < mx => Some((x + 1, y)),
        Dir::Down if y < my => Some((x, y + 1)),
        Dir::Up if y > 0 => Some((x, y - 1)),
        _ => None,
    }
}

//type Map = Vec<Vec<Tile>>;
#[derive(Clone, Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    size: Coord,
}

impl Index<Coord> for Map {
    type Output = Tile;
    fn index(&self, (x, y): Coord) -> &Tile {
        &self.tiles[y][x]
    }
}

fn parse_tile(t: char) -> Tile {
    match t {
        '.' => Tile::Empty,
        '|' => Tile::Splitter(Axis::Vertical),
        '-' => Tile::Splitter(Axis::Horizontal),
        '/' => Tile::Mirror(Axis::Vertical),
        '\\' => Tile::Mirror(Axis::Horizontal),
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
    let size = (tiles[0].len(), tiles.len());
    Map { tiles, size }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Photon {
    dir: Dir,
    coord: Coord,
}

fn proceed(m: &Map, w: Photon) -> Vec<Photon> {
    match step(w.dir, w.coord, m) {
        None => vec![],
        Some(coord) => distort(w.dir, m[coord]).iter().map(|&dir| Photon { dir, coord }).collect_vec(),
    }
}

fn part1(m: Map, d: Dir) -> usize {
    //create_loop(m, d).count() / 2
    3
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Field {
    Energized,
    Dormant,
}

static DIRECTIONS: &[Dir] = &[Dir::Left, Dir::Right, Dir::Up, Dir::Down];

fn main() {
    //println!("part 1: {:?}", part1(parse("inputs/10b"), Dir::Up));
    //println!("part 2: {:?}", part2(parse("inputs/10b"), Dir::Up));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn reflection_test() {
        assert_eq!(vec![Dir::Left], distort(Dir::Left, parse_tile('.')));
        assert_eq!(vec![Dir::Up], distort(Dir::Left, parse_tile('\\')));
        assert_eq!(vec![Dir::Down], distort(Dir::Left, parse_tile('/')));

        assert_eq!(vec![Dir::Down], distort(Dir::Right, parse_tile('\\')));
        assert_eq!(vec![Dir::Up], distort(Dir::Right, parse_tile('/')));

        assert_eq!(vec![Dir::Left], distort(Dir::Up, parse_tile('\\')));
        assert_eq!(vec![Dir::Right], distort(Dir::Up, parse_tile('/')));

        assert_eq!(vec![Dir::Right, Dir::Left], distort(Dir::Up, parse_tile('-')));
        assert_eq!(vec![Dir::Left, Dir::Right], distort(Dir::Down, parse_tile('-')));

        assert_eq!(vec![Dir::Up], distort(Dir::Up, parse_tile('|')));
        assert_eq!(vec![Dir::Down], distort(Dir::Down, parse_tile('|')));

        assert_eq!(vec![Dir::Up, Dir::Down], distort(Dir::Left, parse_tile('|')));
        assert_eq!(vec![Dir::Down, Dir::Up], distort(Dir::Right, parse_tile('|')));

        assert_eq!(vec![Dir::Left], distort(Dir::Left, parse_tile('-')));
        assert_eq!(vec![Dir::Right], distort(Dir::Right, parse_tile('-')));

    }
}
