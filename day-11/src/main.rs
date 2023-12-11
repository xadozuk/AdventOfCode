use std::{io::BufReader, fs::File};

mod observatory;

use observatory::Space;

fn main()
{
    let file = File::open("./day-11/input.txt").unwrap();
    let buffer: BufReader<File>  = BufReader::new(file);

    let space = Space::from(buffer);

    let galaxy_pairs = space.find_galaxy_pairs(2);
    let galaxy_pairs2 = space.find_galaxy_pairs(1_000_000);

    let result: u64 = galaxy_pairs.iter().map(|pair| pair.1).sum();
    let result2: u64 = galaxy_pairs2.iter().map(|pair| pair.1).sum();

    println!("Result: {}", result);
    println!("Result: {}", result2);
}
