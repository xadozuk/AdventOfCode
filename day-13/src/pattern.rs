#[derive(PartialEq, Eq, Debug)]
pub enum Tile
{
    Ash,
    Rock
}

pub struct Pattern
{
    matrix: Vec<Vec<Tile>>
}

impl Pattern
{
    pub fn width(&self) -> usize
    {
        if self.matrix.len() > 0 { self.matrix[0].len() }
        else { 0 }
    }

    pub fn height(&self) -> usize
    {
        self.matrix.len()
    }

    pub fn row(&self, index: usize) -> Vec<&Tile>
    {
        self.matrix[index].iter()
            .map(|t| t)
            .collect()
    }

    pub fn col(&self, index: usize) -> Vec<&Tile>
    {
        self.matrix.iter()
            .map(|row| &row[index])
            .collect()
    }

    pub fn vertical_reflection_with_errors(&self, error_count: usize) -> Option<(usize, usize)>
    {
        for j in 1..self.width()
        {
            let n_diffs = self.vertical_reflection_diffs(((j - 1), j));

            if n_diffs == error_count
            {
                return Some((j - 1, j))
            }
        }

        None
    }

    fn vertical_reflection_diffs(&self, coord: (usize, usize)) -> usize
    {
        let iter = (0..=coord.0).rev().zip(coord.1..self.width());
        let mut n_diffs = 0;

        for (a, b) in iter
        {
            n_diffs += diffs(&self.col(a), &self.col(b));
        }

        return n_diffs;
    }

    pub fn horizontal_reflection_with_errors(&self, error_count: usize) -> Option<(usize, usize)>
    {
        for i in 1..self.height()
        {
            let n_diffs = self.horizontal_reflection_diffs(((i - 1), i));

            if n_diffs == error_count
            {
                return Some((i - 1, i))
            }
        }

        None
    }

    fn horizontal_reflection_diffs(&self, coord: (usize, usize)) -> usize
    {
        let iter = (0..=coord.0).rev().zip(coord.1..self.height());
        let mut n_diffs = 0;

        for (a, b) in iter
        {
            n_diffs += diffs(&self.row(a), &self.row(b));
        }

        return n_diffs;
    }
}

impl From<&str> for Pattern
{
    fn from(value: &str) -> Self
    {
        let tiles = value.lines()
            .map(|row| {
                row.chars().map(|c| {
                    match c
                    {
                        '#' => Tile::Rock,
                        _   => Tile::Ash
                    }
                })
                .collect()
            })
            .collect();

        Pattern { matrix: tiles }
    }
}

fn diffs(a: &Vec<&Tile>, b: &Vec<&Tile>) -> usize
{
    if a.len() != b.len() { panic!("Vec must have the same size.") }

    a.iter().enumerate()
        .map(|(i, t)| {
            if b[i] != *t { 1 }
            else { 0 }
        })
        .sum::<usize>()
}