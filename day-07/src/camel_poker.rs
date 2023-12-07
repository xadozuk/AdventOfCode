use std::{cmp::Ordering, collections::HashMap, fmt::{Formatter, Error}};

const CARDS_ORDER_1: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'
];

const CARDS_ORDER_2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'
];

pub struct Bid
{
    pub amount: u32,
    pub hand: Hand,
}

pub struct Hand
{
    pub cards: [Card; 5]
}

#[derive(Debug)]
pub struct Card(pub char);

impl Hand
{
    pub fn as_string(&self) -> String
    {
        self.cards.iter().map(|c| c.0).collect::<String>()
    }

    pub fn value(&self) -> u32
    {
        // Group cards by value
        let mut grouped_cards = self.cards
            .iter()
            .fold(HashMap::new(), |mut acc, card| {
                *acc.entry(card.0).or_insert(0) += 1;

                acc
            });

        let n_joker = grouped_cards.remove(&'J').unwrap_or(0);

        // If we only have joker, we have a five of a kind
        if n_joker == 5 { return 6 }

        // We don't really care about with card form the group
        let max_group = grouped_cards.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
        let second_max = grouped_cards.iter().filter(|(k, _)| **k != *max_group.0).max_by(|a, b| a.1.cmp(b.1));

        let max_group_with_joker = max_group.1 + n_joker;

        return match max_group_with_joker
        {
            4..=5 => max_group_with_joker + 1,
            3 => {
                if second_max.is_some() && *second_max.unwrap().1 == 2 { 4 }
                else { max_group_with_joker }
            },
            2 => {
                if second_max.is_some() && *second_max.unwrap().1 == 2 { 2 }
                else { 1 }
            }
            _ => 0
        }
    }
}

impl std::fmt::Debug for Hand
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        f.debug_struct("Hand")
            .field("cards", &self.as_string())
            .finish()
    }
}

impl From<&str> for Bid
{
    fn from(value: &str) -> Self
    {
        let mut parts = value.split(' ');

        let hand = Hand::from(parts.next().unwrap());
        let bid = parts.next().unwrap().parse::<u32>().unwrap();

        Bid {
            amount: bid,
            hand: hand
        }
    }
}

impl From<&str> for Hand
{
    fn from(value: &str) -> Self
    {
        if value.len() != 5
        {
            panic!("A hand must be 5 cards");
        }

        let cards = value.chars()
            .map(|c| Card(c))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Hand { cards: cards }
    }
}

impl PartialEq for Hand
{
    fn eq(&self, other: &Self) -> bool
    {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for Hand
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        let self_value  = self.value();
        let other_value = other.value();

        return if self_value == other_value
        {
            let first_match = self.cards.iter().zip(&other.cards)
                .find(|(a, b)| a != b);

            if let Some((a, b)) = first_match
            {
                a.cmp(b)
            }
            else
            {
                Ordering::Equal
            }
        }
        else
        {
            self_value.cmp(&other_value)
        };
    }
}

impl PartialEq for Card
{
    fn eq(&self, other: &Self) -> bool
    {
        self.0 == other.0
    }
}

impl Eq for Card {}

impl PartialOrd for Card
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for Card
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        if self == other { return Ordering::Equal }


        // Switch to 1 for part 1
        let self_index  = CARDS_ORDER_2.into_iter().position(|c| c == self.0).unwrap();
        let other_index = CARDS_ORDER_2.into_iter().position(|c| c == other.0).unwrap();

        return if self_index < other_index { Ordering::Greater }
               else { Ordering::Less };
    }
}

#[cfg(test)]
mod tests
{
    mod CardTest
    {
        use crate::camel_poker::Card;

        #[test]
        fn compare()
        {
            let a = Card('2');
            let b = Card('7');
            let c = Card('A');

            assert!(a < b);
            assert!(b < c);
            assert!(a < c);
            assert!(a <= a);
            assert!(a == a);
            assert!(a != b);
        }
    }

    mod HandTest
    {
        use crate::camel_poker::{Hand, Card};

        #[test]
        fn from_string()
        {
            let cards = "4A6K8";

            let hand = Hand::from(cards);

            assert_eq!(hand, Hand { cards: [
                Card('4'),
                Card('A'),
                Card('6'),
                Card('K'),
                Card('8')
            ]});
        }

        #[test]
        fn compare()
        {
            let pair = Hand::from("AA345");
            let two_pair = Hand::from("22669");
            let tok = Hand::from("44649");
            let flush = Hand::from("76766");
            let fok = Hand::from("66667");
            let f5ok = Hand::from("QQQQQ");

            assert!(pair < two_pair);
            assert!(two_pair < tok);
            assert!(tok < flush);
            assert!(flush < fok);
            assert!(fok < f5ok);
        }

        #[test]
        fn compare_same_value()
        {
            let pair = Hand::from("3AA45");
            let pair2 = Hand::from("4JJ65");

            assert!(pair < pair2);
        }
    }
}