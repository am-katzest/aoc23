use std::fs::read_to_string;

static JUST_DIGITS: &[(char, &str)] = &[
    ('0', "0"),
    ('1', "1"),
    ('2', "2"),
    ('3', "3"),
    ('4', "4"),
    ('5', "5"),
    ('6', "6"),
    ('7', "7"),
    ('8', "8"),
    ('9', "9"),
];

static DIGITS_AND_STRINGS: &[(char, &str)] = &[
    ('0', "0"), ('0', "zero"),
    ('1', "1"), ('1', "one"),
    ('2', "2"), ('2', "two"),
    ('3', "3"), ('3', "three"),
    ('4', "4"), ('4', "four"),
    ('5', "5"), ('5', "five"),
    ('6', "6"), ('6', "six"),
    ('7', "7"), ('7', "seven"),
    ('8', "8"), ('8', "eight"),
    ('9', "9"), ('9', "nine"),
];

#[derive(Clone, Copy)]
enum End {
    Left,
    Right,
}

fn solve(matchers: &[(char, &str)], f: &str) -> i32 {
    read_to_string(f)
        .unwrap()
        .lines()
        .map(|x| edge_digits(matchers, x))
        .map(dgts_to_int)
        .sum()
}

fn main() {
    println!("part 1: {}", solve(JUST_DIGITS, "1b.input"));
    println!("part 2: {}", solve(DIGITS_AND_STRINGS, "1b.input"));
}

fn char_to_int(a: char) -> i32 {
    (a.to_digit(10).unwrap() - '0'.to_digit(10).unwrap()) as i32
}

fn dgts_to_int(pair: (char, char)) -> i32 {
    let (tens, ones) = pair;
    char_to_int(tens) * 10 + char_to_int(ones)
}

fn index(end: End, string: &str, matcher: (char, &str)) -> Option<(char, usize)> {
    let (c, sub) = matcher;
    match end {
        End::Left => string.find(sub).map(|position| (c, position)),
        End::Right => {
            string.rfind(sub).map(|position| {
                (c, string.len() - sub.len() - position)
            })
        }
    }
}

fn select(matchers: &[(char, &str)], string: &str, end: End) -> char {
    matchers
        .iter()
        .filter_map(|m| index(end, string, *m))
        .min_by_key(|(_, dist)| *dist)
        .unwrap()
        .0
}

fn edge_digits(matchers: &[(char, &str)], x: &str) -> (char, char) {
    (
        select(matchers, x, End::Left),
        select(matchers, x, End::Right),
    )
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn to_int_test() {
        assert_eq!(3, char_to_int('3'));
        assert_eq!(33, dgts_to_int(('3', '3')));
        assert_eq!(30, dgts_to_int(('3', '0')));
    }
    #[test]
    fn parta_test() {
        assert_eq!(142, solve(JUST_DIGITS, "1a.input"))
    }
    #[test]
    fn partb_test() {
        assert_eq!(281, solve(DIGITS_AND_STRINGS, "1c.input"))
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
        assert_eq!('3', select(DIGITS_AND_STRINGS, "3", End::Right));
        assert_eq!('1', select(DIGITS_AND_STRINGS, "3meow1", End::Right));
        assert_eq!('3', select(DIGITS_AND_STRINGS, "3meow1", End::Left));
        assert_eq!('1', select(DIGITS_AND_STRINGS, "onexxxxxtwo", End::Left));
        assert_eq!('2', select(DIGITS_AND_STRINGS, "onexxxxxtwo", End::Right));
        assert_eq!('6', select(JUST_DIGITS, "onex56xxtwo", End::Right));
        assert_eq!('5', select(JUST_DIGITS, "onex56xxtwo", End::Left));
    }

}
