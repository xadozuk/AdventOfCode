use std::{collections::{HashMap, BinaryHeap, HashSet}, cmp::Reverse};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub type Coord = (usize, usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, EnumIter, Debug)]
pub enum Direction
{
    Up,
    Down,
    Left,
    Right
}

pub struct Factory
{
    matrix: Vec<Vec<u32>>,
    min: u32,
    max: u32
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
pub struct Node
{
    coord: Coord,
    direction: Option<Direction>,
    straight_steps_count: u32
}

impl Node
{
    pub fn new(coord: (usize, usize), direction: Option<Direction>, straight_steps_count: u32) -> Node
    {
        Node { coord, direction, straight_steps_count }
    }

    pub fn neighboors(&self, size: (usize, usize), min: u32, max: u32) -> Vec<Node>
    {
        let mut neighboors = vec![];

        // Initial node
        if self.direction.is_none()
        {
            // Iterate all directions
            for d in Direction::iter()
            {
                if let Some(coord) = self.coord_for(d, size)
                {
                    neighboors.push(Node::new(coord, Some(d), self.straight_steps_count + 1));
                }
            }

            // Return early
            return neighboors;
        }

        // Can we continue straight ?
        if self.straight_steps_count < max - 1
        {
            if let Some(coord) = self.coord_for(self.direction.unwrap(), size)
            {
                neighboors.push(Node::new(coord, self.direction, self.straight_steps_count + 1));
            }
        }

        // +1 to avoid -1 min (it can be 0, and panic for u32)
        if self.straight_steps_count + 1 >= min
        {
            // We turn perpendiculary (not forward nor backward)
            for d in Direction::iter().filter(|d| *d != self.direction.unwrap() && *d != self.opposite_direction())
            {
                if let Some(coord) = self.coord_for(d, size)
                {
                    neighboors.push(Node::new(coord, Some(d), 0));
                }
            }
        }

        neighboors
    }

    fn opposite_direction(&self) -> Direction
    {
        match self.direction.unwrap()
        {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }

    fn coord_for(&self, direction: Direction, max_size: (usize, usize)) -> Option<Coord>
    {
        if direction == Direction::Up && self.coord.0 == 0 ||
           direction == Direction::Down && self.coord.0 == max_size.0 - 1 ||
           direction == Direction::Left && self.coord.1 == 0 ||
           direction == Direction::Right && self.coord.1 == max_size.1 - 1
        {
            return None;
        }

        return Some(match direction
        {
            Direction::Up => (self.coord.0 - 1, self.coord.1),
            Direction::Down => (self.coord.0 + 1, self.coord.1),
            Direction::Left => (self.coord.0, self.coord.1 - 1),
            Direction::Right => (self.coord.0, self.coord.1 + 1)
        })
    }
}

impl From<&str> for Factory
{
    fn from(value: &str) -> Self
    {
        let matrix = value.split('\n')
            .map(|row| {
                row.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();

        Factory { matrix, min: 0, max: 3 }
    }
}

impl Factory
{
    pub fn new(min: u32, max: u32, content: &str) -> Self
    {
        let mut factory = Factory::from(content);

        factory.min = min;
        factory.max = max;

        factory
    }

    pub fn find_lesser_heat_loss(&self, start: Node) -> u32
    {
        let path = dijkstra(self, start);

        path.last().unwrap().1
    }

    pub fn size(&self) -> (usize, usize)
    {
        (self.matrix.len(), self.matrix[0].len())
    }

    fn end_coord(&self) -> Coord
    {
        let (height, width) = self.size();
        (height - 1, width - 1)
    }
}

fn dijkstra(factory: &Factory, start: Node) -> Vec<(Node, u32)>
{
    let mut dist: HashMap<Node, u32> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(u32, Node)>> = BinaryHeap::new();
    let mut visited: HashSet<Node> = HashSet::new();
    let mut previous: HashMap<Node, Node> = HashMap::new();

    let end_coord = factory.end_coord();

    dist.insert(start, 0);
    queue.push(Reverse((0, start)));

    while let Some(Reverse((_, node))) = queue.pop()
    {
        debug!("PF: + Visiting node {:?}", node);

        visited.insert(node);

        for neighboor in node.neighboors(factory.size(), factory.min, factory.max)
        {
            // Avoid loop and backtracking
            if visited.contains(&neighboor) { continue }

            debug!("PF: |-- Neighboor: {:?}", neighboor);

            let node_dist = *dist.get(&node).unwrap_or(&u32::MAX);
            let neighboor_dist = *dist.get(&neighboor).unwrap_or(&u32::MAX);
            let alt_dist = node_dist + factory.matrix[neighboor.coord.0][neighboor.coord.1];

            // TODO: if we are on the end node, we need to check that we are at the min steps

            if alt_dist < neighboor_dist
            {
                debug!("PF: |--- Found shortest path: {} ({})", alt_dist, neighboor_dist);

                // +1 to avoid substract overflow (min can be 0)
                if neighboor.coord != end_coord || neighboor.straight_steps_count + 1 >= factory.min
                {
                    debug!("PF: |---- Possible path, saving...");

                    if neighboor.coord == end_coord { debug!("PF: |----- Found path to the end"); }

                    dist.insert(neighboor, alt_dist);
                    queue.push(Reverse((alt_dist, neighboor)));
                    previous.insert(neighboor, node);
                }
            }
        }
    }

    let end_coord = factory.end_coord();
    let end_node = dist.iter()
        .filter(|(node, _)| {
            node.coord == end_coord
        })
        .min_by(|a, b| a.1.cmp(b.1))
        .unwrap();

    unfold_dijkstra(*end_node.0, start, &previous, &dist)
}

fn unfold_dijkstra(end: Node, start: Node, previous: &HashMap<Node, Node>, dist: &HashMap<Node, u32>) -> Vec<(Node, u32)>
{
    debug!("UNFOLD: -->");

    if end.coord == start.coord
    {
        debug!("UNFOLD: Found start returning...");
        return vec![(end, dist[&end])]
    }

    let previous_node = previous[&end];

    debug!("UNFOLD: Current node:  {:?}", end);
    debug!("UNFOLD: Previous node: {:?}", previous_node);

    let mut path = unfold_dijkstra(previous_node, start, previous, dist);
    path.push((end, dist[&end]));

    path
}

#[cfg(test)]
mod tests
{
    use super::{Direction, Node};

    #[test]
    fn test_node_coord_for()
    {
        let node = Node::new((1, 0), None, 0);

        let neighboors = node.neighboors((3, 3), 0, 3);

        assert_eq!(neighboors.len(), 3);

        assert!(neighboors.contains(&Node::new((1, 1), Some(Direction::Right), 1)));
        assert!(neighboors.contains(&Node::new((0, 0), Some(Direction::Up), 1)));
        assert!(neighboors.contains(&Node::new((2, 0), Some(Direction::Down), 1)));

        let node = Node::new((0, 0), Some(Direction::Right), 2);
        let neighboors = node.neighboors((2, 2), 0, 3);

        assert_eq!(neighboors.len(), 1);

        assert!(neighboors.contains(&Node::new((1, 0), Some(Direction::Down), 0)));
    }
}