use std::fmt::Display;
use std::ops::{Sub, Div, Add};

use num_integer::gcd;

type Coord2d = (f64, f64);

#[derive(PartialEq, Eq, Debug)]
pub struct Point3d<T>
{
    pub x: T,
    pub y: T,
    pub z: T
}

pub struct Hail
{
    hailstones: Vec<Hailstone>
}

pub struct Hailstone
{
    pub coords: Point3d<i128>,
    pub velocity: Point3d<i128>
}

impl From<&str> for Hail
{
    fn from(value: &str) -> Self
    {
        let hailstones = value.split("\n")
            .map(|row| Hailstone::from(row))
            .collect();

        Hail { hailstones }
    }
}

impl From<&str> for Hailstone
{
    fn from(value: &str) -> Self
    {
        let mut parts = value.split("@");

        let mut coords = parts.next().unwrap()
            .split(",")
            .map(|p| p.trim().parse::<i128>().unwrap());

        let mut velocity = parts.next().unwrap()
            .split(",")
            .map(|p| p.trim().parse::<i128>().unwrap());

        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        let z = coords.next().unwrap();

        let vx = velocity.next().unwrap();
        let vy = velocity.next().unwrap();
        let vz = velocity.next().unwrap();

        Hailstone {
            coords: Point3d::new(x, y, z),
            velocity: Point3d::new(vx, vy, vz)
        }
    }
}

impl Display for Hailstone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_fmt(format_args!(
            "{}, {}, {} @ {}, {}, {}",
            self.coords.x, self.coords.y, self.coords.z,
            self.velocity.x, self.velocity.y, self.velocity.z
        ))
    }
}

impl<T> Point3d<T>
    where T: Default
{
    pub fn new(x: T, y: T, z: T) -> Self
    {
        Point3d { x, y, z }
    }

    pub fn zero() -> Self
    {
        Self::new(T::default(), T::default(), T::default())
    }
}

impl<T> Sub<Point3d<T>> for Point3d<T>
    where T: Sub<Output = T> + Copy + Default
{
    type Output = Point3d<T>;

    fn sub(self, rhs: Point3d<T>) -> Self::Output
    {
        &self - &rhs
    }
}


