mod game;

use std::fs::File;
use std::io::{prelude::*, BufReader};

use game::Game;

use crate::game::CubeSet;

fn parse_games(buffer: BufReader<File>) -> Vec<Game>
{
    let mut games = vec![];

    for line in buffer.lines()
    {
        match line
        {
            Ok(content) =>
            {
                games.push(Game::from(content.as_str()));
            },
            Err(e) => println!("Error while reading line: {0}", e)
        }
    }

    return games;
}

fn main()
{
    let file = File::open("./day-02/input.txt").unwrap();
    let buffer = BufReader::new(file);
    let games = parse_games(buffer);

    let ref_set = CubeSet { red: 12, green: 13, blue: 14 };

    let sum: u32 = games.iter()
        .filter(|g| g.can_have_set(&ref_set))
        .map(|g| g.id)
        .sum();

    // Part 2
    let sum2: u32 = games.iter()
        .map(|g| g.min_cube_set().power())
        .sum();

    println!("Result (part 1): {0}", sum);
    println!("Result (part 2): {0}", sum2);
}