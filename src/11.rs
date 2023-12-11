fn count_distances(d: Vec<usize>, expansion: usize) -> usize {
    let mut ahead: usize = d.iter().sum();
    let mut acc: usize = 0;
    let mut visited = 0;
    for i in d {
        ahead -= i;
        visited += i;
        acc += visited * ahead * (if i == 0 { expansion } else { 1 })
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

fn solve(f: &str, expansion: usize) -> usize {
    let (h, v) = parse(f);
    count_distances(h, expansion) + count_distances(v, expansion)
}

fn main() {
    println!("part 1: {:?}", solve("inputs/11b", 2));
    println!("part 2: {:?}", solve("inputs/11b", 1_000_000));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn solver_test() {
        assert_eq!(0, count_distances(vec![5], 2));
        assert_eq!(0, count_distances(vec![5, 0], 2));
        assert_eq!(3, count_distances(vec![1, 3], 2));
        assert_eq!(4, count_distances(vec![2, 2], 2));
        assert_eq!(4, count_distances(vec![1, 1, 1], 2));
        assert_eq!(6, count_distances(vec![1, 2, 1], 2));
        assert_eq!(7, count_distances(vec![2, 1, 1], 2));
        assert_eq!(12, count_distances(vec![2, 0, 2], 2));
    }
    #[test]
    fn solving_test() {
        assert_eq!(374, solve("inputs/11a", 2));
        assert_eq!(9623138, solve("inputs/11b", 2));
    }
}
