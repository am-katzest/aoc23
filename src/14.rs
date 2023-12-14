use itertools::Itertools;
use std::fs::read_to_string;
use std::iter;
use std::ops::Index;


// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty,
    Round,
    Square,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

type Coord = (usize, usize);

type Map = Vec<Vec<Tile>>;
type Row = Vec<Tile>;

fn parse_tile(t: char) -> Tile {
    match t {
        '.' => Tile::Empty,
        '#' => Tile::Square,
        'O' => Tile::Round,
        _ => panic!("the fuck is {t}"),
    }
}

fn shove_right(m: &mut Row) {
    let mut stones = 0;
    for i in 0..m.len() {
        match m[i] {
                Tile::Square => {
                    for i in (i-stones)..i {
                        m[i] = Tile::Round;
                    }
                    stones = 0;
                }
                Tile::Round => {stones += 1; m[i] = Tile::Empty},
                _ => {},
            }
    }
    for i in (m.len()-stones)..m.len() {
        m[i] = Tile::Round;
    }

}

fn parse(f: &str) -> Map {
    read_to_string(f)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(parse_tile).collect_vec())
        .collect_vec()
}

fn main() {
    parse("inputs/14a");
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn shove_right_test() {

        let mut a = vec![Tile::Round, Tile::Empty];
        let b = vec![Tile::Empty, Tile::Round];
        shove_right(& mut a);
        assert_eq!(a, b);

        let mut a = vec![Tile::Round, Tile::Round, Tile::Empty, Tile::Empty];
        let b = vec![Tile::Empty,Tile::Empty, Tile::Round, Tile::Round];
        shove_right(& mut a);
        assert_eq!(a, b);

        let mut a = vec![Tile::Round, Tile::Empty, Tile::Square, Tile::Empty];
        let b = vec![Tile::Empty, Tile::Round, Tile::Square, Tile::Empty];
        shove_right(& mut a);
        assert_eq!(a, b);

    }
}
