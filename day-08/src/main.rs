use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
use num::integer::lcm;

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";

enum Instruction {
    Left,
    Right
}

#[derive(PartialEq, Eq, Hash)]
struct Node(String);

impl Node
{
    pub fn is_start(&self) -> bool
    {
        self.0.ends_with('A')
    }

    pub fn is_end(&self) -> bool
    {
        self.0.ends_with('Z')
    }
}

fn main()
{
    let file = File::open("./day-08/input.txt").unwrap();
    let buffer = BufReader::new(file);

    let (instructions, nodes) = parse(buffer);

    let result = result_part_1(&instructions, &nodes);
    println!("Result: {0}", result);

    let result2 = result_part_2(&instructions, &nodes);
    println!("Result2: {0}", result2);
}

fn result_part_1(instructions: &Vec<Instruction>, nodes: &HashMap<Node, (Node, Node)>) -> u32
{
    let mut step: u32 = 0;
    let mut current_node = &Node(START_NODE.to_string());

    for instruction in instructions.iter().cycle()
    {
        let node = nodes.get(current_node).unwrap();

        current_node = match instruction
        {
            Instruction::Left => &node.0,
            Instruction::Right => &node.1
        };

        step += 1;

        if current_node == &Node(END_NODE.to_string())
        {
            break;
        }
    }

    step
}

fn result_part_2(instructions: &Vec<Instruction>, nodes: &HashMap<Node, (Node, Node)>) -> u64
{
    let mut smallest_steps : Vec<u64> = vec![];

    let starting_nodes : Vec<&Node> = nodes.keys()
        .filter(|n| n.is_start())
        .collect();

    for starting_node in starting_nodes
    {
        let mut step: u64 = 0;
        let mut current_node = starting_node;

        for instruction in instructions.iter().cycle()
        {
            let node = nodes.get(current_node).unwrap();

            current_node = match instruction
            {
                Instruction::Left => &node.0,
                Instruction::Right => &node.1
            };

            step += 1;

            if current_node.is_end()
            {
                break;
            }
        }

        smallest_steps.push(step);
    }

    let init = smallest_steps[0].clone() as u64;
    let lcm = smallest_steps.into_iter().fold(init, |acc, step| lcm(acc, step));

    return lcm;
}

fn parse(buffer: BufReader<File>) -> (Vec<Instruction>, HashMap<Node, (Node, Node)>)
{
    let mut lines    = buffer.lines();
    let instructions = lines.next().unwrap().unwrap();

    // Skip empty line
    lines.next();

    // Parse map
    let mut nodes = HashMap::new();

    for line in lines
    {
        match line
        {
            Ok(content) =>
            {
                let (start, left, right) = parse_node(content);
                nodes.insert(start, (left, right));
            }
            Err(e) => panic!("Error while reading file: {0}", e)
        }
    }

    (
        instructions.chars().map(|c| if c == 'L' { Instruction::Left } else { Instruction::Right }).collect(),
        nodes
    )
}

fn parse_node(content: String) -> (Node, Node, Node)
{
    let mut parts = content.split('=').map(|p| p.trim());
    let start = parts.next().unwrap();

    let mut remaining = parts.next().unwrap().split(',')
        .map(|p| p.trim_matches('(').trim_matches(')').trim());

    let left = remaining.next().unwrap();
    let right = remaining.next().unwrap();

    return (Node(start.to_string()), Node(left.to_string()), Node(right.to_string()));
}