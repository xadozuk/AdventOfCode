use std::collections::{HashMap, HashSet};

use colored::{Colorize, ColoredString};
use kiss3d::{light::Light, window::{Window, RenderLoopClosure}, nalgebra::{UnitQuaternion, Vector3, Point3, Translation3}, camera::{self, FirstPerson, ArcBall}};
use rand::Rng;
use random_color::RandomColor;

use crate::DRAW;

pub type Id = usize;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Coord3d
{
    x: u32,
    y: u32,
    z: u32
}

#[derive(Debug)]
pub struct Brick
{
    id: Id,
    coords: (Coord3d, Coord3d),
    bricks_above: HashSet<Id>,
    on_bricks: HashSet<Id>
}

pub struct Tower
{
    bricks: HashMap<Id, Brick>,
    pos_cache: HashMap<Coord3d, Id>
}

impl From<&str> for Tower
{
    fn from(value: &str) -> Self
    {
        let bricks = value.split("\n").enumerate()
            .map(|(i, brick)| (i, Brick::from(brick, i)))
            .collect();

        Tower { bricks, pos_cache: HashMap::new() }
    }
}

impl From<&str> for Coord3d
{
    fn from(value: &str) -> Self
    {
        let mut coords = value.split(",").map(|n| n.parse::<u32>().unwrap());

        Coord3d {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap()
        }
    }
}

impl From<(u32, u32, u32)> for Coord3d
{
    fn from(value: (u32, u32, u32)) -> Self
    {
        Coord3d::new(value)
    }
}

impl PartialEq for Brick
{
    fn eq(&self, other: &Self) -> bool
    {
        self.id == other.id
    }
}

impl Eq for Brick {}

impl Clone for Brick
{
    fn clone(&self) -> Self
    {
        Self {
            id: self.id.clone(),
            coords: self.coords.clone(),
            bricks_above: HashSet::new(),
            on_bricks: HashSet::new()
        }
    }
}

impl Coord3d
{
    pub fn new(value: (u32, u32, u32)) -> Self
    {
        Coord3d { x: value.0, y: value.1, z: value.2 }
    }
}

impl Brick
{
    pub fn new(a: Coord3d, b: Coord3d) -> Self
    {
        Brick {
            id: 0,
            coords: (a, b),
            bricks_above: HashSet::new(),
            on_bricks: HashSet::new()
        }
    }

    pub fn from(value: &str, id: usize) -> Self
    {
        let mut parts = value.split("~");

        let start = Coord3d::from(parts.next().unwrap());
        let end = Coord3d::from(parts.next().unwrap());

        Brick {
            id,
            coords: (start, end),
            bricks_above: HashSet::new(),
            on_bricks: HashSet::new()
        }
    }

    fn min_x(&self) -> u32 { self.coords.0.x.min(self.coords.1.x) }
    fn max_x(&self) -> u32 { self.coords.0.x.max(self.coords.1.x) }
    fn min_y(&self) -> u32 { self.coords.0.y.min(self.coords.1.y) }
    fn max_y(&self) -> u32 { self.coords.0.y.max(self.coords.1.y) }
    fn min_z(&self) -> u32 { self.coords.0.z.min(self.coords.1.z) }
    fn max_z(&self) -> u32 { self.coords.0.z.max(self.coords.1.z) }

    pub fn size(&self) -> u32
    {
        (1 + self.coords.0.x.abs_diff(self.coords.1.x)) *
        (1 + self.coords.0.y.abs_diff(self.coords.1.y)) *
        (1 + self.coords.0.z.abs_diff(self.coords.1.z))
    }

    fn surfaces_facing_down(&self) -> Vec<Coord3d>
    {
        self.surfaces_at(self.min_z()).unwrap()
    }

    fn surfaces(&self) -> Vec<Coord3d>
    {
        (self.min_z()..=self.max_z()).flat_map(|z| self.surfaces_at(z).unwrap()).collect()
    }

    fn surfaces_at(&self, z: u32) -> Option<Vec<Coord3d>>
    {
        if z < self.min_z() || z > self.max_z() { return None; }

        let x_start = self.coords.0.x.min(self.coords.1.x);
        let x_end = self.coords.0.x.max(self.coords.1.x);
        let y_start = self.coords.0.y.min(self.coords.1.y);
        let y_end = self.coords.0.y.max(self.coords.1.y);

        let mut result = vec![];

        for x in x_start..=x_end
        {
            for y in y_start..=y_end
            {
                result.push((x, y, z).into());
            }
        }

        Some(result)
    }

