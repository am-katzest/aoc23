
fn hash(s: String) -> i32 {
    s.as_bytes().iter().fold(0, |acc, x| ((acc + *x as i32) * 17) % 256)
}

fn read(f: &str) -> Vec<String> {
    std::fs::read_to_string(f).unwrap().lines().next().unwrap().split(",").map(|x| String::from(x)).collect()
}

fn part1(f: &str) -> i32 {
    read(f).into_iter().map(hash).sum()
}

fn main() {
    println!("part 1: {:?}", part1("inputs/15b"));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn hash_test() {
        assert_eq!(200, hash(String::from("H")));
        assert_eq!(52, hash(String::from("HASH")));
    }
}
