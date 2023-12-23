use itertools::Itertools;
use std::fs::read_to_string;
use std::iter;
use std::ops::Index;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Dir),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

type Addr = u32; // could be u16 ( ***i think*** )

type Coord = (Addr, Addr);

fn step(d: Dir, (x, y): Coord) -> Coord {
    match d {
        Dir::Left => (x - 1, y),
        Dir::Right => (x + 1, y),
        Dir::Down => (x, y + 1),
        Dir::Up => (x, y - 1),
    }
}
//fn can_move(d:Dir, t: Tile) {
//
//}

//fn try_step(d: Dir, c: Coord, m: &Map) -> Option<Coord> {
//
//}

//type Map = Vec<Vec<Tile>>;
#[derive(Clone, Debug)]
struct Map {
    start: Coord,
    end: Coord,
    size: Coord,
    tiles: Vec<Vec<Tile>>,
}

impl Index<Coord> for Map {
    type Output = Tile;
    fn index(&self, (x, y): Coord) -> &Tile {
        &self.tiles[y as usize][x as usize]
    }
}

fn parse_tile(t: char) -> Tile {
    match t {
        '.' => Tile::Path,
        '#' => Tile::Forest,
        '>' => Tile::Slope(Dir::Right),
        '<' => Tile::Slope(Dir::Left),
        '^' => Tile::Slope(Dir::Up),
        'v' => Tile::Slope(Dir::Down),
        _ => panic!("the fuck is {t}"),
    }
}

fn parse(f: &str) -> Map {
    let tiles = read_to_string(f)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(parse_tile).collect_vec())
        .collect_vec();
    let size = (tiles[0].len() as Addr, tiles.len() as Addr);
    let start = (1, 0);
    let end = ((size.0 -2) as Addr, (size.0-1) as Addr);
    Map { tiles, start, end, size }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Node {
    start: Coord,
    end: Coord,
    length: usize,
}

fn get_nodes(m: Map) -> Vec<Node> {
    vec![]
}

fn main() {
    println!("part 1: {:?}", get_nodes(parse("inputs/10b")));

}
