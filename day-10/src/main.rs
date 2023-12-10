use std::{fs::File, io::{BufReader, BufRead}};

use colored::Colorize;

type Coord = (usize, usize);

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Tile
{
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start
}

fn main()
{
    let file = File::open("./day-10/input.txt").unwrap();
    let buffer: BufReader<File>  = BufReader::new(file);

    let matrix = parse(buffer);
    let l = find_loop(&matrix);

    let enclosed_tiles = find_enclosed_tiles(&matrix, &l);

    // DEBUG
    for i in 0..matrix.len()
    {
        for j in 0..matrix[i].len()
        {
            let coord = (i, j);

            let char = match matrix[coord.0][coord.1]
            {
                Tile::Start => 'S',
                Tile::Horizontal => '━',
                Tile::Vertical => '┃',
                Tile::NorthEast => '┗',
                Tile::NorthWest => '┛',
                Tile::SouthEast => '┏',
                Tile::SouthWest => '┓',
                Tile::Ground => '.'
            };

            if char == 'S'
            {
                print!("{}", "S".green().on_red());
            }
            else if l.contains(&coord)
            {
                print!("{}", char.to_string().green());
            }
            else if enclosed_tiles.contains(&coord)
            {
                print!("{}", char.to_string().on_yellow());
            }
            else
            {
                print!("{}", char);
            }
        }

        println!();
    }

    println!("Result: {}", (l.len() - 1) / 2);
    println!("Result 2: {}", enclosed_tiles.len());
}

fn find_enclosed_tiles(matrix: &Vec<Vec<Tile>>, loop_coords: &Vec<Coord>) -> Vec<Coord>
{
    let all_non_loop_tiles: Vec<Coord> = matrix.iter().enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate()
                .map(|(j, _)| (i, j))
                .filter(|coord| !loop_coords.contains(coord))
                .collect::<Vec<Coord>>()
        })
        .collect();

    let mut enclosed_tiles = vec![];

    for coord in all_non_loop_tiles
    {
        // Ray-cast to check if enclosed or not
        // Odd: inside, Even: outside
        let hit = raycast_hits(coord, loop_coords, matrix);

        if hit % 2 == 1
        {
            enclosed_tiles.push(coord);
        }
    }

    return enclosed_tiles;
}

fn raycast_hits(coord: Coord, polygon_coords: &Vec<Coord>, matrix: &Vec<Vec<Tile>>) -> u32
{
    let row = coord.0;
    let mut hit_count = 0;
    let mut j = 0;

    while j < coord.1
    {
        let current_coord = (row, j);
        let hit = polygon_coords.iter()
            .find(|polygon_coord| **polygon_coord == current_coord);

        // If we hit something
        if let Some(hit_coord) = hit
        {
            let start_tile = matrix[hit_coord.0][hit_coord.1];

            if start_tile != Tile::Vertical
            {
                // We search the first tile that is not horizontal
                let shift = matrix[hit_coord.0].iter()
                    .skip(hit_coord.1 + 1)
                    .position(|tile| *tile != Tile::Horizontal)
                    .unwrap();

                let end_tile = matrix[hit_coord.0][j + shift + 1];

                // We need to know if its a U form or Z form
                // We only hit on Z form
                let should_hit = match start_tile
                {
                    Tile::NorthEast => end_tile == Tile::SouthWest,
                    Tile::SouthEast => end_tile == Tile::NorthWest,
                    _ => false // Impossible case
                };

                if should_hit { hit_count += 1 }

                // +1 for the skipped tile
                j += shift + 1;
            }
            else
            {
                // If it's a vertical tile, we count a hit
                hit_count += 1;
            }
        }

        j += 1;
    }

    return hit_count;
}

