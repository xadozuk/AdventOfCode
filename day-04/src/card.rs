pub struct Card
{
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>
}

impl Card
{
    pub fn score(&self) -> u32
    {
        let n_winning_numbers = self.matching_numbers();

        if n_winning_numbers == 0
        {
            return 0;
        }
        else
        {
            return 2u32.pow(n_winning_numbers - 1);
        }
    }

    pub fn matching_numbers(&self) -> u32
    {
        let mut matching_numbers = 0;

        for number in &self.numbers
        {
            if self.winning_numbers.contains(number)
            {
                matching_numbers += 1;
            }
        }

        return matching_numbers
    }
}

impl From<&str> for Card
{
    fn from(value: &str) -> Self
    {
        let mut id_parts = value.split(':');
        let id = id_parts.next().unwrap().replace("Card", "").trim().parse::<u32>().unwrap();

        let mut numbers_parts = id_parts.next().unwrap().split('|').map(|p| p.trim());

        let winning_numbers = numbers_parts.next().unwrap()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse::<u32>().unwrap())
            .collect();

        let numbers = numbers_parts.next().unwrap()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse::<u32>().unwrap())
            .collect();

        return Card {
            id: id,
            winning_numbers: winning_numbers,
            numbers: numbers
        }
    }
}