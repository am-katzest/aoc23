use itertools::Itertools;
use std::fs::read_to_string;
use std::iter;
use std::ops::Index;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Ground,
    Blocked,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

type Coord = (usize, usize);

fn blocked(t: Tile) -> bool {
    t == Tile::Blocked
}

fn step(d: Dir, (x, y): Coord) -> Coord {
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
}

impl Index<Coord> for Map {
    type Output = Tile;
    fn index(&self, (x, y): Coord) -> &Tile {
        &self.tiles[y][x]
    }
}

fn parse_tile(t: char) -> Tile {
    match t {
        '.' | 'S' => Tile::Ground,
        '#' => Tile::Blocked,
        _ => panic!("the fuck is {t}"),
    }
}

fn parse(f: &str) -> Map {
    let mut start = (0, 0);
    let tiles = read_to_string(f)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y)
                    };
                    parse_tile(c)
                })
                .collect_vec()
        })
        .collect_vec();
    Map { tiles, start }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Walker {
    age: usize,
    coord: Coord,
}

static DIRECTIONS: &[Dir] = &[Dir::Left, Dir::Right, Dir::Up, Dir::Down];

fn part1(m: Map) -> usize {
    3
}

fn main() {
    println!("part 1: {:?}", part1(parse("inputs/10b")));
    //println!("part 2: {:?}", part2(parse("inputs/10b"), Dir::Up));
}

#[cfg(test)]
mod tests {
    use crate::*;
}
