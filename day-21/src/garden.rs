use std::{collections::{HashMap, BinaryHeap}, cmp::Reverse};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Direction
{
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Eq)]
pub enum Tile
{
    Start,
    Plot,
    Rock
}

pub type Coord = (usize, usize);
pub type Layer = (i32, i32);

pub struct Garden
{
    matrix: Vec<Vec<Tile>>,
    starting_point: Coord
}

impl From<&str> for Garden
{
    fn from(value: &str) -> Self
    {
        let mut starting_point = (0, 0);

        let tiles = value.split("\n").enumerate()
            .map(|(r, line)| {
                line.chars().enumerate().map(|(c, ch)| {
                    match ch
                    {
                        'S' => { starting_point  = (r, c); Tile::Start }
                        '#' => Tile::Rock,
                        '.' => Tile::Plot,
                        _ => panic!("Unknown tile {} at {:?}", ch, (r, c))
                    }
                })
                .collect()
            })
            .collect();

        Garden { matrix: tiles, starting_point: starting_point }
    }
}

impl Garden
{
    pub fn walk_optimized(&self, max_distance: usize) -> i64
    {
        let width = self.matrix.len();
        let remainder = max_distance % width;

        let n0 = remainder;
        let n1 = remainder + width;
        let n2 = remainder + width * 2;

        verbose!("Solving for {} steps", n0);
        let s0 = self.walk(n0, false).len() as i64;

        verbose!("Solving for {} steps", n1);
        let s1 = self.walk(n1, true).len() as i64;

        verbose!("Solving for {} steps", n2);
        let s2 = self.walk(n2, true).len() as i64;

        verbose!("[{}, {}, {}]", s0, s1, s2);

        let a = (s2 - 2*s1 + s0) / 2;
        let b = (-3*s0 + 4*s1 - s2) / 2;
        let c = s0;

        // We included the remainder in the quadratic formula
        let n = (max_distance / width) as i64;

        verbose!("Equation: {}n^2 + ({})n + {} (with n = {})", a, b, c, n);

        a * n.pow(2) + b * n + c
    }

    pub fn walk(&self, max_distance: usize, infinite: bool) -> Vec<(Coord, Layer)>
    {
        // To check path with min_dist first
        let mut queue: BinaryHeap<Reverse<(usize, Coord, Layer)>> = BinaryHeap::new();
        let mut visited: HashMap<(Coord, Layer), usize> = HashMap::new();
        let mut last_distance = 0;

        queue.push(Reverse((0, self.starting_point, (0, 0))));

        while let Some(Reverse((distance, coord, layer))) = queue.pop()
        {
            debug!("[WALK] Visiting {:?} (current distance: {})", coord, distance);

            if distance % 1000 == 0 && last_distance != distance
            {
                last_distance = distance;
                verbose!("[WALK] Current distance {}/{}", distance, max_distance);
            }

            // Skip if we 2 tile send back to the same coord (to avoid duplicating checks)
            // if visited.contains_key(&coord) && *visited.get(&coord).unwrap() == distance { continue }
            if visited.contains_key(&(coord, layer)) { continue }

            // If we have covered the distance already
            if distance > max_distance { continue }

            visited.insert((coord, layer), distance);

            for direction in Direction::iter()
            {
                if let Some((new_coord, layer)) = self.move_to(coord, direction, layer, infinite)
                {
                    queue.push(Reverse((distance + 1, new_coord, layer)));
                }
            }
        }

        let max_distance_mod_of_2 = max_distance % 2;

        visited.iter()
            .filter(|(_, dist)| **dist % 2 == max_distance_mod_of_2)
            .map(|(k, _)| *k)
            .collect()
    }

    fn move_to(&self, coord: Coord, direction: Direction, layer: Layer, infinite: bool) -> Option<(Coord, Layer)>
    {
        if infinite { self.move_to_infinite(coord, direction, layer) }
        else { self.move_to_finite(coord, direction, layer) }
    }

    fn move_to_finite(&self, coord: Coord, direction: Direction, layer: Layer) -> Option<(Coord, Layer)>
    {
        let new_coord = match direction
        {
            Direction::Up =>
            {
                if coord.0 > 0 { Some((coord.0 - 1, coord.1)) }
                else { None }
            },
            Direction::Down => {
                if coord.0 < self.matrix.len() - 1 { Some((coord.0 + 1, coord.1)) }
                else { None }
            },
            Direction::Left => {
                if coord.1 > 0 { Some((coord.0, coord.1 - 1)) }
                else { None }
            },
            Direction::Right => {
                if coord.1 < self.matrix[0].len() - 1 { Some((coord.0, coord.1 + 1)) }
                else { None }
            }
        };

        if let Some(c) = new_coord
        {
            if self.matrix[c.0][c.1] == Tile::Rock { return None; }
        }

        new_coord.map(|c| (c, layer))
    }

    fn move_to_infinite(&self, coord: Coord, direction: Direction, layer: Layer) -> Option<(Coord, Layer)>
    {
        let result = match direction
        {
            Direction::Up =>
            {
                // Switch layer
                if coord.0 == 0
                {
                    (
                        (self.matrix.len() - 1, coord.1),
                        (layer.0 - 1, layer.1)
                    )
                }
                else
                {
                    (
                        (coord.0 - 1, coord.1),
                        layer
                    )
                }
            },
            Direction::Down =>
            {
                // Switch layer
                if coord.0 == self.matrix.len() - 1
                {
                    (
                        (0, coord.1),
                        (layer.0 + 1, layer.1)
                    )
                }
                else
                {
                    (
                        (coord.0 + 1, coord.1),
                        layer
                    )
                }
            },
            Direction::Left =>
            {
                // Switch layer
                if coord.1 == 0
                {
                    (
                        (coord.0, self.matrix[0].len() - 1),
                        (layer.0, layer.1 - 1)
                    )
                }
                else
                {
                    (
                        (coord.0, coord.1 - 1),
                        layer
                    )
                }
            },
            Direction::Right =>
            {
                // Switch layer
                if coord.1 == self.matrix[0].len() -1
                {
                    (
                        (coord.0, 0),
                        (layer.0, layer.1 + 1)
                    )
                }
                else
                {
                    (
                        (coord.0, coord.1 + 1),
                        layer
                    )
                }
            }
        };

        let (c, _) = result;
        if self.matrix[c.0][c.1] == Tile::Rock { return None; }

        Some(result)
    }
}