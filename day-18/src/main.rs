use std::time::Instant;

use crate::digger::Digger;

#[allow(dead_code)]
const DEBUG: bool = true;

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

mod digger;

fn main()
{
    let start = Instant::now();
    let content = std::fs::read_to_string("./day-18/input.txt").unwrap();

    let mut digger = Digger::from(content.as_str());
    digger.dig();

    let result = digger.cubic_meters();
    println!("Result: {} ({:?})",
        result,
        start.elapsed()
    );

    let mut digger = Digger::from2(content.as_str());
    digger.dig();

    let result2 = digger.cubic_meters();
    println!("Result 2: {} ({:?})",
        result2,
        start.elapsed());

    println!("Total time: {:?}", start.elapsed());
}


#[cfg(test)]
mod tests
{
    use crate::digger::Digger;

    #[test]
    fn test_example1()
    {
        let mut d = Digger::from(
"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
        );

        d.dig();

        assert_eq!(d.cubic_meters(), 62);
    }

    #[test]
    fn test_example2()
    {
        let mut d = Digger::from2(
"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
        );

        d.dig();

        assert_eq!(d.cubic_meters(), 952408144115);
    }
}