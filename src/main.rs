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
    
fn parse_line(line:&str){
    //let (nr, rest) = line.split_once(":").unwrap();
    ()
}

fn read_file(f:&str) {
//    read_to_string(f)
//        .unwrap()
//        .lines()
//        .map(parse_line)
}
    
fn main() {
    //println!("part 1: {}", solve(JUST_DIGITS, "1b.input"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_test() {
        assert_eq!(Cubes {red: 1, green:2, blue:6}, parse_section(" 1 red, 2 green, 6 blue"));
        assert_eq!((6, "blue"), parse_subsection(" 6 blue"));
    }
}
