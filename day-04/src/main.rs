use std::{fs::File, io::BufReader};
use std::io::prelude::*;

mod card;

use card::Card;

fn count_copy_cards(cards: &Vec<Card>) -> u32
{
    let mut copy_cards : Vec<u32> = vec![0; cards.len()];

    for (i, card) in cards.iter().enumerate()
    {
        let matching_numbers = card.matching_numbers();

        for j in 0..matching_numbers
        {
            copy_cards[i + (j + 1) as usize] += copy_cards[i] + 1;
        }
    }

    return copy_cards.iter().sum();
}

fn main()
{
    let file = File::open("./day-04/input.txt").unwrap();
    let buffer = BufReader::new(file);

    let mut cards = vec![];

    for line in buffer.lines()
    {
        match line
        {
            Ok(content) => cards.push(Card::from(content.as_str())),
            Err(e) => println!("Error while reading file: {:?}", e)
        }
    }

    let result: u32 = cards.iter()
        .map(|c| c.score())
        .sum();

    let result2: u32 = cards.len() as u32 + count_copy_cards(&cards);

    println!("Result: {0}", result);
    println!("Result 2: {0}", result2);
}
