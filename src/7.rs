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
        .into_iter()  // (A, [A A]) (B, [B]) (C, [C]) (D, [D])
        .map(|(_, a)| a.count()) // wounder if there's a function combining them
        .sorted()
        .rev() // 2 1 1 1
        .collect();
    if jokers > 0 {println!("-->{}", jokers);}
    match (counts.len(), counts.first().unwrap_or(&0) + jokers) {
        // number of unique cards, highest count
        (_, 5) => HandType::FiveofAKind,
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
        'X' => Card::Joker,
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

fn parse_hand(h: &str) -> Hand {
    let cards = h.chars().map(parse_card).collect::<Vec<Card>>();
    let t = classify(&cards);
    Hand { cards, t }
}

fn parse_line(l: &str) -> (Hand, usize) {
    let (f, s) = l.split(" ").collect_tuple().unwrap();
    (parse_hand(f), s.parse().unwrap())
}

fn parse(f: &str, j: Card) -> Vec<(Hand, usize)> {
    let r = match j {
        Card::Joker => 'X',
        Card::Jack => 'J',
        _ => panic!()
    };
    read_to_string(f).unwrap().chars().map(|c| if c == 'J' {r} else {c}).collect::<String>() .lines().map(parse_line).collect()
}

fn solve(deck: Vec<(Hand, usize)>) -> usize {
    deck.into_iter().sorted().rev().enumerate().map(|(i, (_, score))| (i+1)*score).sum()
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
        assert_eq!(HandType::FiveofAKind, parse_hand("33333").t);
        assert_eq!(HandType::FullHouse, parse_hand("32323").t);
        assert_eq!(HandType::FourofAKind, parse_hand("44442").t);
        assert_eq!(HandType::ThreeOfAKind, parse_hand("444AQ").t);
        assert_eq!(HandType::TwoPair, parse_hand("2323A").t);
        assert_eq!(HandType::OnePair, parse_hand("55234").t);
        assert_eq!(HandType::HighCard, parse_hand("62345").t);

        assert_eq!(HandType::FiveofAKind, parse_hand("5555X").t);
        assert_eq!(HandType::FiveofAKind, parse_hand("555XX").t);
        assert_eq!(HandType::FiveofAKind, parse_hand("55XXX").t);
        assert_eq!(HandType::FiveofAKind, parse_hand("5XXXX").t);
        assert_eq!(HandType::FiveofAKind, parse_hand("XXXXX").t);

        assert_eq!(HandType::FourofAKind, parse_hand("5553X").t);
        assert_eq!(HandType::FourofAKind, parse_hand("553XX").t);
        assert_eq!(HandType::FourofAKind, parse_hand("53XXX").t);

        assert_eq!(HandType::FullHouse, parse_hand("5533X").t);

        assert_eq!(HandType::ThreeOfAKind, parse_hand("234XX").t);

        assert_eq!(HandType::OnePair, parse_hand("5234X").t);


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
