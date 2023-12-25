use std::{time::Instant, result};

use crate::tower::Tower;

#[allow(dead_code)]
const DEBUG: bool = true;
#[allow(dead_code)]
const VERBOSE: bool = true;
#[allow(dead_code)]
const DRAW: bool = true;

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

mod tower;

fn main()
{
    let start = Instant::now();
    let content = std::fs::read_to_string("./day-22/input.txt").unwrap();

    let mut t = Tower::from(content.as_str());

//     let mut t = Tower::from(
// "1,0,1~1,2,1
// 0,0,2~2,0,2
// 0,2,3~2,2,3
// 0,0,4~0,2,4
// 2,0,5~2,2,5
// 0,1,6~2,1,6
// 1,1,8~1,1,9"
// );

    t.apply_gravity();

    // t.render();

    let result = t.safe_bricks_to_disintegrate().len();
    println!("Result: {} ({:?})",
        result,
        start.elapsed()
    );

    let result2 = t.falling_bricks_on_disintegrate();
    println!("Result 2: {} ({:?})",
        result2,
        start.elapsed());

    println!("Total time: {:?}", start.elapsed());
}


#[cfg(test)]
mod tests
{
    use crate::tower::Tower;

    #[test]
    fn test_example_p1()
    {
        let mut t = Tower::from(
"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
);

        // t.debug_info(vec![]);

        t.apply_gravity();

        assert_eq!(t.safe_bricks_to_disintegrate().len(), 5);
        assert_eq!(t.falling_bricks_on_disintegrate(), 7);
    }

    #[test]
    fn test_example_p1_2()
    {
        let mut t = Tower::from(
"3,0,1~3,0,1
2,0,2~5,0,2
0,0,3~3,0,3
5,0,3~6,0,3
2,0,4~2,0,5
3,0,4~5,0,4"
);

        t.apply_gravity();

        assert_eq!(t.safe_bricks_to_disintegrate().len(), 3);
    }
}