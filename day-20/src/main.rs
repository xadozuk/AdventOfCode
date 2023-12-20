use std::time::Instant;

use crate::factory::Factory;

#[allow(dead_code)]
const DEBUG: bool = false;
const VERBOSE: bool = true;

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

mod factory;
mod modules;

fn main()
{
    let start = Instant::now();
    let content = std::fs::read_to_string("./day-20/input.txt").unwrap();

    let mut f = Factory::from(content.as_str());

    f.run(1000);
    let result = f.low_pulses() * f.high_pulses();
    println!("Result: {} ({:?})",
        result,
        start.elapsed()
    );

    let result2 = f.run_until_low_rx();
    println!("Result 2: {} ({:?})",
        result2,
        start.elapsed());

    println!("Total time: {:?}", start.elapsed());
}


#[cfg(test)]
mod tests
{
    use crate::factory::Factory;

    #[test]
    fn test_example_p1_1_1it()
    {
        let mut f = Factory::from(
"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
);
        f.run(1);

        assert_eq!(f.low_pulses(), 8);
        assert_eq!(f.high_pulses(), 4);
    }

    #[test]
    fn test_example_p1_1_1000it()
    {
        let mut f = Factory::from(
"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
);
        f.run(1000);

        assert_eq!(f.low_pulses(), 8000);
        assert_eq!(f.high_pulses(), 4000);
    }

    #[test]
    fn test_example_p1_2_4it()
    {
        let mut f = Factory::from(
"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
);

        // One cycle
        f.run(4);

        assert_eq!(f.low_pulses(), 17);
        assert_eq!(f.high_pulses(), 11);
    }

    #[test]
    fn test_example_p1_2_1000it()
    {
        let mut f = Factory::from(
"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
);

        // One cycle
        f.run(1000);

        assert_eq!(f.low_pulses(), 4250);
        assert_eq!(f.high_pulses(), 2750);
    }
}