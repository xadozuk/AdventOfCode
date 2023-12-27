use std::{collections::{HashMap, HashSet, VecDeque}, fs::File, io::Write};

type Coord = (usize, usize);

pub struct Graph
{
    nodes: HashMap<Coord, Node>,
    edges: HashMap<Coord, HashSet<Edge>>
}

pub struct Node
{
    coord: Coord
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Edge
{
    from: Coord,
    to: Coord,
    distance: usize
}

pub struct Path
{
    distance: usize,
    coords: Vec<Coord>
}

impl Path
{
    pub fn new(distance: usize, coord: Coord) -> Self
    {
        Path {
            distance,
            coords: vec![coord]
        }
    }

    fn add_step(&mut self, distance: usize, coord: Coord)
    {
        self.distance += distance;
        self.coords.push(coord);
    }
}

impl Graph
{
    pub fn new() -> Self
    {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn reset(&mut self)
    {
        self.nodes.clear();
        self.edges.clear();
    }

    pub fn new_node(&mut self, coord: Coord)
    {
        self.nodes.insert(coord, Node { coord });
    }

    pub fn connect_nodes(&mut self, from: Coord, to: Coord, distance: usize)
    {
        if !self.nodes.contains_key(&from)
        {
            panic!("Node at {:?} doesn't exist", from);
        }

        self.edges.entry(from).or_insert(HashSet::new()).insert(Edge {
            from: from,
            to: to,
            distance
        });
    }

    pub fn distances(&self, start: Coord, end: Coord) -> Vec<usize>
    {
        // Optimization: find the closest node to end that have more than 1 edge
        // (because it is a one-way road and if we don't go to the end, we block the road)

        let (virtual_end, distance_to_end) = self.virtual_end(end);

        let paths = self.recurse_walk(start, virtual_end, &mut HashSet::new());

        paths.map_or_else(
            || vec![],
            |paths| paths.iter().map(|p| p.distance + distance_to_end).collect()
        )
    }

    fn virtual_end(&self, end: Coord) -> (Coord, usize)
    {
        let mut end = end;
        let mut from = end;
        let mut distance = 0;

        loop
        {
            let edges = self.edges.get(&end);
            if edges.is_none() { break; }

            let edges: Vec<_> = edges.unwrap()
                .iter()
                .filter(|e| e.to != from)
                .collect();

            if edges.len() == 1
            {
                from = edges[0].from;
                end = edges[0].to;
                distance += edges[0].distance;
            }
            else
            {
                break;
            }
        }

        (end, distance)
    }

    fn recurse_walk(&self, start: Coord, end: Coord, visited: &mut HashSet<Coord>) -> Option<Vec<Path>>
    {
        if visited.contains(&start)
        {
            debug!("[GRAPH WALK] Already visited");
            return None;
        }

        if end == start
        {
            debug!("[GRAPH WALK] Hit END");
            return Some(vec![Path::new(0, start)]);
            // Done
        }

        let edges = self.edges.get(&start);

        // Dead end
        if edges.is_none() { return None; }

        visited.insert(start);

        let edges = edges.unwrap();
        let mut results = vec![];

        for edge in edges
        {
            let next_node = self.nodes.get(&edge.to).unwrap();

            debug!("[GRAPH WALK] recurse: {:?} -> {:?} [{}]", edge.from, edge.to, edge.distance);

            if let Some(mut paths) = self.recurse_walk(next_node.coord, end, &mut visited.clone())
            {
                paths.iter_mut().for_each(|path| path.add_step(edge.distance, start));
                results.append(&mut paths);
            }
        }

        Some(results)
    }

    pub fn contains(&self, coord: Coord) -> bool
    {
        self.nodes.contains_key(&coord)
    }

    pub fn debug(&self, start: Coord)
    {
        // Assume starting node is the first in the list
        println!("Start: {:?}", start);
        self.debug_node(self.nodes[&start].coord, 1, &mut HashSet::new());
    }

    fn debug_node(&self, coord: Coord, depth: usize, mut drawn: &mut HashSet<Edge>)
    {
        let edges = &self.edges.get(&coord);

        if edges.is_none()
        {
            println!("{:width$} #", "", width = depth * 4);
            return;
        }

        // println!("{} {:?}", prefix, coord);

        for e in edges.unwrap_or(&HashSet::new())
        {
            println!("{:width$}[{}] -> {:?}", "", e.distance, e.to, width = depth * 4);

            if !drawn.contains(e)
            {
                drawn.insert(e.clone());
                self.debug_node(e.to, depth + 1, &mut drawn);
            }
        }
    }

    pub fn to_mermaid_chart(&self, path: &str, start: Coord) -> Result<(), std::io::Error>
    {
        let mut file = File::create(path)?;
        file.write(b"flowchart TD\n")?;

        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut visited = HashSet::new();

        let mut index = HashMap::new();

        for (i, (coord, _)) in self.nodes.iter().enumerate()
        {
            file.write_fmt(format_args!("\t{}[\"{} {:?}\"]\n", i, i, coord))?;
            index.insert(coord, i);
        }

        file.write(b"\n")?;

        while let Some(coord) = queue.pop_front()
        {
            if let Some(edges) = self.edges.get(&coord)
            {
                for e in edges
                {
                    if visited.contains(&e) { continue; }

                    let mut dir = "-->";
                    if let Some(reverse_edges) = self.edges.get(&e.to)
                    {
                        if let Some(re) = reverse_edges.iter().find(|re| re.to == e.from)
                        {
                            visited.insert(re);
                            dir = "<-->";
                        }
                    }

                    visited.insert(e);
                    file.write_fmt(format_args!("\t{} {} |{}| {}\n", index[&e.from], dir, e.distance, index[&e.to]))?;

                    queue.push_back(e.to);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests
{
    use crate::graph::Graph;

    #[test]
    fn test_node_sum_distance()
    {
        let mut g = Graph::new();

        let n1 = (0, 0);
        let n2 = (1, 0);
        let n3 = (2, 0);
        let n4 = (2, 2);

        g.new_node(n1);
        g.new_node(n2);
        g.new_node(n3);
        g.new_node(n4);

        g.connect_nodes(n1, n2, 10);
        g.connect_nodes(n2, n3, 15);
        g.connect_nodes(n3, n4, 10);

        g.connect_nodes(n1, n3, 5);
        g.connect_nodes(n3, n4, 10);

        let distances = g.distances(n1, n4);

        assert!(distances.contains(&35));
        assert!(distances.contains(&15));
    }
}