use std::{fs::File, io::{BufReader, BufRead}};

mod camel_poker;

use camel_poker::Bid;

fn main()
{
    let file = File::open("./day-07/input.txt").unwrap();
    let buffer = BufReader::new(file);

    let mut bids = vec![];

    for line in buffer.lines()
    {
        match line
        {
            Ok(content) => bids.push(Bid::from(content.as_str())),
            Err(e) => println!("Error while reading file: {:?}", e)
        }
    }

    bids.sort_by(|a, b| a.hand.cmp(&b.hand));

    let result: u32 = bids
        .iter().enumerate()
        .map(|(i, b)| ((i + 1) as u32) * b.amount)
        .sum();

    for (i, bid) in bids.iter().enumerate()
    {
        println!("[{0}] {1} = {2}", i, &bid.hand.as_string(), bid.hand.value());
    }

    println!("Result: {0}", result);
    println!("Result 2: {0}", 0);
}
