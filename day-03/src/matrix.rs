use std::{collections::HashMap, ops::Range};

pub struct Matrix
{
    data: Vec<Vec<char>>,
    symbol_indexes: Vec<(usize, usize)>
}

impl Matrix
{
    pub fn new(lines: Vec<String>) -> Self
    {
        let mut matrix = Matrix {
            data: vec![],
            symbol_indexes: vec![]
        };

        for (i, line) in lines.iter().enumerate()
        {
            let mut line_vec = vec![];

            for (j, char) in line.chars().enumerate()
            {
                line_vec.push(char);

                if char != '.' && !char.is_digit(10)
                {
                    matrix.symbol_indexes.push((i, j));
                }
            }

            matrix.data.push(line_vec);
        }

        return matrix;
    }

    pub fn symbols_iter(&self) -> impl Iterator<Item=&(usize, usize)> + '_
    {
        return self.symbol_indexes.iter();
    }

    pub fn motor_parts(&self) -> Vec<u32>
    {
        let mut parts = vec![];

        for symbol_pos in self.symbols_iter()
        {
            parts.append(&mut self.neighboor_parts(symbol_pos))
        }

        return parts;
    }

    pub fn gear_ratios(&self) -> Vec<u32>
    {
        let mut gears = vec![];

        for symbol_pos in self.symbols_iter()
        {
            if self.data[symbol_pos.0][symbol_pos.1] != '*'
            {
                continue;
            }

            let parts = self.neighboor_parts(symbol_pos);

            if parts.len() != 2
            {
                continue;
            }

            gears.push(parts.iter().product());
        }

        return gears;
    }

    fn neighboor_parts(&self, pos: &(usize, usize)) -> Vec<u32>
    {
        if self.data.len() < pos.0 || self.data[0].len() < pos.1
        {
            panic!("Outside of matrix");
        }

        // Use HashMap to avoid duplicates
        let mut neighboors = HashMap::new();

        // Search around the pos

        for x in -1..=1
        {
            for y in -1..=1
            {
                let search_pos: (isize, isize) = (pos.0 as isize + x, pos.1 as isize + y);

                if  search_pos.0 < 0 ||
                    search_pos.1 < 0 ||
                    search_pos.0 >= self.data.len() as isize ||
                    search_pos.1 >= self.data[0].len() as isize
                {
                    continue;
                }

                if self.data[search_pos.0 as usize][search_pos.1 as usize].is_digit(10)
                {
                    let motor_part = self.part_at((search_pos.0 as usize, search_pos.1 as usize));
                    neighboors.insert(motor_part.0, motor_part.1);
                }
            }
        }

        return neighboors.values().map(|value| value.to_owned()).collect()
    }

    fn part_at(&self, pos: (usize, usize)) -> ((usize, Range<usize>), u32)
    {
        let mut start = 0;
        let mut end = self.data.len();

        // find beginning of motor part
        for j in 1..=pos.1
        {
            if !self.data[pos.0][pos.1 - j].is_digit(10)
            {
                start = pos.1 - j + 1;
                break;
            }
        }

        // If we didn't find a blank, the motor part start at the beginning

        // find end of motor part
        for j in 1..(self.data[pos.0].len() - pos.1)
        {
            if !self.data[pos.0][pos.1 + j].is_digit(10)
            {
                end = pos.1 + j;
                break;
            }
        }
        // If we didn't find a blank, the motor part finish at the end of the line

        let number = (start..end)
            .map(|j| self.data[pos.0][j])
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        return ((pos.0, (start..end)), number)
    }
}