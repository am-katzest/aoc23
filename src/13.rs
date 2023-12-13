
use itertools::Itertools;

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
    Ash,
    Rocks,
}

fn parse_tile(t: char) -> Tile {
    match t {
        '.' => Tile::Ash,
        'J' => Tile::Rocks,
        _ => panic!("the fuck is {t}"),
    }
}

type Field = Vec<Vec<Tile>>;

fn parse_field(f: &str) -> Field {
    f.lines().map(|x| x.chars().map(parse_tile).collect_vec()).collect_vec()
}

fn parse(f: &str) -> Vec<Field> {
    std::fs::read_to_string(f).unwrap().split("\n\n").map(parse_field).collect()
}

fn find_reflections<T>(u: Vec<T>) -> Vec<usize>
where
    T: PartialEq,
{
    u.iter()
        .tuple_windows()
        .enumerate()
        .filter_map(|(e, (a, b))| if a == b { Some(e) } else { None })
        .collect_vec()
}

fn check_reflection<T>(u: Vec<T>, start: usize) -> bool
where
    T: PartialEq,
{
    for i in 0.. {
        match (u.get(start - i), u.get(start + i + 1)) {
            (Some(x), Some(y)) if x != y => return false,
            (Some(x), Some(y)) if x == y => {}
            _ => return true,
        }
    }
    panic!()
}

fn meow<T>(u: Vec<T>) -> usize where T:PartialEq, T:Clone, {
    find_reflections(u.to_owned()).iter().copied().filter(|i|  check_reflection(u.to_owned(), *i)).next().unwrap()
}

fn main() {
    parse("inputs/13a");
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn reflection_test() {
        assert_eq!(vec![2], find_reflections(vec![1, 2, 3, 3, 5, 7]));
        assert_eq!(vec![0, 2], find_reflections(vec![2, 2, 3, 3, 5, 7]));
    }
    fn part1_test () {
        assert_eq!(1, meow(vec![2, 2]));
    }
}
