use std::fs;

mod pattern;
use pattern::Pattern;

fn main()
{
    let patterns = parse(&fs::read_to_string("./day-13/input.txt").unwrap());

    let result = summarize(&patterns, 0);
    let result2 = summarize(&patterns, 1);

    println!("Result: {}", result);
    println!("Result 2: {}", result2);
}

fn parse(file_content: &str) -> Vec<Pattern>
{
    file_content.split("\n\n")
        .map(|p| Pattern::from(p))
        .collect()
}

fn summarize(patterns: &Vec<Pattern>, error_count: usize) -> usize
{
    let vertical_reflections: Vec<_> = patterns.iter()
        .map(|p| p.vertical_reflection_with_errors(error_count))
        .collect();

    let n_v_reflections: usize = vertical_reflections.iter()
        .filter(|r| r.is_some())
        .map(|r| r.unwrap().0 + 1)
        .sum();

    let horizontal_reflections: Vec<_> = patterns.iter()
        .map(|p| p.horizontal_reflection_with_errors(error_count))
        .collect();

    let n_h_reflections: usize = horizontal_reflections.iter()
        .filter(|r| r.is_some())
        .map(|r| r.unwrap().0 + 1)
        .sum();

    n_v_reflections + 100 * n_h_reflections
}

#[cfg(test)]
mod tests
{
    use crate::{parse, summarize};

    #[test]
    fn text_example1()
    {
        let patterns = parse(
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#");

        assert_eq!(patterns.len(), 2);
        assert_eq!(summarize(&patterns, 0), 405);
    }

    #[test]
    fn test_example2()
    {
        let patterns = parse(
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#");

        assert_eq!(summarize(&patterns, 1), 400);
    }
}