fn find_loop(matrix: &Vec<Vec<Tile>>) -> Vec<Coord>
{
    let (start_coord, _) = find_start(&matrix).unwrap();

    // Check loop in 4 directions
    let mut next_coords = vec![];

    // Up
    if start_coord.0 > 0
    {
        match matrix[start_coord.0 - 1][start_coord.1]
        {
            Tile::Vertical | Tile::SouthEast | Tile::SouthWest => next_coords.push((start_coord.0 - 1, start_coord.1)),
            _ => ()
        }
    }

    // Down
    if start_coord.0 < matrix.len() - 1
    {
        match matrix[start_coord.0 + 1][start_coord.1]
        {
            Tile::Vertical | Tile::NorthEast | Tile::NorthWest => next_coords.push((start_coord.0 + 1, start_coord.1)),
            _ => ()
        }
    }

    // Left
    if start_coord.1 > 0
    {
        match matrix[start_coord.0][start_coord.1 - 1]
        {
            Tile::Horizontal | Tile::NorthWest | Tile::SouthWest => next_coords.push((start_coord.0, start_coord.1 - 1)),
            _ => ()
        }
    }

    // Right
    if start_coord.1 < matrix.len() - 1
    {
        match matrix[start_coord.0][start_coord.1 + 1]
        {
            Tile::Horizontal | Tile::SouthEast | Tile::NorthEast => next_coords.push((start_coord.0, start_coord.1 + 1)),
            _ => ()
        }
    }

    for next_coord in next_coords
    {
        let mut possible_loop = follow_possible_loop(next_coord, &matrix);

        // We found the loop
        if possible_loop.len() > 0 && *possible_loop.last().unwrap() == start_coord
        {
            possible_loop.insert(0, start_coord);
            return possible_loop;
        }
    }

    vec![]
}

fn find_start(matrix: &Vec<Vec<Tile>>) -> Option<(Coord, Tile)>
{
    matrix.iter().enumerate()
        .map(|(i, row)| {
            row.iter().enumerate()
                .map(|(j , c)| ((i, j), *c))
                .find(|(_, c)| *c == Tile::Start)
        })
        .find(|x| x.is_some())
        .unwrap_or(None)
}

fn follow_possible_loop(start: Coord, matrix: &Vec<Vec<Tile>>) -> Vec<Coord>
{
    let mut current_loop = vec![];
    let mut current_coord = start;

    loop
    {
        // If we found starting point, we closed the loop
        if matrix[current_coord.0][current_coord.1] == Tile::Start
        {
            // Add starting point to close the loop
            current_loop.push(current_coord);
            break
        }

        let neighboors = neighboors(current_coord, &matrix);

        // No neightboors or at an impasse, we skip
        if let Some(n) = neighboors
        {
            if n.len() < 2 { break }

            let last_coord = current_loop.last();

            // We don't go backward
            let next_coord =
                if let Some(last_coord) = last_coord
                {
                    n.iter().find(|(coord, _)| last_coord != coord)
                }
                else
                {
                    // Special case, on the first step we don't want the starting pos
                    n.iter().find(|(_, tile)| *tile != Tile::Start)
                };

            // We don't have anywhere else to go
            if next_coord.is_none() { break }

            current_loop.push(current_coord);
            current_coord = next_coord.unwrap().0;
        }
        else
        {
            break
        }
    }

    current_loop
}

