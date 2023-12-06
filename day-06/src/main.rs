use std::{fs::File, io::{BufReader, BufRead}};

// (time, distance)
pub struct Race(u64, u64);

fn main()
{
    let file = File::open("./day-06/input.txt").unwrap();
    let buffer = BufReader::new(file);

    let (races, one_race) = parse_races(buffer);

    let result: u64 = races
        .iter()
        .map(|r| possible_solutions(r))
        .product();

    let one_race_solutions = possible_solutions(&one_race);

    println!("Result: {0}", result);
    println!("Result 2: {0}", one_race_solutions);
}

fn parse_races(buffer: BufReader<File>) -> (Vec<Race>, Race)
{
    let mut lines = buffer.lines();

    let times_string = lines.next().unwrap().unwrap().replace("Time:", "");

    let one_time = times_string.replace(" ", "").parse::<u64>().unwrap();
    let times = times_string
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u64>().unwrap());


    let distances_string = lines.next().unwrap().unwrap().replace("Distance:", "");

    let one_distance = distances_string.replace(" ", "").parse::<u64>().unwrap();
    let distances = distances_string
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u64>().unwrap());

    let races = times.zip(distances)
        .map(|(t, d)| Race(t, d))
        .collect();

    return (races, Race(one_time, one_distance))
}

fn possible_solutions(race: &Race) -> u64
{
    let delta = (race.0.pow(2) - 4 * race.1) as f64;
    let x1 = ((-(race.0 as f64) - delta.sqrt()) / (-2f64)).ceil() as u64;
    let x2 = ((-(race.0 as f64) + delta.sqrt()) / (-2f64)).floor() as u64;

    return x1 - x2 - 1;
}