use std::{fs::read_to_string, iter};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Thing{
    Number(i32),
    Symbol(char),
    Padding
}

fn parse_thing(l:&str) -> Thing {
    match l.parse::<i32>(){
        Ok(r) => Thing::Number(r),
        Err(_) => Thing::Symbol(l.chars().next().unwrap()),
    }
}

fn is_symbol(t:char) -> bool {
    t != '.' && !t.is_ascii_digit()
}

fn solve(f:&str) -> i32 {
    let lines: Vec<Vec<char>> = read_to_string(f).unwrap().lines().map(|x| x.chars().collect_vec()).collect_vec();
    let sizey = lines.len();
    let sizex = lines.first().unwrap().len();
    let mut acc = 0;
    let mut current:i32 = 0;
    let mut current_counts = false;
    let mut reading  = false;
    let get = |x:usize, y:usize| -> char {
        match lines.get(y) {
            None => '.',
            Some(line) =>  match line.get(x){
                None => '.',
                Some(&c) => c
            }
        }
    };

    let mut add_number = |dgt:char| {
        let n = dgt.to_digit(10).unwrap();
        current = current * 10 + n as i32;
    };

    let mut finish_reading_number = ||{
        if reading{
            if current_counts{acc+=current;}
            current_counts = false;
            current = 0;
            reading = false;
    }};

    let adjectant_to_symbol = |x:usize, y:usize|{
        for xi in x-1..=x+1{
            for yi in y-1..=y+1{
                if !(xi==x && yi==x){
                    if is_symbol(get(xi, yi)) {return true;}
                }
            }
        }
        false
    };

    for y in 0..sizey{
        for x in 0..sizex{
            let c = get(x, y);
            if c.is_digit(10) {
                add_number(c);
                if current_counts {
                    if adjectant_to_symbol(x, y){
                        current_counts = true;
                    }
                }
            } else {
                finish_reading_number()
            }
        }
    }
    acc
}

fn main() {
    println!("part 1: {}", solve("inputs/3a"));
//    println!("part 2: {}", solve2("inputs/2b"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_test() {
        assert_eq!(Thing::Number(617),parse_thing("617"));
        assert_eq!(Thing::Symbol('#'),parse_thing("#"));
    }
    #[test]
    fn line_parsing_test() {
                assert_eq!(vec![Thing::Number(617),Thing::Symbol('*')],parse_line("617*......"));
    }
}
