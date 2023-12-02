use std::fs::read_to_string;

fn part1(f: &str) -> i32 {
    read_to_string(f)
    .unwrap()
    .lines()
    .map(edge_digits)
    .map(dgts_to_int)
    .sum()
}

fn main() {
    println!("part 1: {}", part1("1b.input"));
}

fn is_digit(x: &char) -> bool {
    x >= &'0' && x <= &'9'
}

fn edge_digits(x: &str) -> (char,char) {
    (x.chars().find(is_digit).unwrap(),
     x.chars().rev().find(is_digit).unwrap())
}

fn char_to_int(a:char) -> i32 {
    (a.to_digit(10).unwrap() - '0'.to_digit(10).unwrap()) as i32
}

fn dgts_to_int(a: (char, char)) -> i32 {
    match a{
        (tens, digit) => char_to_int(tens) * 10 + char_to_int(digit)
    }
}

#[cfg(test)]
mod tests {
    use crate::{is_digit, edge_digits, char_to_int, dgts_to_int, part1};

    #[test]
    fn is_digit_test() {
        assert!(is_digit(&'0'));
        assert!(is_digit(&'9'));
        assert!(is_digit(&'3'));
        assert!(is_digit(&'4'));
        assert!(!is_digit(&'a'));
        assert!(!is_digit(&':'));
        assert!(!is_digit(&'/'));
    }
    #[test]
    fn firstdgt_test() {
        assert_eq!(('1','1'), edge_digits("1"));
        assert_eq!(('1','1'), edge_digits("meow1"));
        assert_eq!(('1','1'), edge_digits("1meow"));
        assert_eq!(('1','1'), edge_digits("1meow1"));
        assert_eq!(('2','3'), edge_digits("23"));
        assert_eq!(('2','3'), edge_digits("meow2meow3meow"));

    }
    #[test]
    fn to_int_test() {
        assert_eq!(3, char_to_int('3'));
        assert_eq!(33, dgts_to_int(('3','3')));
        assert_eq!(30, dgts_to_int(('3','0')));
    }
    #[test]
    fn parta_test() {
        assert_eq!(142, part1("1a.input"))
    }
}
