use std::{fs::read_to_string };

fn main() {
    read_to_string("1a.input")
        .unwrap()
        .lines()
        .for_each(|x| println!("{}\n", x));
}

fn is_digit(x: char) -> bool {
    x >= '0' && x <= '9'
}

//fn firstdgt(x: &str) -> Some(char) {
//    x.chars().into_iter().find(is_digit).map(|x| x.clone())
//}


#[cfg(test)]
mod tests {
    use crate::is_digit;

    #[test]
    fn is_digit_test() {
        assert!(is_digit('0'));
        assert!(is_digit('9'));
        assert!(is_digit('3'));
        assert!(is_digit('4'));
        assert!(!is_digit('a'));
        assert!(!is_digit(':'));
        assert!(!is_digit('/'));
    }
}
