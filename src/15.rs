use itertools::Itertools;

fn hash(s: String) -> usize {
    s.as_bytes().iter().fold(0, |acc, x| ((acc + *x as i32) * 17) % 256) as usize
}

fn read(f: &str) -> Vec<String> {
    std::fs::read_to_string(f)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| String::from(x))
        .collect()
}

struct Addr {
    label: String,
    hash: usize,
}

struct Op {
    addr: Addr,
    lens:Option<usize>
}

fn parse(f: String) -> Op {
    let u = f.split(|c| c == '=' || c == '-').collect_vec();
    let label = String::from(u[0].clone());
    let addr = Addr {
        hash: hash(label.clone()),
        label,
    };
    let lens = u[1].parse::<usize>().ok();
    Op {addr, lens}
}
type Box = Vec<(String, usize)>;

fn remove(b: &mut Box, label: String) {
    match b.iter().find_position(|(l, _)| *l == label) {
        None => {}
        Some((i, _)) => {
            b.remove(i);
        }
    }
}

fn add(b: &mut Box, label: String, lens: usize) {
    match b.iter().find_position(|(l, _)| *l == label) {
        None => b.push((label, lens)),
        Some((i, _)) => {
            b[i] = (label, lens);
        }
    }
}

fn apply(b: &mut Box, op:Op) {
        match op.lens {
            None => {
                remove(b, op.addr.label);
            }
            Some(lens) => {
                add(b, op.addr.label, lens);
            }
        }
}

fn part2(f: &str) -> usize {
    let mut boxes: Vec<Box> = std::iter::repeat(vec![]).take(256).collect();
    for s in read(f) {
        let op = parse(s);
        apply(&mut boxes[op.addr.hash], op);
    }
    boxes.into_iter().enumerate().map(|(b,x)| x.into_iter().enumerate().map(|(i, (_, l))| l*(i+1)*(b+1)).sum::<usize>()).sum()
}

fn part1(f: &str) -> i32 {
    read(f).into_iter().map(hash).map(|x| x as i32).sum()
}

fn main() {
    println!("part 1: {:?}", part1("inputs/15b"));
    println!("part 2: {:?}", part2("inputs/15b"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn hash_test() {
        assert_eq!(200, hash(String::from("H")));
        assert_eq!(52, hash(String::from("HASH")));
    }
    #[test]
    fn remove_test1() {
        let mut b: Box = vec![(String::from("meow"), 3)];
        remove(&mut b, String::from("meow"));
        assert_eq!(b, vec![]);
    }
    #[test]
    fn remove_test2() {
        let mut b: Box = vec![(String::from("mraow"), 3)];
        remove(&mut b, String::from("meow"));
        assert_eq!(b, vec![(String::from("mraow"), 3)]);
    }
    #[test]
    fn remove_test3() {
        let mut b: Box = vec![(String::from("meow"), 3), (String::from("mraow"), 3)];
        remove(&mut b, String::from("meow"));
        assert_eq!(b, vec![(String::from("mraow"), 3)]);
    }
    #[test]
    fn add_test1() {
        let mut b: Box = vec![];
        add(&mut b, String::from("meow"), 3);
        assert_eq!(b, vec![(String::from("meow"), 3)]);
    }
    #[test]
    fn add_test2() {
        let mut b: Box = vec![(String::from("meow"), 5)];
        add(&mut b, String::from("meow"), 3);
        assert_eq!(b, vec![(String::from("meow"), 3)]);
    }
    #[test]
    fn add_test3() {
        let mut b: Box = vec![(String::from("meow"), 5)];
        add(&mut b, String::from("mraow"), 3);
        assert_eq!(b, vec![(String::from("meow"), 5), (String::from("mraow"), 3)]);
    }
}
