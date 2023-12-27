use std::time::Instant;

use crate::walk::Walk;

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

mod walk;
mod graph;

fn main()
{
    let start = Instant::now();
    let content = std::fs::read_to_string("./day-23/input.txt").unwrap();

    let mut w = Walk::from(content.as_str());
    w.compute_graph();

    let result = w.max_hike();

    println!("Result: {} ({:?})",
        result,
        start.elapsed()
    );

    let mut w = Walk::from(content.as_str());
    w.set_slippy(false);
    w.compute_graph();

    w.graph().to_mermaid_chart("./day-23/chart.txt", w.start).unwrap();

    let result2 = w.max_hike();
    println!("Result 2: {} ({:?})",
        result2,
        start.elapsed());

    println!("Total time: {:?}", start.elapsed());
}


#[cfg(test)]
mod tests
{
    use crate::walk::Walk;

    #[test]
    fn test_example_p1()
    {
        let mut l = Walk::from(
"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
);

        l.compute_graph();

        assert_eq!(l.max_hike(), 94);
    }

    #[test]
    fn test_example_p2_1()
    {
        let mut l = Walk::from(
"#.#############
#.............#
#.#####.#####.#
#.............#
#.#####.#####.#
#.............#
#.#####.#####.#
#.............#
#############.#"
);

        l.set_slippy(false);
        l.compute_graph();

        l.graph().debug((0, 1));

        l.graph().to_mermaid_chart("./chart.ex.txt", (0, 1)).unwrap();

        assert_eq!(l.max_hike(), 48);
    }

    #[test]
    fn test_example_p2_2()
    {
        let mut l = Walk::from(
"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
);

        l.set_slippy(false);
        l.compute_graph();

        assert_eq!(l.max_hike(), 154);
    }
}