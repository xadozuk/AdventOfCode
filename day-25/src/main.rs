use std::time::Instant;

use crate::machine::Machine;

#[allow(dead_code)]
const DEBUG: bool = false;
#[allow(dead_code)]
const VERBOSE: bool = true;
#[allow(dead_code)]
const DRAW: bool = true;

#[allow(unused_macros)]
macro_rules! debugln {
    ($($arg:tt)*) => {{
        debug!($($arg)*);
        debug_raw!("\n");
    }};
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($arg:tt)*) => {{
        debug_raw!("[*] ");
        debug_raw!($($arg)*);
    }};
}

#[allow(unused_macros)]
macro_rules! debug_raw {
    ($($arg:tt)*) => {{
        if $crate::DEBUG
        {
            print!($($arg)*);
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

mod machine;

fn main()
{
    let start = Instant::now();
    let content = std::fs::read_to_string("./day-25/input.txt").unwrap();

    let mut m = Machine::from(content.as_str());

    let split = m.find_split();

    let result = split.0 * split.1;

    println!("Result: {} ({:?})",
        result,
        start.elapsed()
    );

    let result2 = 0;
    println!("Result 2: {} ({:?})",
        result2,
        start.elapsed());

    println!("Total time: {:?}", start.elapsed());
}


#[cfg(test)]
mod tests
{
    use crate::machine::Machine;

    #[test]
    fn test_example_p1()
    {
        let mut m = Machine::from(
"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"
);

        let (comp_a, comp_b) = m.find_split();

        assert_eq!(9, comp_a);
        assert_eq!(6, comp_b);
    }
}