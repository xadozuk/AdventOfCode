use core::panic;

pub type Coord = (i64, i64);

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction
{
    Up,
    Down,
    Left,
    Right
}

pub struct Instruction
{
    direction: Direction,
    length: usize
}

pub struct Digger
{
    plan: Vec<Instruction>,
    points: Vec<Coord>,
}

impl Instruction
{
    pub fn from(content: &str) -> Self
    {
        let mut parts = content.split(' ');
        let direction = match parts.next().unwrap()
        {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction")
        };

        let length = parts.next().unwrap().parse::<usize>().unwrap();

        Instruction { direction, length }
    }

    pub fn from2(content: &str) -> Self
    {
        let parts = content.split(' ');
        let inst = parts.skip(2).next().unwrap();

        // Strip (# and )
        let inst = &inst[2..inst.len() - 1];

        let length = usize::from_str_radix(&inst[0..5], 16).unwrap();
        let direction = match &inst[5..6]
        {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("Unknown direction")
        };

        Instruction { direction, length }
    }
}

impl From<&str> for Digger
{
    fn from(value: &str) -> Self
    {
        let plan = value.split('\n')
            .map(|row| {
                Instruction::from(row)
            })
            .collect();

        Digger { plan, points: vec![] }
    }
}

impl Digger
{
    pub fn from2(value: &str) -> Self
    {
        let plan = value.split('\n')
            .map(|row| {
                Instruction::from2(row)
            })
            .collect();

        Digger { plan, points: vec![] }
    }

    pub fn dig(&mut self)
    {
        self.points.clear();

        let mut current_coord = (0, 0);

        self.points.push(current_coord);

        // Dig edge
        for inst in &self.plan
        {
            current_coord = move_in(current_coord, inst.direction, inst.length);
            self.points.push(current_coord);
        }
    }

    pub fn cubic_meters(&self) -> i64
    {
        let s1 = self.points.iter().zip(self.points.iter().skip(1))
            .fold(0, |acc, (a, b)| acc + a.0 * b.1);

        let s2 = self.points.iter().zip(self.points.iter().skip(1))
            .fold(0, |acc, (a, b)| acc + a.1 * b.0);

        let perimeter = self.points.iter().zip(self.points.iter().skip(1))
            .fold(0, |acc, (a, b)| {
                // "Rectangular" polygon so it work as either x or y coord will be diff than 0
                acc + (a.0 - b.0 + (a.1 - b.1)).abs()
            });

        ((s1 - s2) / 2).abs() + perimeter / 2 + 1
    }
}

fn move_in((r, c): Coord, direction: Direction, length: usize) -> Coord
{
    match direction
    {
        Direction::Up => (r - length as i64, c),
        Direction::Down => (r + length as i64, c),
        Direction::Left => (r, c - length as i64),
        Direction::Right => (r, c + length as i64)
    }
}