impl<T> Sub<&Point3d<T>> for &Point3d<T>
    where T: Sub<Output = T> + Copy + Default
{
    type Output = Point3d<T>;

    fn sub(self, rhs: &Point3d<T>) -> Self::Output
    {
        Point3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Add<&Point3d<T>> for &Point3d<T>
    where T: Add<Output = T> + Copy + Default
{
    type Output = Point3d<T>;

    fn add(self, rhs: &Point3d<T>) -> Self::Output
    {
        Point3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Div<T> for Point3d<T>
    where T: Div<Output = T> + Default + Copy
{
    type Output = Point3d<T>;

    fn div(self, rhs: T) -> Self::Output
    {
        Point3d::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs
        )
    }
}

impl Hailstone
{
    fn intersect_2d(&self, other: &Hailstone) -> Option<(f64, Coord2d)>
    {
        let (x1, y1) = (self.coords.x, self.coords.y);
        let (dx1, dy1) = (self.velocity.x, self.velocity.y);
        let (x2, y2) = (other.coords.x, other.coords.y);
        let (dx2, dy2) = (other.velocity.x, other.velocity.y);

        // Don't use division to avoid rounding error
        let denominator = dx2 * dy1 - dy2 * dx1;
        if denominator == 0 { return None; }

        let m_num = dx2 * (y2 - y1) + dy2 * (x1 - x2);
        let n_num = dx1 * (y2 - y1) + dy1 * (x1 - x2);

        let m = m_num as f64 / denominator as f64;
        let n = n_num as f64 / denominator as f64;

        if m < 0.0 || n < 0.0
        {
            // Intersection in the "past"
            return None;
        }

        let x = dx1 as f64 * m + x1 as f64;
        let y = dy1 as f64 * m + y1 as f64;

        Some((m, (x, y)))
    }

    fn pos_at(&self, t: i128) -> Point3d<i128>
    {
        Point3d::new(
            t * self.velocity.x + self.coords.x,
            t * self.velocity.y + self.coords.y,
            t * self.velocity.z + self.coords.z,
        )
    }
}

impl Sub<&Hailstone> for &Hailstone
{
    type Output = Hailstone;

    fn sub(self, rhs: &Hailstone) -> Self::Output
    {
        Hailstone {
            coords: &self.coords - &rhs.coords,
            velocity: &self.velocity - &rhs.velocity
        }
    }
}

impl Add<&Hailstone> for &Hailstone
{
    type Output = Hailstone;

    fn add(self, rhs: &Hailstone) -> Self::Output
    {
        Hailstone {
            coords: &self.coords + &rhs.coords,
            velocity: &self.velocity + &rhs.velocity
        }
    }
}

impl Hail
{
    pub fn intersections_2d_between(&self, from: f64, to: f64) -> usize
    {
        let mut intersections = vec![];

        for (i, a) in self.hailstones.iter().enumerate()
        {
            for b in self.hailstones.iter().skip(i + 1)
            {
                debug!("[INTERSECT] A: {}", a);
                debug!("[INTERSECT] B: {}", b);

                if let Some((time, coords)) = a.intersect_2d(b)
                {
                    debug!("[INTERSECT] Found ({:.3}, {:.3}) for n={:.3}ns", coords.0, coords.1, time);

                    if time >= 1.0 && self.is_intersect_2d_between(coords, from, to)
                    {
                        debug!("[INTERSECT] INSIDE!");
                        intersections.push((time, coords));
                    }
                }
                else
                {
                    debug!("[INTERSECT] No intersections");
                }

                debug!();
            }
        }

        intersections.len()
    }

    fn is_intersect_2d_between(&self, intersect: Coord2d, start: f64, end: f64) -> bool
    {
        start <= intersect.0 && intersect.0 <= end &&
        start <= intersect.1 && intersect.1 <= end
    }

    pub fn find_throw(&self) -> Hailstone
    {
        // Change perspective from 1st hailstone
        let origin_hailstone = self.hailstones.first().unwrap();
        let hailstones: Vec<_> = self.hailstones.iter()
            .skip(1)
            .map(|h| h - origin_hailstone)
            .collect();

        // We find the plane formed by origin (first hailstone) and second hailstone path
        let second_hailstone = hailstones.first().unwrap();
        let third_hailstone = hailstones.get(1).unwrap();
        let fourth_hailstone = hailstones.get(2).unwrap();
        let plane = plane_from_origin_and(second_hailstone);

        let (p3, t3) = plane_line_intersection(&plane, third_hailstone);
        let (p4, t4) = plane_line_intersection(&plane, fourth_hailstone);

        let dt = t4 - t3;
        let dp = &p4 - &p3;

        let velocity = dp / dt;
        let pos = Point3d::new(
            p3.x - t3 * velocity.x,
            p3.y - t3 * velocity.y,
            p3.z - t3 * velocity.z
        );

        &Hailstone { coords: pos, velocity } + origin_hailstone
    }
}

fn plane_line_intersection(plane: &Point3d<i128>, hailstone: &Hailstone) -> (Point3d<i128>, i128)
{
    let num = plane.x * hailstone.coords.x + plane.y * hailstone.coords.y + plane.z * hailstone.coords.z;
    let denom = plane.x * hailstone.velocity.x + plane.y * hailstone.velocity.y + plane.z * hailstone.velocity.z;

    let t = -num / denom;

    (hailstone.pos_at(t), t)
}

fn plane_from_origin_and(hailstone: &Hailstone) -> Point3d<i128>
{
    let p1 = hailstone.pos_at(0);
    let p2 = hailstone.pos_at(1);

    let mut x = p1.y * p2.z - p1.z * p2.y;
    let mut y = p1.z * p2.x - p1.x * p2.z;
    let mut z = p1.x * p2.y - p1.y * p2.x;

    let denominator = gcd(gcd(x, y), z);

    // Invert normal vector (to ease debugging)
    if x < 0 && y < 0 && z < 0
    {
        x = -x;
        y = -y;
        z = -z;
    }

    Point3d::new(
        x / denominator,
        y / denominator,
        z / denominator
    )
}