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
static MATCHERS: &[(char, &str)] =
    &[('0', "0"), ('0', "zero"),
      ('1', "1"), ('1', "one"),
      ('2', "2"), ('2', "two"),
      ('3', "3"), ('3', "three"),
      ('4', "4"), ('4', "four"),
      ('5', "5"), ('5', "five"),
      ('6', "6"), ('6', "six"),
      ('7', "7"), ('7', "seven"),
      ('8', "8"), ('8', "eight"),
      ('9', "9"), ('9', "nine")];

#[derive(Clone, Copy)]
enum End {
    Left,
    Right,
}

fn index(end:End, string: &str, matcher: (char, &str)) -> Option<(char, usize)>{
    let (c, sub) = matcher;
    match end{
        End::Left => string.find(sub).map(|position| (c, position)),
        End::Right => string.rfind(sub).map(|position| (c, string.len() - sub.len() - position)),
    }
}

fn select(string: &str, end:End) -> char{
    MATCHERS.iter()
    .filter_map(|m| index(end, string, *m))
    .min_by_key(|(_, dist)| *dist)
    .unwrap()
    .0
}

#[cfg(test)]
mod tests {
    use crate::{is_digit, edge_digits, char_to_int, dgts_to_int, part1, index, End, select};

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
    #[test]
    fn index_test() {
        assert_eq!(Some(('3', 0)), index(End::Left, "meow", ('3', "me")));
        assert_eq!(Some(('3', 2)), index(End::Left, "bemeow", ('3', "me")));
        assert_eq!(None, index(End::Left, "bemeow", ('3', "niema")));
        assert_eq!(Some(('3', 0)), index(End::Right, "bemeow", ('3', "ow")));
        assert_eq!(Some(('3', 1)), index(End::Right, "bemeow", ('3', "eo")));
    }
    #[test]
    fn select_test() {
        assert_eq!('3', select("3", End::Right));
        assert_eq!('1', select("3meow1", End::Right));
        assert_eq!('3', select("3meow1", End::Left));
        assert_eq!('1', select("onexxxxxtwo", End::Left));
        assert_eq!('2', select("onexxxxxtwo", End::Right));
    }

}
