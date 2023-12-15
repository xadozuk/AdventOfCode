const DEBUG: bool = false;

macro_rules! debug {
    ($($arg:tt)*) => {{
        if $crate::DEBUG
        {
            print!("[DEBUG] ");
            println!($($arg)*);
        }
    }};
}

mod hashmap;

use hashmap::Factory;

fn main()
{
    let content = std::fs::read_to_string("./day-15/input.txt").unwrap();

    let hashes: Vec<_> = content.split(',')
        .map(|str| hash(str))
        .collect();

    println!("Result: {}", hashes.iter().sum::<u32>());

    let mut factory = Factory::new();
    factory.run(&std::fs::read_to_string("./day-15/input.txt").unwrap());

    println!("Result 2: {}", factory.focusing_power());
}

fn hash(value: &str) -> u32
{
    value.chars()
        .fold(0, |acc, c| {
            (acc + (c as u32)) * 17 % 256
        })
}

#[cfg(test)]
mod tests
{
    use crate::{hash, hashmap::Factory};

    #[test]
    fn test_example1()
    {
        let string = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let hashes: Vec<_> = string.split(',').map(|str| hash(str)).collect();

        assert_eq!(hashes, vec![
            30,
            253,
            97,
            47,
            14,
            180,
            9,
            197,
            48,
            214,
            231
        ]);
    }

    #[test]
    fn test_example2()
    {
        let string = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let mut factory = Factory::new();

        factory.run(string);

        assert_eq!(factory.focusing_power(), 145);
    }
}