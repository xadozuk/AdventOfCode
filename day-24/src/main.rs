use std::time::Instant;

use crate::system::Hail;

#[allow(dead_code)]
const DEBUG: bool = false;
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

mod system;

fn main()
{
    let start = Instant::now();
    let content = std::fs::read_to_string("./day-24/input.txt").unwrap();

    let h = Hail::from(content.as_str());

    let result = h.intersections_2d_between(200_000_000_000_000.0, 400_000_000_000_000.0);

    println!("Result: {} ({:?})",
        result,
        start.elapsed()
    );

    let rock = h.find_throw();

    let result2 = rock.coords.x + rock.coords.y + rock.coords.z;
    println!("Result 2: {} ({:?})",
        result2,
        start.elapsed());

    println!("Total time: {:?}", start.elapsed());
}


#[cfg(test)]
mod tests
{
    use crate::system::{Hail, Point3d};

    #[test]
    fn test_example_p1()
    {
        let h = Hail::from(
"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"
);

        assert_eq!(h.intersections_2d_between(7.0, 27.0), 2);
    }

    #[test]
    fn test_example_p2()
    {
        let h = Hail::from(
"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"
);

        let rock = h.find_throw();

        assert_eq!(rock.coords, Point3d::new(24, 13, 10));
        assert_eq!(rock.velocity, Point3d::new(-3, 1, 2));
    }
}