    fn move_down(&mut self, distance: u32)
    {
        self.coords.0.z -= distance;
        self.coords.1.z -= distance;
    }

    fn on_brick(&mut self, brick_id: Id)
    {
        self.on_bricks.insert(brick_id);
    }

    fn below_brick(&mut self, brick_id: Id)
    {
        self.bricks_above.insert(brick_id);
    }

    fn size_x(&self) -> u32 { self.max_x() - self.min_x() + 1 }
    fn size_y(&self) -> u32 { self.max_y() - self.min_y() + 1 }
    fn size_z(&self) -> u32 { self.max_z() - self.min_z() + 1 }
}

impl Clone for Tower
{
    fn clone(&self) -> Self
    {
        Tower {
            bricks: self.bricks.clone(),
            pos_cache: HashMap::new()
        }
    }
}

impl Tower
{
    pub fn apply_gravity(&mut self) -> usize
    {
        let mut bricks: Vec<_> = self.bricks.values().collect();
        bricks.sort_by(Tower::compare_bricks);

        let mut n_bricks_moved = 0;

        let ids: Vec<_> = bricks.iter().map(|b| b.id).collect();

        // Iterate over all bricks and push them down
        // Start with lower bricks
        for (n, id) in ids.into_iter().enumerate()
        {
            if n % 100 == 0
            {
                // debug!("[GRAVITY] {}/{}", n, self.bricks.len());
            }

            // For each bricks, get the surface facing down and check if we can go down
            // Take the min distance possible
            // Then update dpends_on
            let bricks_below: Vec<_> = self.bricks[&id].surfaces_facing_down().iter()
                .map(|face| self.move_down_distance(face))
                .collect();

            let max_down_distance = *bricks_below.iter()
                .min_by_key(|(dist, _)| dist)
                .map(|(dist, _)| dist)
                .unwrap();

            let bricks_below: Vec<_> = bricks_below.into_iter()
                .filter(|(dist, id)| *dist == max_down_distance && id.is_some())
                .map(|(_, id)| id.unwrap())
                .collect();

            if max_down_distance > 0
            {
                self.bricks.get_mut(&id).unwrap().move_down(max_down_distance);
                n_bricks_moved += 1;
            }

            for coord in self.bricks[&id].surfaces()
            {
                self.pos_cache.insert(coord, id);
            }

            for below_id in &bricks_below
            {
                self.bricks.get_mut(&id).unwrap().on_brick(*below_id);
                self.bricks.get_mut(&below_id).unwrap().below_brick(id);
            }
        }

        n_bricks_moved
    }

    pub fn debug_info(&self, safe_to_disintegrate: &Vec<Id>)
    {
        let mut tower: Vec<_> = vec![];
        let max_x = self.bricks.values().map(|b| b.max_x()).max().unwrap();
        let max_y = self.bricks.values().map(|b| b.max_y()).max().unwrap();
        let max_z = self.bricks.values().map(|b| b.max_z()).max().unwrap();

        for z in 1..=max_z
        {
            let mut bricks = vec![];
            for x in 0..=max_x
            {
                let b =  self.brick_at((x, 0, z).into());

                if let Some(id) = b
                {
                    let s = id.to_string();
                    let s = if safe_to_disintegrate.contains(&id) { s.green() }
                        else { s.red() };

                    bricks.push(s);
                }
                else
                {
                    bricks.push(".".white());
                }
            }

            bricks.push(" | ".white());

            for y in 0..=max_y
            {
                let b = self.brick_at((0, y, z).into());
                if let Some(id) = b
                {
                    let s = id.to_string();
                    let s = if safe_to_disintegrate.contains(&id) { s.green() }
                        else { s.red() };

                        bricks.push(s);
                }
                else
                {
                    bricks.push(".".white());
                }
            }

            tower.push(bricks);
        }

        debug!("Tower view:");
        for row in tower.iter().rev()
        {
            for b in row
            {
                print!("{:>4} ", b);
            }

            print!("\n");
        }

        debug!("On bricks:");
        for brick in self.bricks.values()
        {
            debug!(
                "{} -> {}",
                brick.id,
                brick.on_bricks.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(", ")
            );
        }

        debug!("\nBricks above:");
        for brick in self.bricks.values()
        {
            debug!(
                "{} -> {}",
                brick.id,
                brick.bricks_above.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(", ")
            );
        }
    }

