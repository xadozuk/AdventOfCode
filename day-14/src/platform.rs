use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Tile
{
    Empty,
    RoundRock,
    CubeRock,
}

pub enum Direction
{
    North,
    West,
    South,
    East
}

#[derive(PartialEq, Eq, Debug)]
pub struct Platform
{
    matrix: Vec<Vec<Tile>>
}

impl Platform
{
    pub fn tilt(&mut self, direction: Direction)
    {
        match direction
        {
            Direction::North => {
                for c in 0..self.matrix[0].len()
                {
                    let compressed = self.tilt_left(&self.matrix.iter().map(|row| row[c]).collect());

                    compressed.iter().enumerate()
                        .for_each(|(r, t)| self.matrix[r][c] = *t);
                }
            },
            Direction::South => {
                for c in 0..self.matrix[0].len()
                {
                    let compressed: Vec<_> = self.tilt_left(&self.matrix.iter().map(|row| row[c]).rev().collect());

                    compressed.iter().rev().enumerate()
                        .for_each(|(r, t)| self.matrix[r][c] = *t);
                }
            },
            Direction::East => {
                for r in 0..self.matrix.len()
                {
                    let compressed: Vec<_> = self.tilt_left(&self.matrix[r].iter().rev().map(|t| *t).collect());

                    compressed.iter().rev().enumerate()
                        .for_each(|(c, t)| self.matrix[r][c] = *t);
                }
            },
            Direction::West => {
                for r in 0..self.matrix.len()
                {
                    let compressed: Vec<_> = self.tilt_left(&self.matrix[r]);

                    compressed.iter().enumerate()
                        .for_each(|(c, t)| self.matrix[r][c] = *t);
                }
            }
        }
    }

    pub fn run_cycle(&mut self, cycles: usize, cache: &mut HashMap<Vec<Vec<Tile>>, usize>)
    {
        // Find first cycle length
        let mut n_cycle = 0;
        let mut ref_cycle = 0;

        while n_cycle < cycles
        {
            let current = self.matrix.clone();

            if let Some(cycle) = cache.get(&current)
            {
                ref_cycle = *cycle;
                break;
            }

            cache.insert(current, n_cycle);

            self.tilt_cycle();
            n_cycle += 1;
        }

        let cycles_already_done = n_cycle;
        let remaining_cycles = cycles - cycles_already_done;
        let cycle_repeating_interval = cycles_already_done - ref_cycle;

        let skippable_cycles = (remaining_cycles as f64 / cycle_repeating_interval as f64).floor() as usize * cycle_repeating_interval;
        let remaining_cycles = remaining_cycles - skippable_cycles;

        // println!("[DEBUG] Found repetition between [{}, {}]", ref_cycle, n_cycle - ref_cycle);
        // println!("[DEBUG] Cycles done: {}, Interval: {}, Can be skipped: {}, Remaining: {}", n_cycle, cycle_repeating_interval, skippable_cycles, remaining_cycles);

        for _ in 0..remaining_cycles
        {
            self.tilt_cycle();
        }

    }

    pub fn tilt_cycle(&mut self)
    {
        for direction in [Direction::North, Direction::West, Direction::South, Direction::East]
        {
            self.tilt(direction)
        }
    }

    pub fn load(&self) -> u64
    {
        let height = self.matrix.len();

        self.matrix.iter().enumerate()
            .map(|(i, row)| {
                let n_round_rocks = row.iter().filter(|t| **t == Tile::RoundRock).count() as u64;

                (height - i) as u64 * n_round_rocks
            })
            .sum()
    }

    fn tilt_left(&self, vec: &Vec<Tile>) -> Vec<Tile>
    {
        let parts = vec.split(|t| *t == Tile::CubeRock);
        let mut results = vec![];

        for part in parts
        {
            let part_len = part.len();
            let n_round_rocks = part.iter().filter(|t| **t == Tile::RoundRock).count();

            let tilted_parts: Vec<_> = (0..part_len)
                .map(|i| {
                    if i < n_round_rocks { Tile::RoundRock }
                    else { Tile::Empty }
                })
                .collect();

            results.push(tilted_parts);
        }

        results.join(&Tile::CubeRock)
    }
}

impl From<&str> for Platform
{
    fn from(value: &str) -> Self
    {
        let matrix = value.split('\n')
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        'O' => Tile::RoundRock,
                        '#' => Tile::CubeRock,
                        _ => Tile::Empty
                    })
                    .collect()
            })
            .collect();

        Platform { matrix }
    }
}

#[cfg(test)]
mod tests
{
    use crate::platform::{Platform, Tile};

    #[test]
    fn test_eq()
    {
        let p1 = Platform::from("O..#");
        let p2 = Platform::from("O..#");
        let p3 = Platform::from("OO.#");

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_tile_north()
    {
        let mut p = Platform::from(
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

        let expected = Platform::from(
"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
        );

        p.tilt(super::Direction::North);
        assert_eq!(p, expected);
    }

    #[test]
    fn test_load()
    {
        let p = Platform::from(
"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
        );

        assert_eq!(p.load(), 136);
    }

    #[test]
    fn test_tile_left()
    {
        let p = Platform::from("#.#...O.#.");

        let tilted = p.tilt_left(&p.matrix[0]);

        assert_eq!(tilted, vec![
            Tile::CubeRock,
            Tile::Empty,
            Tile::CubeRock,
            Tile::RoundRock,
            Tile::Empty,
            Tile::Empty,
            Tile::Empty,
            Tile::Empty,
            Tile::CubeRock,
            Tile::Empty
        ]);
    }
}