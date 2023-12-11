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

type Coord = (usize, usize);

fn opposite(x: Dir) -> Dir {
    match x {
        Dir::Left => Dir::Right,
        Dir::Right => Dir::Left,
        Dir::Down => Dir::Up,
        Dir::Up => Dir::Down,
    }
}

fn counterclockwise(x: Dir) -> Dir {
    match x {
        Dir::Left => Dir::Down,
        Dir::Right => Dir::Up,
        Dir::Down => Dir::Right,
        Dir::Up => Dir::Left,
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
                start = (x, y);
                break 'outer;
            }
        }
    }
    Map { tiles, start }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Walker {
    dir: Dir,
    coord: Coord,
}

fn proceed(m: Map, w: Walker) -> Option<Walker> {
    let target = step(w.dir, w.coord);
    if target == m.start {
        None
    } else {
        Some(Walker {
            coord: target,
            dir: other(m[target], opposite(w.dir)).unwrap(),
        })
    }
}

fn create_loop(m: Map, d: Dir) -> impl Iterator<Item = Walker> {
    let starting = Walker { coord: m.start, dir: d };
    iter::successors(Some(starting), move |x| proceed(m.to_owned(), x.to_owned()))
}

fn part1(m: Map, d: Dir) -> usize {
    create_loop(m, d).count() / 2
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Field {
    Touched,
    Untouched,
    Path,
}

static DIRECTIONS: &[Dir] = &[Dir::Left, Dir::Right, Dir::Up, Dir::Down];

fn fill (f: &mut Vec<Vec<Field>>, c: Coord) {
    if f[c.1][c.0] == Field::Untouched {
        f[c.1][c.0] = Field::Touched;
        for dir in DIRECTIONS {
            fill(f, step(*dir, c));
        }
    } 
}

fn print (f: &Vec<Vec<Field>>) {
    for l in f {
        for i in l {
            let c = match i {
                Field::Touched => "██",
                Field::Path => "[]",
                Field::Untouched => "  ",
            };
            print!("{c}");
        }
        println!("");
    }
}

fn part2(m: Map, d: Dir) -> usize {
    let mut x = m.tiles.iter().map(|x| x.iter().map(|_| Field::Untouched).collect_vec()).collect_vec();
    for i in create_loop(m.to_owned(), d) {
        x[i.coord.1][i.coord.0] = Field::Path;
    }
    for i in create_loop(m, d) {
        let c = step(counterclockwise(i.dir),i.coord);
        fill(&mut x, c);
    }
    print(&x);
    x.into_iter().flatten().filter(|&x| x == Field::Touched).count()
}

fn main() {
    //println!("part 1: {:?}", part1(parse("inputs/10a"), Dir::Down));
    //println!("part 1: {:?}", part1(parse("inputs/10b"), Dir::Up));
    println!("part 2: {:?}", part2(parse("inputs/10a"), Dir::Down));
    println!("part 2: {:?}", part2(parse("inputs/10b"), Dir::Up));
    println!("part 2: {:?}", part2(parse("inputs/10c"), Dir::Down));
    println!("part 2: {:?}", part2(parse("inputs/10d"), Dir::Right));
    println!("part 2: {:?}", part2(parse("inputs/10e"), Dir::Left));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part1_test() {
        assert_eq!(8, part1(parse("inputs/10a"), Dir::Down));
        assert_eq!(6927, part1(parse("inputs/10b"), Dir::Up));
    }
    #[test]
    fn part2_test(){
        //assert_eq!(2, part2(parse("inputs/10a"), Dir::Down, counterclockwise));
    //assert_eq!(, part2(parse("inputs/10b"), Dir::Up, counterclockwise));
    assert_eq!(4, part2(parse("inputs/10c"), Dir::Down));
    assert_eq!(8, part2(parse("inputs/10d"), Dir::Right));
    assert_eq!(10, part2(parse("inputs/10e"), Dir::Left));

    }
}
