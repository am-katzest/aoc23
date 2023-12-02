use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Cubes {red: i32,green: i32, blue: i32}


fn parse_subsection(sub: &str) -> (i32, &str) {
    let (num, s) = sub.trim().split_once(" ").unwrap();
    (num.parse::<i32>().unwrap(),s)
}

// mutation, just for the variety
fn add_subsection_(acc: &mut Cubes, new: &str){
    let (n, s)= parse_subsection(new);
    match s{
        "red" => acc.red = n,
        "green" => acc.green = n,
        "blue" => acc.blue = n,
        &_ => panic!("unknown thing: {}", s),
    }
}

fn parse_section(section:&str) -> Cubes {
    let mut acc = Cubes { red: 0, green: 0, blue: 0};
    section.split(",").for_each(|x| add_subsection_(&mut acc, x));
    acc
}
    
fn parse_line(line:&str) -> (i32, Vec<Cubes>){
    let (head, rest) = line.split_once(":").unwrap();
    let nl = head.split(" ").last().unwrap().parse::<i32>().unwrap();
    let cubess = rest.split(";").map(parse_section).collect();
    (nl, cubess)
}

const MAX: &Cubes = &Cubes{red: 12, green: 13, blue:14};

/// each element of `a` is not less then it's `b` counterpart
fn fits(a: &Cubes, b: &Cubes) -> bool {
    a.red >= b.red && a.green >= b.green && a.blue >= b.blue
}

fn game_possible(xs: Vec<Cubes>) -> bool {
    xs.iter().all(|x| fits(MAX, x))
}

fn solve(f:&str) -> i32 {
    read_to_string(f)
        .unwrap()
        .lines()
        .map(parse_line)
        .filter_map(|(n, xs)| if game_possible(xs) {Some(n)} else {None})
        .sum()
}
    
fn main() {
    println!("part 1: {}", solve("2b.input"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_test() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let ss = " 1 red, 2 green, 6 blue";
        assert_eq!((1, vec![Cubes {red: 4, green:0, blue:3},
                            Cubes {red: 1, green:2, blue:6},
                            Cubes {red: 0, green:2, blue:0}]),
                   parse_line(line));
        assert_eq!(Cubes {red: 1, green:2, blue:6}, parse_section(ss));
        assert_eq!((6, "blue"), parse_subsection(" 6 blue"));
    }
    #[test]
    fn comp_test() {
        let a = Cubes {red: 0, green:2, blue:0};
        let b = Cubes {red: 0, green:3, blue:0};
        let c = Cubes {red: 0, green:0, blue:2};
        assert!(!fits(&a, &b));
        assert!(fits(&b, &a));
        assert!(fits(&a, &a));
        assert!(!fits(&a, &c));
        assert!(!fits(&c, &a));
    }
}
