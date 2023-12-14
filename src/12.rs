use std::fs::read_to_string;

use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Spring {
    Damaged,
    Operational,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
struct Row {
    springs: Vec<Spring>,
    ecc: Vec<usize>,
}

fn parse_spring(s: char) -> Spring {
    match s {
        '?' => Spring::Unknown,
        '#' => Spring::Damaged,
        '.' => Spring::Operational,
        _ => panic!("wrong spring {s}"),
    }
}

fn parse_line(line: &str) -> Row {
    let (l, r) = line.split(' ').collect_tuple().unwrap();
    let springs = l.chars().map(parse_spring).collect_vec();
    let ecc = r.split(',').map(|x| x.parse().unwrap()).collect_vec();
    Row { springs, ecc }
}

fn min_len(ecc: Vec<usize>) -> usize {
    ecc.iter().sum::<usize>() + ecc.len() - 1
}

fn freedom(r: Row) -> usize {
    // how much we can move the first element towards
    // the end and still be able to potentially succeed
    println!("{}, {}", r.springs.len(), min_len(r.ecc.to_owned()));
    r.springs.len() - min_len(r.ecc)
}

// fn reverse(r: Row) -> Row {
//     let mut u = r.clone();
//     u.ecc.reverse();
//     u.springs.reverse();
//     u
// }

// fn try_snip(r: Row) -> Option<Vec<Row>> {
//     // attempts to snip anything it can from the front
//     assert!(r.ecc.len() > 0);
//     let first: usize = *r.ecc.first()?;
//     let mut possible = vec![];
//     let mut remaining = first;
//     let mut sure_strat:Option<usize> = None;
//     let mut possible_start:Option<usize> = None;
//     let mut yielding = false;
//     for i in 0.. {
//         match r.springs[i] {
//             Spring::Damaged => {
//                 assert!(remaining == first);
//             },
//             Spring::Operational => {remaining += 1}
//             Spring::Unknown => {remaining += 1} // can be
//         }
//     }
//     Some(possible)
// }

fn cut_first(r: Row, offset: usize) -> Row {
    // todo use slices
    let springs = r.springs.into_iter().skip(offset + r.ecc[0] + 1).collect_vec();
    let ecc = r.ecc.into_iter().skip(1).collect_vec();
    Row { ecc, springs }
}

fn feasible(r: Row, offset: usize) -> bool {
    let size = r.ecc[0];
    for i in 0..offset {
        if r.springs[i] == Spring::Damaged {
            return false;
        }
    }
    for i in offset..offset + size {
        if r.springs[i] == Spring::Operational {
            return false;
        }
    }
    (r.springs.len() == (offset + size)) || (r.springs[offset + size] != Spring::Damaged)
}

fn count_possibilities_brute_force(r: Row) -> usize {
    if r.ecc.len() == 0 {
        if r.springs.iter().any(|&x| x == Spring::Damaged) {
            return 0; // impossible
        }
        // recursion end
        return 1;
    }
    (0..=freedom(r.clone()))
        .map(|i| {
            if feasible(r.to_owned(), i) {
                count_possibilities_brute_force(cut_first(r.to_owned(), i))
            } else {
                0
            }
        })
        .sum()
}

fn part1(f: &str) -> usize {
    read_to_string(f)
        .unwrap()
        .lines()
        .map(parse_line)
        .map(count_possibilities_brute_force)
        .inspect(|x| println!("{x}"))
        .sum()
}
fn unfold(r: Row) -> Row {
    let springs = r.springs.iter().copied().cycle().take(5 * r.springs.len()).collect_vec();
    let ecc = r.ecc.iter().copied().cycle().take(5 * r.ecc.len()).collect_vec();
    Row { ecc, springs }
}
fn part2(f: &str) -> usize {
    read_to_string(f)
        .unwrap()
        .lines()
        .map(parse_line)
        .map(unfold)
        .inspect(|x| println!("{:?}", x))
        .map(count_possibilities_brute_force)
        .inspect(|x| println!("{x}"))
        .sum()
}

fn main() {
    println!("part 1: {}", part1("inputs/12b"));
    println!("part 2: {}", part2("inputs/12a"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn parser_test() {
        assert_eq!(
            Row {
                springs: vec![Spring::Damaged, Spring::Damaged, Spring::Unknown, Spring::Operational],
                ecc: vec![2, 1]
            },
            parse_line("##?. 2,1")
        );
    }
    #[test]
    fn dumb_solver_test() {
        assert_eq!(1, count_possibilities_brute_force(parse_line("? 1")));
        assert_eq!(2, count_possibilities_brute_force(parse_line("?? 1")));
        assert_eq!(1, count_possibilities_brute_force(parse_line("??? 1,1")));
        assert_eq!(1, count_possibilities_brute_force(parse_line("???? 2,1")));
        assert_eq!(1, count_possibilities_brute_force(parse_line("???? 1,2")));
        assert_eq!(1, count_possibilities_brute_force(parse_line("#..# 1,1")));
        assert_eq!(1, count_possibilities_brute_force(parse_line("?..? 1,1")));
        assert_eq!(1, count_possibilities_brute_force(parse_line(".?.? 1,1")));
        assert_eq!(4, count_possibilities_brute_force(parse_line("??.?? 1,1")));
        assert_eq!(8, count_possibilities_brute_force(parse_line("??.??.?? 1,1,1")));
        assert_eq!(3, count_possibilities_brute_force(parse_line("?.?.? 1,1")));
        assert_eq!(2, count_possibilities_brute_force(parse_line("#.?.? 1,1")));

        assert_eq!(
            1,
            count_possibilities_brute_force(Row {
                springs: vec![Spring::Damaged],
                ecc: vec![1]
            })
        );
        assert_eq!(
            1,
            count_possibilities_brute_force(Row {
                springs: vec![Spring::Unknown],
                ecc: vec![]
            })
        );
        assert_eq!(
            0,
            count_possibilities_brute_force(Row {
                springs: vec![Spring::Damaged],
                ecc: vec![]
            })
        );

        assert_eq!(6, count_possibilities_brute_force(parse_line("????? 1,1")));
        assert_eq!(6, count_possibilities_brute_force(parse_line("???????? 1,4")));
        assert_eq!(2, count_possibilities_brute_force(parse_line("???#? 1,1")));
        assert_eq!(3, count_possibilities_brute_force(parse_line("????#? 1,1")));
        assert_eq!(6, count_possibilities_brute_force(parse_line("??????#? 1,1,1")));
        assert_eq!(6, count_possibilities_brute_force(parse_line("?????????#? 1,4,1")));
    }
    #[test]
    fn freedom_test() {
        assert_eq!(0, freedom(parse_line("? 1")));
        assert_eq!(1, freedom(parse_line("?? 1")));
        assert_eq!(0, freedom(parse_line("?? 2")));
        assert_eq!(0, freedom(parse_line("??? 1,1")));
        assert_eq!(1, freedom(parse_line("???? 1,1")));
    }

    #[test]
    fn cutting_test() {
        assert_eq!(parse_line("??? 1"), cut_first(parse_line("?????? 2,1"), 0));
        assert_eq!(parse_line("??? 1"), cut_first(parse_line("?????? 1,1"), 1));
    }

    #[test]
    fn feasible_test() {
        assert_eq!(true, feasible(parse_line("? 1"), 0));
        assert_eq!(true, feasible(parse_line("? 1"), 0));
        assert_eq!(true, feasible(parse_line("# 1"), 0));
        assert_eq!(false, feasible(parse_line(". 1"), 0));
        assert_eq!(false, feasible(parse_line("## 1"), 0));

        assert_eq!(true, feasible(parse_line("## 2"), 0));
        assert_eq!(false, feasible(parse_line("#. 2"), 0));
        assert_eq!(false, feasible(parse_line(".# 2"), 0));

        assert_eq!(true, feasible(parse_line(".#. 1"), 1));
        assert_eq!(true, feasible(parse_line(".# 1"), 1));
        assert_eq!(false, feasible(parse_line("##. 1"), 1));
        assert_eq!(false, feasible(parse_line("## 1"), 1));

        assert_eq!(true, feasible(parse_line("..# 1"), 2));
        assert_eq!(false, feasible(parse_line("#.# 1"), 2));
        assert_eq!(false, feasible(parse_line("..## 1"), 2));
        assert_eq!(true, feasible(parse_line(".?# 1"), 2));
        assert_eq!(true, feasible(parse_line("??# 1"), 2));
        assert_eq!(true, feasible(parse_line("?.# 1"), 2));
        assert_eq!(false, feasible(parse_line("#?# 1"), 2));
        assert_eq!(true, feasible(parse_line(".?#? 1"), 2));
    }
    #[test]
    fn part1_test() {
        assert_eq!(21, part1("inputs/12a"));
        assert_eq!(7286, part1("inputs/12b"));
    }
    #[test]
    fn unfold_test() {
        assert_eq!(unfold(parse_line("..# 1")), parse_line("..#..#..#..#..# 1,1,1,1,1"));
    }
}
