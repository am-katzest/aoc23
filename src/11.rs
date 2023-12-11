fn count_distances(d: Vec<usize>) -> usize {
    let mut ahead: usize = d.iter().sum();
    let mut acc: usize = 0;
    let mut visited = 0;
    for i in d {
        ahead -= i;
        visited += i;
        acc += visited * ahead * (if i == 0 { 2 } else { 1 })
    }
    acc
}

fn parse(f: &str) -> (Vec<usize>, Vec<usize>) {
    let s = std::fs::read_to_string(f).unwrap();
    let h: Vec<usize> = s.lines().map(|x| x.chars().filter(|&x| x == '#').count()).collect();
    let v = (0..h.len())
        .map(|i| s.lines().filter(|x| x.chars().nth(i).unwrap() == '#').count())
        .collect();
    (h, v)
}
fn part1(f: &str) -> usize {
    let (h, v) = parse(f);
    count_distances(h) + count_distances(v)
}

fn main() {
    println!("part 1: {:?}", part1("inputs/11b"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn solver_test() {
        assert_eq!(0, count_distances(vec![5]));
        assert_eq!(0, count_distances(vec![5, 0]));
        assert_eq!(3, count_distances(vec![1, 3]));
        assert_eq!(4, count_distances(vec![2, 2]));
        assert_eq!(4, count_distances(vec![1, 1, 1]));
        assert_eq!(6, count_distances(vec![1, 2, 1]));
        assert_eq!(7, count_distances(vec![2, 1, 1]));
        assert_eq!(12, count_distances(vec![2, 0, 2]));
    }
    #[test]
    fn part1_test() {
        assert_eq!(374, part1("inputs/11a"));
        assert_eq!(9623138, part1("inputs/11a"));
    }
}
