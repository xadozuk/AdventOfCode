use crate::facility::{Facility, RIGHT, Coord};

const DEBUG: bool = true;

macro_rules! debug {
    ($($arg:tt)*) => {{
        if $crate::DEBUG
        {
            print!("[DEBUG] ");
            println!($($arg)*);
        }
    }};
}

mod facility;

fn main()
{
    let content = std::fs::read_to_string("./day-16/input.txt").unwrap();

    let mut facility = Facility::from(content.as_str());
    facility.start_beam(Coord::new(0, 0), RIGHT);

    println!("Result: {}", facility.energized_tiles_count());

    let result2 = facility.find_most_enegized_starting_point();
    println!("Result 2: {}", result2.2);
}

#[cfg(test)]
mod tests
{
    use crate::facility::{Facility, RIGHT, Coord, DOWN};

    #[test]
    fn text_example1()
    {
        let mut f = Facility::from(
r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#);

        f.start_beam(Coord::new(0, 0), RIGHT);

        assert_eq!(f.energized_tiles_count(), 46);
    }

    #[test]
    fn test_example2()
    {
        let mut f = Facility::from(
r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#);

        let (coord, direction, energized_tiles) = f.find_most_enegized_starting_point();

        assert_eq!(coord, Coord::new(0, 3));
        assert_eq!(direction, DOWN);
        assert_eq!(energized_tiles, 51);
    }
}