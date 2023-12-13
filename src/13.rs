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

type Field = Vec<Vec<char>>;

fn parse_field(f: &str) -> Field {
    f.lines().map(|x| x.chars().collect_vec()).collect_vec()
}

fn parse(f: &str) -> Vec<Field> {
    std::fs::read_to_string(f).unwrap().split("\n\n").map(parse_field).collect()
}

fn defects(a: &Vec<char>, b: &Vec<char>) -> usize {
    std::iter::zip(a, b).filter(|(a, b)| a != b).count()
}

fn count_defects(u: Field, start: usize) -> usize {
    let mut acc = 0;
    for i in 0..=start {
        match (u.get(start - i), u.get(start + i + 1)) {
            (Some(x), Some(y)) => acc += defects(x, y),
            _ => break,
        }
    }
    acc
}

fn above_reflection(u: &Field, defects: usize) -> usize {
    (0..(u.len() - 1))
        .find(|i| count_defects(u.to_owned(), *i) == defects)
        .map(|x| x + 1)
        .unwrap_or(0)
}

fn summarize(u: Field, defects: usize) -> usize {
    100 * above_reflection(&u, defects) + above_reflection(&transpose(u), defects)
}

fn solve(f: &str, defects: usize) -> usize {
    parse(f).into_iter().map(|x| summarize(x, defects)).sum()
}

fn main() {
    println!("part1: {}", solve("inputs/13b", 0));
    println!("part2: {}", solve("inputs/13b", 1));
}
