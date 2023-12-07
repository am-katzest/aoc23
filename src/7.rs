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
    Joker,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    t: HandType,
    cards: Vec<Card>,
}

fn classify(cards: &Vec<Card>) -> HandType {
    let jokers = cards.iter().filter(|&&x| x == Card::Joker).count();
    let counts: Vec<usize> = cards
        .clone()
        .into_iter()
        .filter(|&x| x != Card::Joker)
        .sorted() // A A B C D
        .group_by(|x| x.clone())
        .into_iter() // (A, [A A]) (B, [B]) (C, [C]) (D, [D])
        .map(|(_, a)| a.count()) // wounder if there's a function combining them
        .sorted()
        .rev() // 2 1 1 1
        .collect();
    if jokers > 0 {
        println!("-->{}", jokers);
    }
    match (counts.len(), counts.first().unwrap_or(&0) + jokers) {
        // number of unique cards, highest count
        (_, 5) => HandType::FiveofAKind,
        (2, 4) => HandType::FourofAKind,
        (2, 3) => HandType::FullHouse,
        (3, 2) => HandType::TwoPair,
        (3, 3) => HandType::ThreeOfAKind,
        (4, 2) => HandType::OnePair,
        (_, _) => HandType::HighCard,
    }
}

fn parse_card(c: char, j: Card) -> Card {
    match c {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => j,
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("Invalid card character: {}", c),
    }
}

fn parse_hand(h: &str, j: Card) -> Hand {
    let cards = h.chars().map(|x| parse_card(x, j)).collect::<Vec<Card>>();
    let t = classify(&cards);
    Hand { cards, t }
}

fn parse_line(l: &str, j: Card) -> (Hand, usize) {
    let (f, s) = l.split(" ").collect_tuple().unwrap();
    (parse_hand(f, j), s.parse().unwrap())
}

fn parse(f: &str, j: Card) -> Vec<(Hand, usize)> {
    read_to_string(f)
        .unwrap()
        .lines()
        .map(|x| parse_line(x, j))
        .collect()
}

fn solve(deck: Vec<(Hand, usize)>) -> usize {
    deck.into_iter()
        .sorted()
        .rev()
        .enumerate()
        .map(|(i, (_, score))| (i + 1) * score)
        .sum()
}

fn main() {
    println!("part 1: {:?}", solve(parse("inputs/7b", Card::Jack)));
    println!("part 2: {:?}", solve(parse("inputs/7b", Card::Joker)));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn hand_parsing_test() {
        let jack = |x| parse_hand(x, Card::Jack);
        let joker = |x| parse_hand(x, Card::Joker);
        assert_eq!(HandType::FiveofAKind, jack("33333").t);
        assert_eq!(HandType::FullHouse, jack("32323").t);
        assert_eq!(HandType::FourofAKind, jack("44442").t);
        assert_eq!(HandType::ThreeOfAKind, jack("444AQ").t);
        assert_eq!(HandType::TwoPair, jack("2323A").t);
        assert_eq!(HandType::OnePair, jack("55234").t);
        assert_eq!(HandType::HighCard, jack("62345").t);

        assert_eq!(HandType::FiveofAKind, joker("5555J").t);
        assert_eq!(HandType::FiveofAKind, joker("555JJ").t);
        assert_eq!(HandType::FiveofAKind, joker("55JJJ").t);
        assert_eq!(HandType::FiveofAKind, joker("5JJJJ").t);
        assert_eq!(HandType::FiveofAKind, joker("JJJJJ").t);

        assert_eq!(HandType::FourofAKind, joker("5553J").t);
        assert_eq!(HandType::FourofAKind, joker("553JJ").t);
        assert_eq!(HandType::FourofAKind, joker("53JJJ").t);

        assert_eq!(HandType::FullHouse, joker("5533J").t);

        assert_eq!(HandType::ThreeOfAKind, joker("234JJ").t);

        assert_eq!(HandType::OnePair, joker("5234J").t);
    }
    #[test]
    fn part1() {
        assert_eq!(6440, solve(parse("inputs/7a", Card::Jack)));
        assert_eq!(246409899, solve(parse("inputs/7b", Card::Jack)));
    }
    #[test]
    fn part2() {
        assert_eq!(5905, solve(parse("inputs/7a", Card::Joker)));
        assert_eq!(244848487, solve(parse("inputs/7b", Card::Joker)));
    }
}
