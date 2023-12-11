use std::{io::{BufReader, BufRead}, fs::File, collections::HashSet};

type Coord = (usize, usize);

#[derive(PartialEq, Eq, Debug)]
pub enum Tile
{
    Galaxy,
}

pub struct Space
{
    matrix: Vec<Vec<Option<Tile>>>,
    galaxies_index: Vec<Coord>,
    void_rows: HashSet<usize>,
    void_columns: HashSet<usize>
}

impl Space
{
    pub fn new(matrix: Vec<Vec<Option<Tile>>>) -> Self
    {
        let mut space = Space {
            matrix: matrix,
            galaxies_index: vec![],
            void_rows: HashSet::new(),
            void_columns: HashSet::new()
        };

        space.build_indexes();

        space
    }

    fn build_indexes(&mut self)
    {
        self.galaxies_index = self.matrix.iter().enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate()
                    .filter(|(_, col)| **col == Some(Tile::Galaxy))
                    .map(move |(j, _)| (i, j))
            })
            .collect();

        self.void_rows = self.matrix.iter().enumerate()
            .filter(|(_, row)| row.iter().all(|r| r.is_none()))
            .map(|(i, _)| i)
            .collect();

        self.void_columns = (0..self.matrix[0].len())
            .filter(|j| self.matrix.iter().all(|row| row[*j].is_none()))
            .collect();
    }

    pub fn find_galaxy_pairs(&self, expansion_factor: u64) -> Vec<((Coord, Coord), u64)>
    {
        let mut galaxy_pairs = vec![];
        let mut already_processed: HashSet<(Coord, Coord)> = HashSet::new();

        for a in &self.galaxies_index
        {
            for b in self.galaxies_index.iter().skip(1).filter(|b| *b != a)
            {
                if !already_processed.contains(&(*a, *b)) && !already_processed.contains(&(*b, *a))
                {
                    let distance = distance(&self, (*a, *b), expansion_factor);

                    galaxy_pairs.push(
                        ((*a, *b), distance)
                    );

                    // Insert permutations and revers to get only unique perms
                    already_processed.insert((*a, *b));
                    already_processed.insert((*b, *a));
                }
            }
        }

        galaxy_pairs
    }
}

impl From<BufReader<File>> for Space
{
    fn from(buffer: BufReader<File>) -> Self
    {
        let mut matrix = vec![];

        for line in buffer.lines()
        {
            match line
            {
                Ok(content) => {
                    matrix.push(
                        content.chars()
                            .map(|c| {
                                match c
                                {
                                    '#' => Some(Tile::Galaxy),
                                    _ => None,
                                }
                            })
                            .collect()
                    );
                },
                Err(e) => panic!("Error while reading file: {}", e)
            }
        }

        Space::new(matrix)
    }
}

pub fn distance(space: &Space, (a, b): (Coord, Coord), expansion_factor: u64) -> u64
{
    let x_start = a.0.min(b.0);
    let x_end   = a.0.max(b.0);
    let y_start = a.1.min(b.1);
    let y_end   = a.1.max(b.1);

    let dx: u64 = (x_start..x_end).map(|i| {
        if space.void_rows.contains(&i) { expansion_factor }
        else { 1 }
    })
    .sum();

    let dy: u64 = (y_start..y_end).map(|j| {
        if space.void_columns.contains(&j) { expansion_factor }
        else { 1 }
    })
    .sum();

    return dx + dy;
}

#[cfg(test)]
mod tests
{
    use super::{Tile, Space};

    #[test]
    fn test_expand_galaxy()
    {
        let mut space = Space::new(vec![
            vec![Some(Tile::Galaxy), None, Some(Tile::Galaxy)],
            vec![None, None, None],
            vec![Some(Tile::Galaxy), None, Some(Tile::Galaxy)]
        ]);

        space.expand(2);

        assert_eq!(space.matrix.len(), 4);
        assert_eq!(space.matrix[0].len(), 4);

        assert!(space.matrix[1].iter().all(|c| c.is_none()));
        assert!(space.matrix[2].iter().all(|c| c.is_none()));
        assert!(space.matrix.iter().all(|c| c[1].is_none()));
        assert!(space.matrix.iter().all(|c| c[2].is_none()));

        let mut space = Space::new(vec![
            vec![None, Some(Tile::Galaxy), None, Some(Tile::Galaxy), None],
        ]);

        space.expand(2);

        assert_eq!(space.matrix.len(), 1);
        assert_eq!(space.matrix[0].len(), 8);

        assert_eq!(space.matrix[0][0], None);
        assert_eq!(space.matrix[0][1], None);
        assert_eq!(space.matrix[0][2], Some(Tile::Galaxy));
        assert_eq!(space.matrix[0][3], None);
        assert_eq!(space.matrix[0][4], None);
        assert_eq!(space.matrix[0][5], Some(Tile::Galaxy));
        assert_eq!(space.matrix[0][6], None);
        assert_eq!(space.matrix[0][7], None);


        let mut space = Space::new(vec![
            vec![None],
            vec![Some(Tile::Galaxy)],
            vec![None],
            vec![Some(Tile::Galaxy)],
            vec![None],
        ]);

        space.expand(2);

        assert_eq!(space.matrix.len(), 8);
        assert_eq!(space.matrix[0].len(), 1);

        assert_eq!(space.matrix[0][0], None);
        assert_eq!(space.matrix[1][0], None);
        assert_eq!(space.matrix[2][0], Some(Tile::Galaxy));
        assert_eq!(space.matrix[3][0], None);
        assert_eq!(space.matrix[4][0], None);
        assert_eq!(space.matrix[5][0], Some(Tile::Galaxy));
        assert_eq!(space.matrix[6][0], None);
        assert_eq!(space.matrix[7][0], None);
    }
}