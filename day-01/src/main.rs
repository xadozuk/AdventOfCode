use std::fs::File;
use std::io::{prelude::*, BufReader};

static DIGIT_WORDS: [(&str, u32); 9] = [
    ("one",     1),
    ("two",     2),
    ("three",   3),
    ("four",    4),
    ("five",    5),
    ("six",     6),
    ("seven",   7),
    ("eight",   8),
    ("nine",    9)
];

fn find_digit(str:  &str, reversed: bool) -> Result<u32, &str>
{
    for pos in 0..(str.len())
    {
        // Take first char of substring
        let char =  &str[pos..(pos + 1)].chars().next().unwrap();

        if char.is_digit(10)
        {
            return char.to_digit(10).ok_or("Unable to parse char into u32");
        }

        for digit_word in DIGIT_WORDS
        {
            if !reversed && str[pos..str.len()].starts_with(digit_word.0) || reversed && str[pos..str.len()].starts_with(&reverse(digit_word.0))
            {
                return Ok(digit_word.1);
            }
        }
    }

    return Err("Unable to find digit");
}

fn reverse(str: &str) -> String
{
    str.chars().rev().collect()
}

fn main()
{
    let file = File::open("./input.txt").unwrap();
    let buffer = BufReader::new(file);

    let mut sum = 0;

    for line in buffer.lines()
    {
        match line {
            Ok(content) =>
            {
                let first_digit = find_digit(&content, false).unwrap();
                let last_digit =  find_digit(&reverse(&content), true).unwrap();

                sum += first_digit * 10 + last_digit;
            },
            Err(e) => println!("Error reading line: {0}", e)
        }
    }

    println!("Result: {0}", sum);
}
