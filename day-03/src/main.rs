use std::fs::File;
use std::io::{prelude::*, BufReader};

mod matrix;

use matrix::Matrix;

fn main()
{
    let file = File::open("./day-03/input.txt").unwrap();
    let buffer = BufReader::new(file);

    let matrix = Matrix::new(buffer.lines().collect::<Result<_, _>>().unwrap());

    let result: u32 = matrix
        .motor_parts()
        .iter()
        .sum();

    let result2: u32 = matrix
        .gear_ratios()
        .iter()
        .sum();

    println!("Result: {0}", result);
    println!("Result 2: {0}", result2);
}
