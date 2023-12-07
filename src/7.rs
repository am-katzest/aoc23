use std::fs::read_to_string;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveofAKind,
    FourofAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
}

#[derive(Clone, Debug, PartialEq)]
struct Hand {
    t: HandType,
    cards: Vec<Card>,
}

fn classify(cards: &Vec<Card>) -> HandType {
    let counts: Vec<usize> = cards
        .clone()
        .into_iter()
        .sorted() // A A B C D
        .group_by(|x| x.clone())
        .into_iter()  // (A, [A A]) (B, [B]) (C, [C]) (D, [D])
        .map(|(_, a)| a.count()) // wounder if there's a function combining them
        .sorted()
        .rev() // 2 1 1 1
        .collect();
    println!("{:?} {:?}", cards, counts);
    match (counts.len(), counts.first().unwrap()) {
        // number of unique cards, highest count
        (1, 5) => HandType::FiveofAKind,
        (2, 4) => HandType::FourofAKind,
        (2, 3) => HandType::FullHouse,
        (3, 2) => HandType::TwoPair,
        (3, 3) => HandType::ThreeOfAKind,
        (4, 2) => HandType::OnePair,
        (_, _) => HandType::HighCard
    }
}

fn parse_card(c: char) -> Card {
    match c {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => Card::Jack,
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        '1' => Card::One,
        _ => panic!("Invalid card character: {}", c),
    }
}

fn parse_hand(h: &str) -> Hand {
    let cards = h.chars().map(parse_card).collect::<Vec<Card>>();
    let t = classify(&cards);
    Hand { cards, t }
}

fn parse_line(l: &str) -> (Hand, i32) {
    let (f, s) = l.split(" ").collect_tuple().unwrap();
    (parse_hand(f), s.parse::<i32>().unwrap())
}

fn parse(f: &str) -> Vec<(Hand, i32)> {
    read_to_string(f).unwrap().lines().map(parse_line).collect()
}

fn solve(f: &str) -> i64 {
    for i in parse(f){
        println!("{:?}", i);
    }
    3
}

fn main() {
    println!("part 1: {:?}", solve("inputs/7a"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn hand_parsing_test() {
        assert_eq!(HandType::FiveofAKind, parse_hand("33333").t);
        assert_eq!(
            vec![Card::One, Card::Two, Card::Three, Card::Four, Card::Five],
            parse_hand("12345").cards
        );
    }
}
