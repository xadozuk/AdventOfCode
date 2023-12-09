use std::{fs::File, io::{BufReader, BufRead}};

mod oasis;

use oasis::History;

fn main()
{
    let file = File::open("./day-09/input.txt").unwrap();
    let buffer: BufReader<File>  = BufReader::new(file);

    let histories = parse(buffer);

    let result: i64 = histories
        .iter()
        .map(|h| h.next_value())
        .sum();

    let result2: i64 = histories
        .iter()
        .map(|h| h.previous_value())
        .sum();

    println!("Result: {:?}", result);
    println!("Result 2: {:?}", result2);
}

fn parse(buffer: BufReader<File>) -> Vec<History>
{
    let mut result = vec![];

    for line in buffer.lines()
    {
        match line
        {
            Ok(content) => result.push(History::from(content)),
            Err(e) => panic!("Error while reading file: {}", e)
        }
    }

    result
}