    pub fn safe_bricks_to_disintegrate(&self) -> Vec<Id>
    {
        let bricks_with_none_above: Vec<_> = self.bricks.values()
            .filter(|b| b.bricks_above.len() == 0)
            .map(|b| b.id)
            .collect();

        // Search all bricks that are on multiple bricks (we will check if we can remove bricks below)
        let bricks_on_multipe: HashSet<_> = self.bricks.values()
            .filter(|b| b.on_bricks.len() >= 2)
            .map(|b| b.id)
            .collect();

        // We can only remove below bricks if they don't support another lone brick
        let mut safe_bricks_on_multiple = HashSet::new();

        for brick_id in bricks_on_multipe
        {
            let bricks_below = &self.bricks[&brick_id].on_bricks;

            for brick_below_id in bricks_below
            {
                let is_safe = self.bricks.values()
                    // We skip the current brick, and we search other bricks that are on the same below brick
                    .filter(|b| b.id != brick_id && b.on_bricks.contains(&brick_below_id))
                    // We check that other bricks are on multiple bricks, else we cannot safely remove
                    .all(|b| b.on_bricks.len() > 1);

                if is_safe { safe_bricks_on_multiple.insert(brick_below_id); }
            }
        }

        let safe_to_disintegrate: Vec<_> = bricks_with_none_above.iter()
            .chain(safe_bricks_on_multiple.into_iter())
            .map(|id| *id)
            .collect();

        if DRAW
        {
            self.debug_info(&safe_to_disintegrate);
        }

        safe_to_disintegrate
    }

    pub fn falling_bricks_on_disintegrate(&self) -> usize
    {
        let mut bricks: Vec<_> = self.bricks.values().collect();
        bricks.sort_by_key(|b| b.min_z());

        let mut falls_by_bricks = HashMap::new();

        for (i, brick) in bricks.iter().enumerate()
        {
            if i % 100 == 0
            {
                debug!("[FALLING] {}/{}", i, bricks.len())
            }

            let mut tower_clone = self.clone();
            tower_clone.bricks.remove_entry(&brick.id);

            let n_bricks_falling = tower_clone.apply_gravity();
            falls_by_bricks.insert(brick.id, n_bricks_falling);
        }

        falls_by_bricks.values().sum()
    }

    fn compare_bricks(a: &&Brick, b: &&Brick) -> std::cmp::Ordering
    {
        // 1. z coordinate (lower first)
        // 2. Size of shape
        match a.min_z().cmp(&b.min_z())
        {
            std::cmp::Ordering::Equal =>
            {
                a.size().cmp(&b.size())
            },
            default => default
        }
    }

    fn move_down_distance(&self, coord: &Coord3d) -> (u32, Option<Id>)
    {
        let mut dz = coord.z;

        while dz > 1
        {
            if let Some(id) = self.brick_at((coord.x, coord.y, dz - 1).into())
            {
                return (coord.z - dz, Some(id));
            }

            dz -= 1;
        }

        (coord.z - dz, None)
    }

    fn brick_at(&self, coord: Coord3d) -> Option<Id>
    {
        if let Some(id) = self.pos_cache.get(&coord) { return Some(*id); }
        else { return None; }

        // Look at the z axis and drill down on x
        // self.bricks.values()
        //     .find(|brick| {
        //         brick.min_z() <= coord.z && brick.max_z() >= coord.z &&
        //         brick.min_x() <= coord.x && brick.max_x() >= coord.x &&
        //         brick.min_y() <= coord.y && brick.max_y() >= coord.y
        //     })
        //     .map(|b| b.id)
    }

