use std::collections::HashSet;
use std::fmt::Debug;

pub const UP: Vector    = Vector::new(-1, 0);
pub const DOWN: Vector  = Vector::new(1, 0);
pub const LEFT: Vector  = Vector::new(0, -1);
pub const RIGHT: Vector = Vector::new(0, 1);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coord
{
    x: usize,
    y: usize
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vector
{
    x: i8,
    y: i8
}

#[derive(Debug)]
pub enum Tile
{
    Empty,
    MirrorSlash,
    MirrorAntiSlash,
    VerticalSplitter,
    HorizontalSplitter
}

pub struct Facility
{
    matrix: Vec<Vec<Tile>>,
    visited_tiles: HashSet<(Coord, Vector)>
}

impl Vector
{
    pub const fn new(x: i8, y: i8) -> Self
    {
        Vector { x, y }
    }
}

impl Debug for Vector
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            &UP => f.write_str("Up"),
            &DOWN => f.write_str("Down"),
            &LEFT => f.write_str("Left"),
            &RIGHT => f.write_str("Right"),
            _ => f.write_str("Unknown")
        }?;

        Ok(())
    }
}

impl Coord
{
    pub const fn new(x: usize, y: usize) -> Self
    {
        Coord { x, y }
    }
}

impl Facility
{
    pub fn start_beam(&mut self, coord: Coord, direction: Vector)
    {
        self.visited_tiles.clear();
        self.run_beam(&coord, &direction);
    }

    fn run_beam(&mut self, coord: &Coord, direction: &Vector)
    {
        let mut direction = direction;
        let mut coord = coord.clone();

        loop
        {
            let tile = &self.matrix[coord.x][coord.y];

            debug!("Current coord: {:?} ({:?})", coord, *direction);
            debug!("Current tile: {:?}", tile);

            if self.visited_tiles.contains(&(coord, *direction))
            {
                // If we are on a path already visited (in same direction)
                debug!("Already visited ({:?}, {:?})", coord, direction);
                return;
            }

            self.visited_tiles.insert((coord, *direction));

            match tile
            {
                Tile::HorizontalSplitter => {
                    if *direction == UP || *direction == DOWN
                    {
                        debug!("Found splitter, recursing...");

                        let left_coord = self.next_coord(&coord, &LEFT);
                        let right_coord = self.next_coord(&coord, &RIGHT);

                        if let Some(left_coord) = left_coord
                        {
                            self.run_beam(&left_coord, &LEFT);
                        }

                        if let Some(right_coord) = right_coord
                        {
                            self.run_beam(&right_coord, &RIGHT);
                        }

                        break;
                    }
                },
                Tile::VerticalSplitter => {
                    if *direction == LEFT || *direction == RIGHT
                    {
                        debug!("Found splitter, recursing...");

                        let up_coord = self.next_coord(&coord, &UP);
                        let down_coord = self.next_coord(&coord, &DOWN);

                        if let Some(up_coord) = up_coord
                        {
                            self.run_beam(&up_coord, &UP);
                        }

                        if let Some(down_coord) = down_coord
                        {
                            self.run_beam(&down_coord, &DOWN);
                        }

                        break;
                    }
                },
                Tile::MirrorSlash => {
                    direction = match direction
                    {
                        &UP => &RIGHT,
                        &DOWN => &LEFT,
                        &LEFT => &DOWN,
                        &RIGHT => &UP,
                        _ => panic!("Unknown direction")
                    };
                },
                Tile::MirrorAntiSlash => {
                    direction = match direction
                    {
                        &UP => &LEFT,
                        &DOWN => &RIGHT,
                        &LEFT => &UP,
                        &RIGHT => &DOWN,
                        _ => panic!("Unknown direction")
                    };
                },
                Tile::Empty => (),
            }

            let next_coord = self.next_coord(&coord, direction);

            if next_coord.is_none()
            {
                debug!("Next tile is out-of-bound, returning...");
                return
            }

            coord = next_coord.unwrap();
        }
    }

    pub fn energized_tiles_count(&self) -> usize
    {
        self.visited_tiles.iter()
            .map(|(c, _)| c)
            .collect::<HashSet<_>>()
            .len()
    }

    fn next_coord(&self, coord: &Coord, direction: &Vector) -> Option<Coord>
    {
        let x = (coord.x as i32) + direction.x as i32;
        let y = (coord.y as i32) + direction.y as i32;

        if x < 0 || y < 0 || x >= self.matrix.len() as i32 || y >= self.matrix[0].len() as i32
        {
            return None;
        }

        Some(Coord { x: x as usize, y: y as usize })
    }

    pub fn find_most_enegized_starting_point(&mut self) -> (Coord, Vector, usize)
    {
        let mut results: Vec<(Coord, Vector, usize)> = vec![];
        // Left / Right
        for i in 0..self.matrix.len()
        {
            let coord = Coord::new(i, 0);
            self.start_beam(coord, RIGHT);
            results.push((coord, RIGHT, self.energized_tiles_count()));

            let coord = Coord::new(i, self.matrix[0].len() - 1);
            self.start_beam(coord, LEFT);
            results.push((coord, LEFT, self.energized_tiles_count()));
        }

        // Top / Down
        for j in 0..self.matrix[0].len()
        {
            let coord = Coord::new(0, j);
            self.start_beam(coord, DOWN);
            results.push((coord, DOWN, self.energized_tiles_count()));

            let coord = Coord::new(self.matrix.len() - 1, j);
            self.start_beam(coord, UP);
            results.push((coord, UP, self.energized_tiles_count()));
        }

        *results.iter()
            .max_by(|a, b| a.2.cmp(&b.2))
            .unwrap()
    }
}

impl From<&str> for Facility
{
    fn from(value: &str) -> Self
    {
        let matrix = value.split('\n')
            .map(|row| {
                row.chars().map(|c| {
                    match c
                    {
                        '|'  => Tile::VerticalSplitter,
                        '-'  => Tile::HorizontalSplitter,
                        '/'  => Tile::MirrorSlash,
                        '\\' => Tile::MirrorAntiSlash,
                        _    => Tile::Empty
                    }
                })
                .collect()
            })
            .collect();

        Facility { matrix, visited_tiles: HashSet::new() }
    }
}