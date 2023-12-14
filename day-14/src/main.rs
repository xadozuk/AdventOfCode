use std::collections::HashMap;

use platform::{Platform, Direction};

mod platform;

fn main()
{
    let mut platform = Platform::from(std::fs::read_to_string("./day-14/input.txt").unwrap().as_str());

    platform.tilt(Direction::North);
    println!("Result: {}", platform.load());

    let mut platform = Platform::from(std::fs::read_to_string("./day-14/input.txt").unwrap().as_str());
    platform.run_cycle(1_000_000_000, &mut HashMap::new());

    println!("Result 2: {}", platform.load());
}

#[cfg(test)]
mod tests
{
    use std::collections::HashMap;

    use crate::platform::{Platform, Direction};

    #[test]
    pub fn test_example1()
    {
        let mut platform = Platform::from(
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
        );

        platform.tilt(Direction::North);
        assert_eq!(platform.load(), 136);
    }

    #[test]
    pub fn test_example2()
    {
        let mut platform = Platform::from(
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
        );

        platform.run_cycle(1_000_000_000, &mut HashMap::new());
        assert_eq!(platform.load(), 64);
    }
}