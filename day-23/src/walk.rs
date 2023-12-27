use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::graph::Graph;

type Coord = (usize, usize);

#[derive(PartialEq, Eq, EnumIter, Copy, Clone, Debug)]
pub enum Direction
{
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Tile
{
    Ground,
    Forest,
    Slope(Direction)
}

pub struct Walk
{
    matrix: Vec<Vec<Tile>>,
    graph: Graph,
    slippery: bool,
    pub start: Coord,
    end: Coord
}

impl From<&str> for Walk
{
    fn from(value: &str) -> Self
    {
        let matrix: Vec<_> = value.split("\n")
            .map(|row| {
                row.chars().map(|c| {
                    match c
                    {
                        '.' => Tile::Ground,
                        '#' => Tile::Forest,
                        '>' => Tile::Slope(Direction::Right),
                        '<' => Tile::Slope(Direction::Left),
                        '^' => Tile::Slope(Direction::Up),
                        'v' => Tile::Slope(Direction::Down),
                        default => panic!("Unknown tile '{}'", default)
                    }
                })
                .collect::<Vec<_>>()
            })
            .collect();

        let start = (
            0,
            matrix[0].iter().enumerate()
                .find(|(_, t)| **t == Tile::Ground)
                .unwrap().0
        );

        let end = (
            matrix.len() - 1,
            matrix[matrix.len() - 1].iter().enumerate()
                .find(|(_, t)| **t == Tile::Ground)
                .unwrap().0
            );

        Walk {
            matrix,
            graph: Graph::new(),
            slippery: true,
            start,
            end
        }
    }
}

impl Direction
{
    pub fn opposite(&self) -> Direction
    {
        match self
        {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

impl Walk
{
    pub fn set_slippy(&mut self, slippery: bool)
    {
        self.slippery = slippery;
    }

    pub fn max_hike(&self) -> usize
    {
        *self.graph.distances(self.start, self.end).iter().max().unwrap()
    }

    pub fn compute_graph(&mut self)
    {
        self.graph.reset();

        self.graph.new_node(self.start);
        self.graph.new_node(self.end);

        self.recurse_compute_graph(self.start, self.end, Direction::Down);
    }

    fn recurse_compute_graph(&mut self, start: Coord, end: Coord, direction: Direction)
    {
        debug!("[WALK] {:?} -> {:?}", start, direction);

        let next_crossroad = self.walk_to_next_crossroad(
            start,
            end,
            direction
        );

        // Dead End
        if next_crossroad.is_none() { return; }

        let (crossroad_coord, direction, distance) = next_crossroad.unwrap();

        // We are at a cross road, so we need to:
        // - Check if node exists and create if if not
        //  -> If node already exists we already created it, so we connect nodes and return
        // - Else, recurse walk in all possible directions

        let create_node = !self.graph.contains(crossroad_coord);

        if create_node
        {
            self.graph.new_node(crossroad_coord);
        }

        // +1 to count the next tiles (we start at 0). only case were it doesn't work is the starting point
        // (But we can -1 the total)
        self.graph.connect_nodes(start, crossroad_coord, distance);

        // In part 2, we connect everything to form loops
        if !self.slippery
        {
            self.graph.connect_nodes(crossroad_coord, start, distance);
        }

        if !create_node { return; }

        for (new_direction, _) in self.next_tiles(crossroad_coord, direction)
        {
            debug!("[WALK] Unexplored crossroad at {:?} -> {:?}", crossroad_coord, new_direction);
            self.recurse_compute_graph(crossroad_coord, end, new_direction);
        }
    }

    fn walk_to_next_crossroad(&self, start: Coord, end: Coord, direction: Direction) -> Option<(Coord, Direction, usize)>
    {
        // Step once to get out of the current crossroad (if we are in one)
        let next_step = self.move_to(start, direction);

        if next_step.is_none()
        {
            panic!("Unable to go {:?} ({:?})", direction, start);
        }

        let next_step = next_step.unwrap();

        let mut distance_walked = 1;
        let mut coord = next_step.1;
        let mut direction = direction;

        // We need to go straight (and turn for corner) until we hit a crossroad
        loop
        {
            // If we hit end, return
            if coord == end
            {
                return Some((coord, direction, distance_walked));
            }

            // For each case we need to check if we are not at a cross road
            let next_tiles = self.next_tiles(coord, direction);

            // Impasse
            match next_tiles.len()
            {
                // Impasse
                0 => return None,
                // Straight / Single corner
                1 => {
                    direction = next_tiles[0].0;
                    coord = next_tiles[0].1;

                    distance_walked += 1;
                },
                _ =>
                {
                    return Some((coord, direction, distance_walked));
                }
            }
        }
    }

    fn move_to(&self, coord: Coord, direction: Direction) -> Option<(Tile, Coord)>
    {
        if coord.0 == 0 && direction == Direction::Up ||
           coord.0 == self.matrix.len() - 1 && direction == Direction::Down ||
           coord.1 == 0 && direction == Direction::Left ||
           coord.1 == self.matrix[0].len() - 1 && direction == Direction::Right
        {
            return None;
        }

        let new_coord = match direction
        {
            Direction::Up => (coord.0 - 1, coord.1),
            Direction::Down => (coord.0 + 1, coord.1),
            Direction::Left => (coord.0, coord.1 - 1),
            Direction::Right => (coord.0, coord.1 + 1)
        };

        Some((self.tile_at(new_coord), new_coord))
    }

    fn tile_at(&self, coord: Coord) -> Tile
    {
        self.matrix[coord.0][coord.1]
    }

    fn next_tiles(&self, coord: Coord, coming_from: Direction) -> Vec<(Direction, Coord)>
    {
        let mut results = vec![];

        for direction in Direction::iter().filter(|d| *d != coming_from.opposite())
        {
            let r = match self.move_to(coord, direction)
            {
                Some((Tile::Ground, new_coord)) => Some((direction, new_coord)),
                Some((Tile::Slope(slope_direction), new_coord)) => {
                    if !self.slippery || slope_direction == direction
                    {
                        Some((direction, new_coord))
                    }
                    else
                    {
                        None
                    }
                },
                _ => None
            };

            if r.is_some()
            {
                results.push(r.unwrap());
            }
        }

        results
    }

    pub fn graph(&self) -> &Graph
    {
        &self.graph
    }
}

#[cfg(test)]
mod tests
{
    use super::{Walk, Direction};

    #[test]
    fn test_walk_to_next_crossroads_1()
    {
        let w = Walk::from(
"#.#
...
#.#
");

        let r = w.walk_to_next_crossroad(
            (0, 1),
            (2, 1),
            Direction::Down,
        );

        assert!(r.is_some());

        let (coord, dir, dist) = r.unwrap();

        assert_eq!((1, 1), coord);
        assert_eq!(Direction::Down, dir);
        assert_eq!(1, dist);
    }

    #[test]
    fn test_walk_to_next_crossroads_2()
    {
        let w = Walk::from(
"#.#
#.#
#.#
");

        let r = w.walk_to_next_crossroad(
            (0, 1),
            (2, 1),
            Direction::Down
        );

        assert!(r.is_some());

        let (coord, dir, dist) = r.unwrap();

        assert_eq!((2, 1), coord);
        assert_eq!(Direction::Down, dir);
        assert_eq!(2, dist);
    }
}