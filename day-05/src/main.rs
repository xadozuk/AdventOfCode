use std::{io::BufReader, fs::File};

mod farm;

fn main()
{
    let file = File::open("./day-05/input.txt").unwrap();
    let buffer = BufReader::new(file);

    let manager = farm::Manager::from_input(buffer);

    println!("Result: {0}", manager.lowest_location());
    println!("Result 2: {0}", manager.lowest_ranges_location());
}
