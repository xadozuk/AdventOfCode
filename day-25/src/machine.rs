use std::{collections::{HashMap, HashSet, VecDeque, BinaryHeap}, fs::File, io::Write, path, rc::Rc, cmp::Reverse};

use rand::seq::IteratorRandom;

pub type Id = Rc<String>;

pub const MAX_EDGE_CAPACITY: usize = 1;

pub struct Machine
{
    nodes: HashSet<Id>,
    edges: HashMap<Id, HashMap<Id, Edge>>,
}

pub struct Edge
{
    from: Id,
    to:   Id,

    capacity: usize,
    flow: usize
}

impl From<&str> for Machine
{
    fn from(value: &str) -> Self
    {
        let mut nodes = HashSet::new();
        let mut edges = HashMap::new();

        for line in value.split("\n")
        {
            let mut parts = line.split(": ");
            let left = parts.next().unwrap();
            let right: Vec<_> = parts.next().unwrap().split(" ").map(|s| s.to_string()).collect();

            let left_id = Rc::new(left.to_string());

            nodes.insert(left_id.clone());

            for to in right
            {
                let right_id = Rc::new(to.to_string());
                nodes.insert(right_id.clone());

                edges.entry(left_id.clone())
                    .or_insert(HashMap::new())
                    .insert(right_id.clone(), Edge::new(left_id.clone(), right_id.clone()));

                // Add reverse edge as well
                edges.entry(right_id.clone())
                    .or_insert(HashMap::new())
                    .insert(left_id.clone(), Edge::new(right_id.clone(), left_id.clone()));
            }
        }

        Machine { nodes, edges }
    }
}

impl Machine
{
    pub fn find_split(&mut self) -> (usize, usize)
    {
        // Find source / sink
        // Take node with most input/output
        // Compute shortest path between them, take couple with longest path

        // Min-cut / Max-flow
        // Build residual graph
        // Explore from source to form on set, remaining nodes form second set
        // Maybe ? Compute cut links ?

        let (source, sink) = self.find_source_and_sink();

        verbose!("Source: {}, Sink: {}", source, sink);

        self.saturate_flow(source.clone(), sink);

        self.components_size(source)
    }

    fn find_source_and_sink(&self) -> (Id, Id)
    {
        verbose!("Finding a random source and sink...");

        let mut edges_by_io: Vec<_> = self.edges.iter()
            .map(|(id, next)| (id, next.len()))
            .collect();

        edges_by_io.sort_by_key(|(_, n)| *n);

        let mut source_sink;

        loop
        {
            source_sink = edges_by_io.iter().rev().take(100).choose_multiple(&mut rand::thread_rng(), 2);

            // Avoid neighboor nodes
            if !self.edges[source_sink[0].0].contains_key(source_sink[1].0)
            {
                break;
            }
        }

        (
            source_sink[0].0.clone(),
            source_sink[1].0.clone()
        )
    }

    fn saturate_flow(&mut self, source: Id, sink: Id)
    {
        verbose!("Saturating flow: {} -> {}", source, sink);

        loop
        {
            if let Some(path) = self.find_path(source.clone(), sink.clone())
            {
                debugln!("Found path through: {:?}", path);

                // For each edge in the path we need to update flow/capacity
                for (from, to) in path
                {
                    let edge = self.edges.get_mut(&from).unwrap().get_mut(&to).unwrap();
                    edge.flow = 1;

                    let reverse_edge = self.edges.get_mut(&to).unwrap().get_mut(&from).unwrap();
                    reverse_edge.flow = 0;
                }
            }
            else
            {
                debugln!("No path found, graph is saturated !");
                break;
            }
        }
    }

    fn components_size(&self, source: Id) -> (usize, usize)
    {
        // Graph must be saturated

        // Collect all reachables nodes from source
        let mut left_components = vec![];

        let mut queue = VecDeque::new();
        queue.push_back(source);

        while let Some(id) = queue.pop_front()
        {
            if left_components.contains(&id) { continue; }
            left_components.push(id.clone());

            for (next_id, edge) in &self.edges[&id]
            {
                if edge.flow == 1 { continue; }
                queue.push_back(next_id.clone());
            }
        }

        let left_n_components = left_components.len();
        let right_n_components = self.nodes.len() - left_n_components;

        (
            left_n_components.max(right_n_components),
            left_n_components.min(right_n_components)
        )
    }

    fn find_path(&self, start: Id, end: Id) -> Option<Vec<(Id, Id)>>
    {
        let mut queue: BinaryHeap<Reverse<(usize, Id)>> = BinaryHeap::new();
        let mut dist: HashMap<Id, usize> = HashMap::new();
        let mut previous: HashMap<Id, Id> = HashMap::new();
        let mut visited = HashSet::new();

        queue.push(Reverse((0, start.clone())));
        dist.insert(start.clone(), 0);

        while let Some(Reverse((_, id))) = queue.pop()
        {
            if visited.contains(&id) { continue; }

            debugln!("[DIJKSTRA] @ {}", id);

            visited.insert(id.clone());

            for (next_id, edge) in &self.edges[&id]
            {
                if edge.flow == 1 { continue; }

                debug!("[DIJKSTRA]\t-> {}", next_id);

                let alt_dist = dist.get(&id).unwrap_or(&usize::MAX) + 1;

                if alt_dist <= *dist.get(next_id).unwrap_or(&usize::MAX)
                {
                    debug_raw!(" !");

                    *dist.entry(next_id.clone()).or_insert(usize::MAX) = alt_dist;
                    previous.insert(next_id.clone(), id.clone());

                    queue.push(Reverse((alt_dist, next_id.clone())));
                }

                debug_raw!("\n");
            }
        }

        if !previous.contains_key(&end) { return None; }

        let mut current = end;
        let mut path = vec![];

        while current != start
        {
            let p = previous[&current].clone();

            path.insert(0, (p.clone(), current));
            current = p;
        }

        Some(path)
    }
}

impl Edge
{
    pub fn new(from: Id, to: Id) -> Self
    {
        Edge { from, to, capacity: MAX_EDGE_CAPACITY, flow: 0 }
    }
}