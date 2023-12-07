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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

fn parse_line(l: &str) -> (Hand, usize) {
    let (f, s) = l.split(" ").collect_tuple().unwrap();
    (parse_hand(f), s.parse().unwrap())
}

fn parse(f: &str) -> Vec<(Hand, usize)> {
    read_to_string(f).unwrap().lines().map(parse_line).collect()
}

fn solve(f: &str) -> usize {
    parse(f).into_iter().sorted().rev().enumerate().map(|(i, (_, score))| (i+1)*score).sum()
}

fn main() {
    println!("part 1: {:?}", solve("inputs/7b"));
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn hand_parsing_test() {
        assert_eq!(HandType::FiveofAKind, parse_hand("33333").t);
        assert_eq!(HandType::FullHouse, parse_hand("32323").t);
        assert_eq!(HandType::FourofAKind, parse_hand("44441").t);
        assert_eq!(HandType::ThreeOfAKind, parse_hand("444AJ").t);
        assert_eq!(HandType::TwoPair, parse_hand("1313A").t);
        assert_eq!(HandType::OnePair, parse_hand("11234").t);
        assert_eq!(HandType::HighCard, parse_hand("12345").t);
        assert_eq!(
            vec![Card::One, Card::Two, Card::Three, Card::Four, Card::Five],
            parse_hand("12345").cards
        );
    }
    #[test]
    fn part1() {
        assert_eq!(6440, solve("inputs/7a"));
        assert_eq!(246409899, solve("inputs/7b"));
    }
}
