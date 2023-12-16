use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
        Dir::Right if x < (mx - 1) => Some((x + 1, y)),
        Dir::Down if y < (my - 1) => Some((x, y + 1)),
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
    let size = (tiles[0].len(), tiles.len());
    Map { tiles, size }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Photon {
    dir: Dir,
    coord: Coord,
}

fn proceed(m: &Map, w: Photon) -> Vec<Photon> {
    distort(w.dir, m[w.coord])
        .iter()
        .filter_map(|&dir| step(dir, w.coord, m).map(|coord| Photon { dir, coord }))
        .collect_vec()
}

fn run(map :Map, initial: Photon) -> HashSet<Photon> {
    let mut photons = vec![initial];
    //let mut photons = distort(initial.dir, map[(0, 0)]).map(|dir| Photon {dir, coord: initial.coord}).collect_vec();
    let mut visited: HashSet<Photon> = HashSet::new();
    loop {
        for p in photons.iter().copied() {
            visited.insert(p);
        }
        photons = photons
            .into_iter()
            .flat_map(|x| proceed(&map, x))
            .filter(|x| !visited.contains(x))
            .collect_vec();
        if photons.len() == 0 {
            break;
        }
    }
    visited
}

fn part1(f: &str) -> usize {
    let map = parse(f);
    let initial = Photon {
        dir: Dir::Right,
        coord: (0, 0),
    };
    run(map, initial).iter().map(|x| x.coord).unique().count()
}

fn main() {
    println!("part 1: {:?}", part1("inputs/16b"));
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
    #[test]
    fn part1_test() {
        assert_eq!(46, part1("inputs/16a"));
        assert_eq!(7870, part1("inputs/16b"));
    }
}
