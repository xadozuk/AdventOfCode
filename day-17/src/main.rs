use std::time::Instant;

use crate::factory::{Factory, Node};

const DEBUG: bool = false;

macro_rules! debug {
    ($($arg:tt)*) => {{
        if $crate::DEBUG
        {
            print!("[*] ");
            println!($($arg)*);
        }
    }};
}

mod factory;

fn main()
{
    let start = Instant::now();

    let content = std::fs::read_to_string("./day-17/input.txt").unwrap();

    let factory = Factory::from(content.as_str());
    println!("Result: {} ({:?})",
        factory.find_lesser_heat_loss(Node::new((0, 0), None, 0)),
        start.elapsed()
    );

    let factory = Factory::new(4, 10, content.as_str());
    println!("Result 2: {} ({:?})",
        factory.find_lesser_heat_loss(Node::new((0, 0), None, 0)),
        start.elapsed());

    println!("Total time: {:?}", start.elapsed());
}


#[cfg(test)]
mod tests
{
    use crate::factory::{Factory, Node, Direction};

    #[test]
    fn test_example1()
    {
        let f = Factory::from(
"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
        );

        assert_eq!(f.find_lesser_heat_loss(Node::new((0, 0), Some(Direction::Right), 0)), 102);
    }

    #[test]
    fn test_example2_1()
    {
        let f = Factory::new(4, 10,
"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
        );

        assert_eq!(f.find_lesser_heat_loss(Node::new((0, 0), None, 0)), 94);
    }

    #[test]
    fn test_example2_2()
    {
        let f = Factory::new(4, 10,
"111111111111
999999999991
999999999991
999999999991
999999999991"
        );

        assert_eq!(f.find_lesser_heat_loss(Node::new((0, 0), None, 0)), 71);
    }
}