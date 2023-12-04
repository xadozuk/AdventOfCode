use std::cmp::max;

pub struct CubeSet
{
    pub green: u32,
    pub red: u32,
    pub blue: u32
}

pub struct Game
{
    pub id: u32,
    sets: Vec<CubeSet>
}

impl Game
{
    pub fn can_have_set(&self, ref_set: &CubeSet) -> bool
    {
        let mut result = true;

        for set in &self.sets
        {
            if  set.red     > ref_set.red ||
                set.green   > ref_set.green  ||
                set.blue    > ref_set.blue
            {
                result = false
            }
        }

        return result;
    }

    pub fn min_cube_set(&self) -> CubeSet
    {
        let mut min_cub_set = CubeSet { red: 0, green: 0, blue: 0 };

        for set in &self.sets
        {
            min_cub_set.red   = max(min_cub_set.red, set.red);
            min_cub_set.green = max(min_cub_set.green, set.green);
            min_cub_set.blue  = max(min_cub_set.blue, set.blue);
        }

        return min_cub_set;
    }
}

impl CubeSet
{
    pub fn power(&self) -> u32
    {
        return self.red * self.green * self.blue;
    }
}

impl From<&str> for CubeSet
{
    fn from(value: &str) -> Self
    {
        let mut set = CubeSet { red: 0, green: 0, blue: 0 };
        let colors = value.split(',');

        for color_content in colors
        {
            let mut color_parts = color_content.trim().split(' ');
            let count = color_parts.next().unwrap().parse::<u32>().unwrap();
            let color = color_parts.next().unwrap();

            match color
            {
                "red" => set.red = count,
                "green" => set.green = count,
                "blue" => set.blue = count,
                _default =>
                {
                    panic!("Unknown color: {0}", _default);
                }
            }
        }

        return set;
    }
}

impl From<&str> for Game
{
    fn from(value: &str) -> Self
    {
        let mut parts = value.split(':');
        let game_id_part = parts.next().unwrap().trim();
        let sets_part    = parts.next().unwrap().trim();

        let sets = sets_part.split(';').map(|set_content|
        {
            CubeSet::from(set_content.trim())
        }).collect();

        return Game {
            id: game_id_part.replace("Game ", "").parse::<u32>().unwrap(),
            sets: sets
        };
    }
}