    pub fn render(&self)
    {
        let safe_bricks = self.safe_bricks_to_disintegrate();
        let mut window = Window::new("AoC - Day 22");

        // let mut c = window.add_cube(2.0, 1.0, 1.0);
        // c.set_color(1.0, 1.0, 1.0);
        // c.set_local_translation(Translation3::new(1.0 + 1.0, 0.5, 0.5));

        let max_x = self.bricks.values().map(|b| b.max_x()).max().unwrap() as f32;
        let max_y = self.bricks.values().map(|b| b.max_y()).max().unwrap() as f32;

        let mut camera = ArcBall::new(
            Point3::new(-5.0, 1.0, 0.0),
            Point3::new(max_x / 2.0, 1.0, max_y / 2.0)
        );

        for brick in self.bricks.values()
        {
            let mut c = window.add_cube(
                brick.size_x() as f32,
                brick.size_z() as f32,
                brick.size_y() as f32
            );

            let color = self.random_color();

            if safe_bricks.contains(&brick.id)
            {
                let green = self.random_greem();
                c.set_color(green.0, green.1, green.2);
            }
            else
            {
                c.set_color(color.0, 0.0, color.2);
            }

            c.set_lines_color(Some(Point3::new(color.0, 0.0, color.2)));

            // c.set_lines_color(Some(Point3::new(1.0, 0.0, 0.0)));
            c.set_local_translation(Translation3::new(
                0.5 * brick.size_x() as f32 + brick.min_x() as f32,
                0.5 * brick.size_z() as f32 + brick.min_z() as f32,
                0.5 * brick.size_y() as f32 + brick.min_y() as f32,
            ));
        }

        let origin = Point3::new(0.0, 0.0, 0.0);
        let x = Point3::new(1.0, 0.0, 0.0);
        let z = Point3::new(0.0, 3.0, 0.0);
        let y = Point3::new(0.0, 0.0, 1.0);

        window.set_light(Light::StickToCamera);

        while window.render_with_camera(&mut camera)
        {
            window.draw_line(&origin, &x, &Point3::new(1.0, 0.0, 0.0));
            window.draw_line(&origin, &y, &Point3::new(0.0, 1.0, 0.0));
            window.draw_line(&origin, &z, &Point3::new(0.0, 0.0, 1.0));
        }
    }

    fn random_greem(&self) -> (f32, f32, f32)
    {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0..128);
        let b = rng.gen_range(0..128);
        (r as f32 / 255.0, 1.0, b as f32 / 255.0)
    }

    fn random_color(&self) -> (f32, f32, f32)
    {
        let c = RandomColor::new().to_rgb_array().map(|v| (v as f32) / 255.0);
        (c[0], c[1], c[2])
    }
}

#[cfg(test)]
mod tests
{
    use crate::tower::Brick;

    use super::Tower;

    #[test]
    fn test_brick_size()
    {
        let b = Brick::new((1, 1, 1).into(), (1, 1, 1).into());
        assert_eq!(b.size(), 1);

        let b = Brick::new((0, 0, 1).into(), (0, 0, 10).into());
        assert_eq!(b.size(), 10);

        let b = Brick::new((1, 1, 1).into(), (2, 2, 2).into());
        assert_eq!(b.size(), 8)
    }

    #[test]
    fn test_tower_has_brick_at()
    {
        let t = Tower::from("1,1,1~5,5,2");

        assert!(t.brick_at((2, 2, 1).into()).is_some());
        assert!(t.brick_at((3, 3, 2).into()).is_some());
        assert!(t.brick_at((6, 6, 1).into()).is_none());
        assert!(t.brick_at((2, 2, 3).into()).is_none());
    }

    #[test]
    fn test_tower_dependencies()
    {
        let mut t = Tower::from(
"0,0,1~0,2,1
2,0,1~2,2,1
0,0,3~2,0,3
0,2,3~2,2,3"
        );

        t.apply_gravity();
        t.debug_info(&vec![]);

        let a = &t.bricks[&0];
        let b = &t.bricks[&1];
        let c = &t.bricks[&2];
        let d = &t.bricks[&3];

        assert!(a.bricks_above.contains(&c.id));
        assert!(a.bricks_above.contains(&d.id));
        assert!(b.bricks_above.contains(&c.id));
        assert!(b.bricks_above.contains(&d.id));

        assert!(c.on_bricks.contains(&a.id));
        assert!(c.on_bricks.contains(&b.id));
        assert!(d.on_bricks.contains(&a.id));
        assert!(d.on_bricks.contains(&b.id));
    }
}