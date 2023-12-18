use itertools::Itertools;
use substring::Substring;
use std::fs::read_to_string;
use std::ops::Index;
use std::iter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

fn step(d: Dir, (x, y): Coord) -> Coord {
    match d {
        Dir::Left => (x - 1, y),
        Dir::Right => (x + 1, y),
        Dir::Down => (x, y + 1),
        Dir::Up => (x, y - 1),
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Undug,
    Trench,
    Filling
}


#[derive(Clone, Debug, PartialEq)]
struct Instruction {
    dir:Dir,
    length:usize
}

fn parse_line(l: &str) -> Instruction {
    let (d, l, _c) = l.split_whitespace().collect_tuple().unwrap();
    let dir = match d {
        "R" => Dir::Right,
        "L" => Dir::Left,
        "D" => Dir::Down,
        "U" => Dir::Up,
        _ => panic!("wrong dir, {d}"),
    };
    let length = l.parse::<usize>().unwrap();
    Instruction {dir, length}
}

fn parse_line2(l: &str) -> Instruction {
    let (_, _, c) = l.split_whitespace().collect_tuple().unwrap();
    let l = c.substring(2, 7);
    let dir = match c.chars().nth(7).unwrap() {
        '0'=> Dir::Right,
        '2' => Dir::Left,
        '1'=> Dir::Down,
        '3' => Dir::Up,
        _ => panic!(";-;"),
    };
    let length = usize::from_str_radix(l, 16).unwrap();
    Instruction {dir, length}
}

fn unroll(i: Instruction) -> Vec<Dir> {
    iter::repeat(i.dir).take(i.length).collect_vec()
}

fn parse(f: &str, parse_line: fn(&str) -> Instruction) -> Vec<Instruction> {
    read_to_string(f)
        .unwrap()
        .lines()
        .map(parse_line)
        .collect()
}


static DIRECTIONS: &[Dir] = &[Dir::Left, Dir::Right, Dir::Up, Dir::Down];

fn fill(f: &mut Vec<Vec<Tile>>, c: Coord) {
    if f[c.1][c.0] == Tile::Undug {
        f[c.1][c.0] = Tile::Filling;
        for dir in DIRECTIONS {
            fill(f, step(*dir, c));
        }
    }
}

fn solve(f: &str) -> usize {
    let instr = parse(f, parse_line);
    let x = 500;
    let y = 400;
    let row = iter::repeat(Tile::Undug).take(x).collect_vec();
    let mut map = iter::repeat(row).take(y).collect_vec();
    let mut c: Coord = (150, 200);
    let f: Coord = (151, 201);
    map[c.1][c.0] = Tile::Trench;
    for d in instr.into_iter().flat_map(unroll).collect_vec() {
        c = step(d, c);
        map[c.1][c.0] = Tile::Trench;
    }
    fill(&mut map, f);
    print(&map);
    map.into_iter().flatten().filter(|x| *x != Tile::Undug).count()
}


fn solve2(f: &str) -> usize {
    let instr = parse(f, parse_line2);
    let x = 500;
    let y = 400;
    let row = iter::repeat(Tile::Undug).take(x).collect_vec();
    let mut map = iter::repeat(row).take(y).collect_vec();
    let mut c: Coord = (150, 200);
    let f: Coord = (151, 201);
    map[c.1][c.0] = Tile::Trench;
    for d in instr.into_iter().flat_map(unroll).collect_vec() {
        c = step(d, c);
        map[c.1][c.0] = Tile::Trench;
    }
    fill(&mut map, f);
    print(&map);
    map.into_iter().flatten().filter(|x| *x != Tile::Undug).count()
}
fn print(map: &Vec<Vec<Tile>>) {
    for r in map {
        for c in r {
            let u = match c {
                Tile::Trench => '#',
                Tile::Undug => '.',
                Tile::Filling => '@',
            };
            print!("{u}");
        }
        println!("");
    }
}

fn main() {
    println!("part 2: {:?}", solve("inputs/18a"));
    println!("part 2: {:?}", solve("inputs/18b"));
}


#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn parsing_test() {
        assert_eq!(Instruction {dir: Dir::Right, length: 6}, parse_line("R 6 (#70c710)"));
        assert_eq!(Instruction {dir: Dir::Right, length: 461937}, parse_line2("R 6 (#70c710)"));
    }
}
