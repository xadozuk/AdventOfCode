use std::time::Instant;

use crate::garden::Garden;

#[allow(dead_code)]
const DEBUG: bool = false;
#[allow(dead_code)]
const VERBOSE: bool = true;
#[allow(dead_code)]
const DRAW: bool = false;

#[allow(unused_macros)]
macro_rules! debug {
    ($($arg:tt)*) => {{
        if $crate::DEBUG
        {
            print!("[*] ");
            println!($($arg)*);
        }
    }};
}

#[allow(unused_macros)]
macro_rules! verbose {
    ($($arg:tt)*) => {{
        if $crate::VERBOSE
        {
            print!("[VERBOSE] ");
            println!($($arg)*);
        }
    }};
}

mod garden;

fn main()
{
    let start = Instant::now();
    let content = std::fs::read_to_string("./day-21/input.txt").unwrap();

    let g = Garden::from(content.as_str());

    // let coords = g.walk(64, false);
    // let result = coords.len();
    // println!("Result: {} ({:?})",
    //     result,
    //     start.elapsed()
    // );

    let result2 = g.walk_optimized(26_501_365);
    println!("Result 2: {} ({:?})",
        result2,
        start.elapsed());

    println!("Total time: {:?}", start.elapsed());
}


#[cfg(test)]
mod tests
{
    use crate::garden::Garden;

    #[test]
    fn test_example_p1()
    {
        let f = Garden::from(
"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
);
        let coords = f.walk(6, false);

        assert_eq!(coords.len(), 16);
    }

    #[test]
    fn test_example_p1_2()
    {
        let f = Garden::from(
"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
);

        assert_eq!(f.walk(10, true).len(), 50);
        assert_eq!(f.walk(50, true).len(), 1594);
        assert_eq!(f.walk(100, true).len(), 6536);
        assert_eq!(f.walk(500, true).len(), 167004);
        assert_eq!(f.walk(1000, true).len(), 668697);
        //assert_eq!(f.walk(5000, true).len(), 16_733_044);
    }
}