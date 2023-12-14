use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

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

fn reverse(m: Map) -> Map {
    m.into_iter().rev().collect_vec()
}

fn flip(m: Map) -> Map {
    m.into_iter().map(|x| x.into_iter().rev().collect_vec()).collect_vec()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Round,
    Square,
}

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

fn shove_line_right(m: &mut Row) {
    let mut stones = 0;
    for i in 0..m.len() {
        match m[i] {
            Tile::Square => {
                for i in (i - stones)..i {
                    m[i] = Tile::Round;
                }
                stones = 0;
            }
            Tile::Round => {
                stones += 1;
                m[i] = Tile::Empty
            }
            _ => {}
        }
    }
    for i in (m.len() - stones)..m.len() {
        m[i] = Tile::Round;
    }
}
fn shr(a: &mut Map) {
    for i in 0..a.len() {
        shove_line_right(&mut a[i]);
    }
}

fn right(m: Map) -> Map {
    let mut a = m.clone();
    shr(&mut a);
    a
}

fn left(m: Map) -> Map {
    let mut a = flip(m);
    shr(&mut a);
    flip(a)
}

fn up(m: Map) -> Map {
    let mut a = transpose(reverse(m));
    shr(&mut a);
    reverse(transpose(a))
}

fn down(m: Map) -> Map {
    let mut a = transpose(m);
    shr(&mut a);
    transpose(a)
}

fn spin(m: Map) -> Map {
    right(down(left(up(m))))
}

fn parse(f: &str) -> Map {
    read_to_string(f)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(parse_tile).collect_vec())
        .collect_vec()
}

fn count_stones(u: &Row) -> usize {
    u.into_iter().filter(|&&x| x == Tile::Round).count()
}

fn calc_load(m: Map) -> usize {
    m.iter().rev().enumerate().map(|(i, x)| (i + 1) * count_stones(x)).sum()
}

fn part1(f: &str) -> usize {
    calc_load(up(parse(f)))
}

fn speen(m: Map, n: usize) -> Map {
    let mut a = m.clone();
    for _ in 0..n {
        a = spin(a);
    }
    a
}

fn part2(f: &str) -> usize {
    let mut visited: HashMap<Map, usize> = HashMap::new();
    let mut a = parse(f);
    let target = 1_000_000_000;
    for i in 0..target {
        match visited.get(&a) {
            None => {
                visited.insert(a.to_owned(), i);
            }
            Some(previous) => {
                let metacycle = i - previous;
                let remaining = target - i;
                let rem = remaining % metacycle;
                return calc_load(speen(a, rem));
            }
        }
        a = spin(a);
    }
    panic!()
}

fn main() {
    println!("part1: {}", part1("inputs/14b"));
    println!("part2: {}", part2("inputs/14b"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn shove_right_test() {
        let mut a = vec![Tile::Round, Tile::Empty];
        let b = vec![Tile::Empty, Tile::Round];
        shove_line_right(&mut a);
        assert_eq!(a, b);

        let mut a = vec![Tile::Round, Tile::Round, Tile::Empty, Tile::Empty];
        let b = vec![Tile::Empty, Tile::Empty, Tile::Round, Tile::Round];
        shove_line_right(&mut a);
        assert_eq!(a, b);

        let mut a = vec![Tile::Round, Tile::Empty, Tile::Square, Tile::Empty];
        let b = vec![Tile::Empty, Tile::Round, Tile::Square, Tile::Empty];
        shove_line_right(&mut a);
        assert_eq!(a, b);
    }
    #[test]
    fn shove_north_test() {
        assert_eq!(parse("inputs/14an"), up(parse("inputs/14a")));
        assert_eq!(parse("inputs/14a1"), spin(parse("inputs/14a")));
        assert_eq!(parse("inputs/14a2"), spin(spin(parse("inputs/14a"))));
    }
    #[test]
    fn part1_test() {
        assert_eq!(136, part1("inputs/14a"));
    }
    #[test]
    fn part2_test() {
        assert_eq!(64, part2("inputs/14a"));
    }
}
