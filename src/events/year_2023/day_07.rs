use crate::utils::*;
use anyhow::Result;

use self::slice_utils::GrpBy;

use super::super::AocDay;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Number(usize),
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CardP2 {
    Joker,
    Number(usize),
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand<Card: Copy + PartialEq + PartialOrd> {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

#[derive(Debug)]
struct Play<Card: Copy + PartialEq + PartialOrd>(pub Hand<Card>, pub usize);

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }

    fn parse_cards(&self, hand: &str) -> Vec<Card> {
        hand.chars()
            .map(|c| match c {
                '1'..='9' => Card::Number(c.to_digit(10).unwrap() as usize),
                'T' => Card::Number(10),
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => unreachable!(),
            })
            .collect()
    }

    fn parse_cards2(&self, hand: &str) -> Vec<CardP2> {
        hand.chars()
            .map(|c| match c {
                '1'..='9' => CardP2::Number(c.to_digit(10).unwrap() as usize),
                'T' => CardP2::Number(10),
                'J' => CardP2::Joker,
                'Q' => CardP2::Queen,
                'K' => CardP2::King,
                'A' => CardP2::Ace,
                _ => unreachable!(),
            })
            .collect()
    }

    fn parse_hand(&self, hand: &str) -> Hand<Card> {
        let cards = self.parse_cards(hand);

        let mut sorted_cards: Vec<_> = cards.to_vec();
        sorted_cards.sort();
        let mut group_lens: Vec<_> = sorted_cards
            .group_by(|a, b| a == b)
            .map(|grp| grp.len())
            .collect();

        // Sort by highest, hacky
        group_lens.sort();
        group_lens.reverse();

        match group_lens[..] {
            [5] => Hand::FiveOfAKind(cards),
            [4, ..] => Hand::FourOfAKind(cards),
            [3, 2] => Hand::FullHouse(cards),
            [3, ..] => Hand::ThreeOfAKind(cards),
            [2, 2, ..] => Hand::TwoPair(cards),
            [2, ..] => Hand::OnePair(cards),
            _ => Hand::HighCard(cards),
        }
    }

    fn parse_hand2(&self, hand: &str) -> Hand<CardP2> {
        let cards = self.parse_cards2(hand);

        let joker_count = cards.iter().filter(|c| matches!(c, CardP2::Joker)).count();
        // Otherwise we encounter no groups below
        if joker_count == 5 {
            return Hand::FiveOfAKind(cards);
        }

        let mut sorted_cards: Vec<_> = cards
            .iter()
            .filter(|c| !matches!(c, CardP2::Joker))
            .copied()
            .collect();
        sorted_cards.sort();
        let mut group_lens: Vec<_> = sorted_cards
            .group_by(|a, b| a == b)
            .map(|grp| grp.len())
            .collect();

        // Sort by highest, hacky
        group_lens.sort();
        group_lens.reverse();
        group_lens[0] += joker_count;

        match group_lens[..] {
            [5] => Hand::FiveOfAKind(cards),
            [4, ..] => Hand::FourOfAKind(cards),
            [3, 2] => Hand::FullHouse(cards),
            [3, ..] => Hand::ThreeOfAKind(cards),
            // [2, 2, ..] if joker_count == 1 => Hand::OnePair(cards), // Edge case due to reuse
            [2, 2, ..] => Hand::TwoPair(cards),
            [2, ..] => Hand::OnePair(cards),
            _ => Hand::HighCard(cards),
        }
    }
}
// A, K, Q, J, T
// 32T3K 765
impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut plays: Vec<_> = input
            .iter()
            .map(|s| {
                let (p1, p2) = s.split_once(' ').unwrap();

                Play(self.parse_hand(p1), p2.parse().unwrap())
            })
            .collect();

        plays.sort_by(|h1, h2| h1.0.cmp(&h2.0));

        let total_winnings: usize = plays
            .iter()
            .enumerate()
            .map(|(index, hand)| (index + 1) * hand.1)
            .sum();

        Ok(total_winnings.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut plays: Vec<_> = input
            .iter()
            .map(|s| {
                let (p1, p2) = s.split_once(' ').unwrap();

                Play(self.parse_hand2(p1), p2.parse().unwrap())
            })
            .collect();

        plays.sort_by(|h1, h2| h1.0.cmp(&h2.0));

        let total_winnings: usize = plays
            .iter()
            .enumerate()
            .map(|(index, hand)| (index + 1) * hand.1)
            .sum();

        Ok(total_winnings.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hand_parsing() {
        let input = "32T3K";

        let cards = Day::new().parse_cards(input);
        let hand = Day::new().parse_hand(input);
        assert_eq!(hand, Hand::OnePair(cards))
    }

    #[test]
    fn hand_parsing2() {
        let input = "QQQJA";

        let cards = Day::new().parse_cards(input);
        let hand = Day::new().parse_hand(input);
        assert_eq!(hand, Hand::ThreeOfAKind(cards))
    }

    #[test]
    fn hand_parsing3() {
        let input = "KK677";

        let cards = Day::new().parse_cards(input);
        let hand = Day::new().parse_hand(input);
        assert_eq!(hand, Hand::TwoPair(cards))
    }

    #[test]
    fn hand_eval() {
        let hand = Day::new().parse_hand("33332");
        let hand2 = Day::new().parse_hand("2AAAA");

        assert!(hand > hand2)
    }

    #[test]
    fn hand_parsing_part2() {
        let inputs = vec!["T55J5", "KTJJT", "QQQJA"];

        for input in inputs {
            let cards = Day::new().parse_cards2(input);
            let hand = Day::new().parse_hand2(input);
            assert_eq!(hand, Hand::FourOfAKind(cards))
        }
    }

    #[test]
    fn hand_parsing_part2_2() {
        let input = "KAJ45";

        let cards = Day::new().parse_cards2(input);
        let hand = Day::new().parse_hand2(input);
        assert_eq!(hand, Hand::OnePair(cards))
    }

    #[test]
    fn hand_parsing_part2_3() {
        let input = "2233J";

        let cards = Day::new().parse_cards2(input);
        let hand = Day::new().parse_hand2(input);
        assert_eq!(hand, Hand::FullHouse(cards))
    }

    #[test]
    fn hand_eval_part2() {
        let hand = Day::new().parse_hand2("QQQQ2");
        let hand2 = Day::new().parse_hand2("JKKK2");

        assert!(hand > hand2)
    }

    #[test]
    fn hand_eval_part2_2() {
        let hand = Day::new().parse_hand2("JJJJJ");
        let hand2 = Day::new().parse_hand2("KKKK1");

        assert!(hand > hand2)
    }

    #[test]
    fn hand_eval_part2_3() {
        let hand = Day::new().parse_hand2("22222");
        let hand2 = Day::new().parse_hand2("JJJJJ");

        assert!(hand > hand2)
    }
}

// T55J5, KTJJT, and QQQJA
