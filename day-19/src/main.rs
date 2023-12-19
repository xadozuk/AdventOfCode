use std::time::Instant;

use crate::sorter::Sorter;

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

mod sorter;
mod ranges;

fn main()
{
    let start = Instant::now();
    let content = std::fs::read_to_string("./day-19/input.txt").unwrap();

    let s = Sorter::from(content.as_str());

    let result = s.run();
    println!("Result: {} ({:?})",
        result,
        start.elapsed()
    );

    let result2 = s.accepted_part_combinations();
    println!("Result 2: {} ({:?})",
        result2,
        start.elapsed());

    println!("Total time: {:?}", start.elapsed());
}


#[cfg(test)]
mod tests
{
    use crate::sorter::Sorter;

    #[test]
    fn test_example1()
    {
        let s = Sorter::from(
"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
        );

        assert_eq!(s.run(), 19114);
    }


    #[test]
    fn test_example2()
    {
        let s = Sorter::from(
"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
        );

        assert_eq!(s.accepted_part_combinations(),  167409079868000);
    }

    #[test]
    fn test_simple_example2()
    {
        let s = Sorter::from(
"in{s<1000:A,bbb}
bbb{s>3000:A,ccc}
ccc{x<1000:A,R}

{x=787,m=2655,a=1222,s=2876}"
        );

        assert_eq!(s.accepted_part_combinations(),  159919984000000);
    }

}