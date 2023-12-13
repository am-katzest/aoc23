
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
        '#' => Tile::Rocks,
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
        if i > start {
            return true;
        }
        match (u.get(start - i), u.get(start + i + 1)) {
            (Some(x), Some(y)) if x != y => return false,
            (Some(x), Some(y)) if x == y => {}
            _ => return true,
        }
    }
    panic!()
}

fn above_reflection<T>(u: &Vec<T>) -> usize where T:PartialEq, T:Clone, {
    match find_reflections(u.to_owned()).iter().copied().filter(|i|  check_reflection(u.to_owned(), *i)).next() {
        Some(x) => x + 1,
        None => 0
    }
}

fn summarize(u: Field) -> usize {
    100 * above_reflection(&u) + above_reflection(&transpose(u))
}
fn  part1(f: &str) -> usize {
    parse(f).into_iter().map(summarize).inspect(|x| println!("{:?}", x)).sum()
}

fn main() {
    println!("part1: {}", part1("inputs/13b"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn reflection_test() {
        assert_eq!(vec![2], find_reflections(vec![1, 2, 3, 3, 5, 7]));
        assert_eq!(vec![0, 2], find_reflections(vec![2, 2, 3, 3, 5, 7]));
    }
    #[test]
    fn part1_test () {
        assert_eq!(1, above_reflection(&vec![2, 2]));
        assert_eq!(2, above_reflection(&vec![3, 2, 2]));
        assert_eq!(2, above_reflection(&vec![3, 2, 2, 3]));
        assert_eq!(1, above_reflection(&vec![2, 2, 3]));
        assert_eq!(1, above_reflection(&vec![2, 2, 3]));
        assert_eq!(1, above_reflection(&vec![2, 2, 3]));
        assert_eq!(0, above_reflection(&vec![2, 3, 1, 1, 3, 22, 33]));
        assert_eq!(5, above_reflection(&vec![4, 3, 2, 1, 0, 0, 1, 2, 3]));
    }
}
