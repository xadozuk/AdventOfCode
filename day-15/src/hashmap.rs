use std::{collections::HashMap, fmt::Debug};

use crate::hash;

#[derive(PartialEq, Eq, Debug)]
enum Op
{
    Set,
    Remove
}

#[derive(Clone)]
struct Lens
{
    label: String,
    focal: u32
}

#[derive(Clone)]
struct Box
{
    lenses: Vec<Lens>,
    indexes: HashMap<String, usize>
}

pub struct Factory
{
    boxes: [Box; 256]
}

impl Factory
{
    pub fn new() -> Self
    {
        Factory {
            boxes: [(); 256].map(|_| Box::new())
        }
    }

    pub fn run(&mut self, instructions: &str)
    {
        for inst in instructions.split(',')
        {
            let (label, op, focal) = self.parse_inst(inst);
            let target_box = hash(&label) as usize;

            debug!("Instruction: {}", inst);
            debug!("Box: {}, Label: {}, Op: {:?}, Focal: {:?})", target_box, label, op, focal);

            self.run_one(target_box, label, op, focal);

            debug!("\n{:?}", self);
        }
    }

    fn parse_inst(&self, inst: &str) -> (String, Op, Option<u32>)
    {
        let label = inst.chars().take_while(|c| *c != '=' && *c != '-').collect();
        let op = if inst.contains('=') { Op::Set }
                     else { Op::Remove };

        let focal = if op == Op::Set
        {
            inst.split('=').nth(1).unwrap().parse::<u32>().ok()
        }
        else
        {
            None
        };

        (label, op, focal)
    }

    fn run_one(&mut self, target_box: usize, label: String, op: Op, focal: Option<u32>)
    {
        match op
        {
            Op::Remove => self.boxes[target_box].remove(&label),
            Op::Set => {
                if self.boxes[target_box].contains(&label)
                {
                    self.boxes[target_box].replace(&label, Lens { label: label.clone(), focal: focal.unwrap() })
                }
                else
                {
                    self.boxes[target_box].push(Lens { label: label.clone(), focal: focal.unwrap() })
                }
            }
        }
    }

    pub fn focusing_power(&self) -> u32
    {
        self.boxes.iter().enumerate().fold(0, |acc, (i, target_box)| {
            acc + (1 + i as u32) * target_box.focusing_power()
        })
    }
}

impl Debug for Factory
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_str("---\n")?;

        for (i, target_box) in self.boxes.iter().enumerate().filter(|b| !b.1.lenses.is_empty())
        {
            f.write_fmt(format_args!("Box {}: {:?}\n", i, target_box))?;
        }

        Ok(())
    }
}

impl Box
{
    pub fn new() -> Self
    {
        Box {
            lenses: vec![],
            indexes: HashMap::new()
        }
    }

    fn contains(&self, label: &String) -> bool
    {
        self.indexes.contains_key(label)
    }

    fn push(&mut self, lens: Lens)
    {
        let label = lens.label.clone();

        self.lenses.push(lens);
        self.indexes.insert(label, self.lenses.len() - 1);
    }

    fn replace(&mut self, label: &String, lens: Lens)
    {
        if !self.indexes.contains_key(label)
        {
            panic!("Box does not contains lens '{}'", label);
        }

        let index = self.indexes[label];
        self.lenses[index] = lens;
    }

    fn remove(&mut self, label: &String)
    {
        if !self.indexes.contains_key(label) { return }

        let index = self.indexes[label];

        self.lenses.remove(index);
        self.indexes.remove(label);

        let indexes_to_update: Vec<_> = self.indexes.iter()
            .filter(|(_, i)| **i > index)
            .map(|(k, _)| k.clone())
            .collect();

        for l in indexes_to_update
        {
            self.indexes.entry(l).and_modify(|e| *e -= 1);
        }
    }

    pub fn focusing_power(&self) -> u32
    {
        self.lenses.iter().enumerate().fold(0, |acc, (i, lens)| {
            acc + (1 + i as u32) * lens.focal
        })
    }
}

impl Debug for Box
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        for lens in self.lenses.iter()
        {
            f.write_fmt(format_args!("[{} {}] ", lens.label, lens.focal))?;
        }

        Ok(())
    }
}