fn neighboors(coord: Coord, matrix: &Vec<Vec<Tile>>) -> Option<Vec<(Coord, Tile)>>
{
    match matrix[coord.0][coord.1]
    {
        Tile::Horizontal => {
            // If we are at the border
            if coord.1 == 0 || coord.1 == matrix[0].len() - 1 { return None }

            let left = &matrix[coord.0][coord.1 - 1];
            let right = &matrix[coord.0][coord.1 + 1];

            match left
            {
                Tile::Start | Tile::Horizontal | Tile::NorthEast | Tile::SouthEast => (),
                _ => return None
            }

            match right
            {
                Tile::Start | Tile::Horizontal | Tile::NorthWest | Tile::SouthWest => (),
                _ => return None
            }

            return Some(vec![
                ((coord.0, coord.1 - 1), *left),
                ((coord.0, coord.1 + 1), *right)
            ]);
        },
        Tile::Vertical => {
            // If we are at the border
            if coord.0 == 0 || coord.0 == matrix.len() - 1 { return None }

            let above = &matrix[coord.0 - 1][coord.1];
            let below = &matrix[coord.0 + 1][coord.1];

            match above
            {
                Tile::Start | Tile::Vertical | Tile::SouthEast | Tile::SouthWest => (),
                _ => return None
            }

            match below
            {
                Tile::Start | Tile::Vertical | Tile::NorthEast | Tile::NorthWest => (),
                _ => return None
            }

            return Some(vec![
                ((coord.0 - 1, coord.1), *above),
                ((coord.0 + 1, coord.1), *below)
            ]);
        },
        Tile::NorthEast => {
            // at top or right border
            if coord.0 == 0 || coord.1 == matrix[0].len() - 1 { return None }

            let above = &matrix[coord.0 - 1][coord.1];
            let right = &matrix[coord.0][coord.1 + 1];

            match above
            {
                Tile::Start | Tile::Vertical | Tile::SouthEast | Tile::SouthWest => (),
                _ => return None
            }

            match right
            {
                Tile::Start | Tile::Horizontal | Tile::NorthWest | Tile::SouthWest => (),
                _ => return None
            }

            return Some(vec![
                ((coord.0 - 1, coord.1), *above),
                ((coord.0, coord.1 + 1), *right)
            ]);
        },
        Tile::NorthWest => {
            // at top or left border
            if coord.0 == 0 || coord.1 == 0 { return None }

            let above = &matrix[coord.0 - 1][coord.1];
            let left = &matrix[coord.0][coord.1 - 1];

            match above
            {
                Tile::Start | Tile::Vertical | Tile::SouthEast | Tile::SouthWest => (),
                _ => return None
            }

            match left
            {
                Tile::Start | Tile::Horizontal | Tile::NorthEast | Tile::SouthEast => (),
                _ => return None
            }

            return Some(vec![
                ((coord.0 - 1, coord.1), *above),
                ((coord.0, coord.1 - 1), *left)
            ]);
        },
        Tile::SouthEast => {
            // at bottom or right border
            if coord.0 == matrix.len() - 1 || coord.1 == matrix[0].len() - 1 { return None }

            let below = &matrix[coord.0 + 1][coord.1];
            let right = &matrix[coord.0][coord.1 + 1];

            match below
            {
                Tile::Start | Tile::Vertical | Tile::NorthEast | Tile::NorthWest => (),
                _ => return None
            }

            match right
            {
                Tile::Start | Tile::Horizontal | Tile::NorthWest | Tile::SouthWest => (),
                _ => return None
            }

            return Some(vec![
                ((coord.0 + 1, coord.1), *below),
                ((coord.0, coord.1 + 1), *right)
            ]);
        },
        Tile::SouthWest => {
            // at bottom or left border
            if coord.0 == matrix.len() - 1 || coord.1 == 0 { return None }

            let below = &matrix[coord.0 + 1][coord.1];
            let left = &matrix[coord.0][coord.1 - 1];

            match below
            {
                Tile::Start | Tile::Vertical | Tile::NorthEast | Tile::NorthWest => (),
                _ => return None
            }

            match left
            {
                Tile::Start | Tile::Horizontal | Tile::NorthEast | Tile::SouthEast => (),
                _ => return None
            }

            return Some(vec![
                ((coord.0 + 1, coord.1), *below),
                ((coord.0, coord.1 - 1), *left)
            ]);
        },
        Tile::Ground => return None,
        _ => panic!("We don't care about starting point neighboors"),
    }
}

fn parse(buffer: BufReader<File>) -> Vec<Vec<Tile>>
{
    buffer.lines().map(|line| {
        line.unwrap().chars().map(|char| {
            match char
            {
                '-' => Tile::Horizontal,
                '|' => Tile::Vertical,
                'L' => Tile::NorthEast,
                'J' => Tile::NorthWest,
                '7' => Tile::SouthWest,
                'F' => Tile::SouthEast,
                'S' => Tile::Start,
                _   => Tile::Ground
            }
        })
        .collect()
    })
    .collect()
}

#[cfg(test)]
mod tests
{
    use crate::{neighboors, Tile};

    #[test]
    fn test_neighboors()
    {
        let matrix = vec![
            vec![Tile::Ground, Tile::Vertical],
            vec![Tile::Horizontal, Tile::NorthWest]
        ];

        let n1 = neighboors((0, 0), &matrix);
        assert!(n1.is_none());

        let n2 = neighboors((1, 1), &matrix);
        assert!(n2.is_some());

        let n2 = n2.unwrap();
        assert_eq!(n2.len(), 2);
        assert_eq!(n2[0], ((0, 1), Tile::Vertical));
        assert_eq!(n2[1], ((1, 0), Tile::Horizontal));
    }
}