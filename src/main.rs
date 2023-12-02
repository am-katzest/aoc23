use std::{fs::read_to_string };

fn main() {
    read_to_string("1a.input")
        .unwrap()
        .lines()
        .for_each(|x| println!("{}\n", x));
}

fn is_digit(x: &char) -> bool {
    x >= &'0' && x <= &'9'
}

fn edge_digits(x: &str) -> (char,char) {
    (x.chars().find(is_digit).unwrap(),
     x.chars().rev().find(is_digit).unwrap())
}


#[cfg(test)]
mod tests {
    use crate::{is_digit, edge_digits};

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
}
