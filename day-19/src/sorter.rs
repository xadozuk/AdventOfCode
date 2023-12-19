use std::{collections::HashMap, ops::Range};

use crate::ranges::MultiRange;

pub const START: &str = "in";
pub const MIN: u32 = 1;
pub const MAX: u32 = 4001;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation
{
    Jump,
    Accept,
    Reject
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Condition
{
    GreaterThan,
    LesserThan,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Target
{
    X,
    M,
    A,
    S
}

pub struct Sorter
{
    workflows: HashMap<String, Workflow>,
    input_parts: Vec<Part>,
}

#[derive(Debug)]
pub struct Workflow
{
    name: String,
    rules: Vec<Rule>
}

#[derive(Debug, Clone)]
pub struct Rule
{
    op: Operation,
    condition: Option<Condition>,
    target: Option<Target>,
    value: Option<u32>,
    jump_to: Option<String>
}

#[derive(Debug, Clone)]
pub struct Part
{
    values: HashMap<Target, u32>
}

#[derive(Debug, Clone)]
pub struct PartRange
{
    values: HashMap<Target, MultiRange<u32>>
}

impl From<&str> for Sorter
{
    fn from(value: &str) -> Self
    {
        let mut parts = value.split("\n\n");

        let workflows = parts.next().unwrap().split("\n")
            .map(|w| Workflow::from(w))
            .map(|w| (w.name.clone(), w))
            .collect();

        let parts = parts.next().unwrap().split("\n")
            .map(|p| Part::from(p))
            .collect();

        Sorter { workflows, input_parts: parts }
    }
}

impl From<&str> for Workflow
{
    fn from(value: &str) -> Self
    {
        let mut parts = value[..value.len() - 1].split("{");
        let name = parts.next().unwrap().to_string();

        let rules = parts.next().unwrap().split(",")
            .map(|r| Rule::from(r))
            .collect();

        Workflow { name, rules }
    }
}

impl From<&str> for Rule
{
    fn from(value: &str) -> Self
    {
        // With condition
        if value.contains(":")
        {
            let mut parts = value.split(":");
            let condition = parts.next().unwrap();
            let result = parts.next().unwrap();

            let condition_op = if condition.contains("<")      { Condition::LesserThan }
                                   else if condition.contains(">") { Condition::GreaterThan }
                                   else { panic!("Unknown condition {}", condition) };

            let target = Target::from(condition.chars().take_while(|c| *c != '<' && *c != '>').collect::<String>().as_str());
            let value = condition.chars()
                .skip_while(|c| *c != '<' && *c != '>')
                .skip(1)
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            match result
            {
                "A" => Rule::accept_if(condition_op, target, value),
                "R" => Rule::reject_if(condition_op, target, value),
                jump_target => Rule::jump_if(condition_op, target, value, jump_target.to_string())
            }
        }
        else
        {
            match value
            {
                "A" => Rule::accept(),
                "R" => Rule::reject(),
                jump_target => Rule::jump(jump_target.to_string())
            }
        }
    }
}

impl From<&str> for Part
{
    fn from(value: &str) -> Self
    {
        let mut values = HashMap::new();

        // Strip enclosing bracket
        let parts = value[1..value.len() - 1].split(",");

        for part_ranking in parts
        {
            let mut p = part_ranking.split('=');
            let target = p.next().unwrap();
            let value = p.next().unwrap().parse::<u32>().unwrap();

            let target = Target::from(target);

            values.insert(target, value);
        }

        Part { values }
    }
}

impl From<&str> for Target
{
    fn from(value: &str) -> Self
    {
        match value
        {
            "x" => Target::X,
            "m" => Target::M,
            "a" => Target::A,
            "s" => Target::S,
            default => panic!("Unknown target {}", default)
        }
    }
}

impl Rule
{
    fn accept() -> Rule
    {
        Rule { op: Operation::Accept, jump_to: None, target: None, value: None, condition: None }
    }

    fn reject() -> Rule
    {
        Rule { op: Operation::Reject, jump_to: None, target: None, value: None, condition: None }
    }

    fn jump(jump_to: String) -> Rule
    {
        Rule { op: Operation::Jump, jump_to: Some(jump_to), target: None, value: None, condition: None }
    }

    fn accept_if(condition: Condition, target: Target, value: u32) -> Rule
    {
        Rule
        {
            op: Operation::Accept,
            condition: Some(condition),
            target: Some(target),
            value: Some(value),
            jump_to: None
        }
    }

    fn reject_if(condition: Condition, target: Target, value: u32) -> Rule
    {
        Rule
        {
            op: Operation::Reject,
            condition: Some(condition),
            target: Some(target),
            value: Some(value),
            jump_to: None
        }
    }

    fn jump_if(condition: Condition, target: Target, value: u32, jump_to: String) -> Rule
    {
        Rule
        {
            op: Operation::Jump,
            condition: Some(condition),
            target: Some(target),
            value: Some(value),
            jump_to: Some(jump_to)
        }
    }

    fn is_match(&self, part: &Part) -> bool
    {
        if let Some(c) = &self.condition
        {
            let t = self.target.unwrap();
            let v = self.value.unwrap();

            return *c == Condition::LesserThan && part.values[&t] < v ||
                   *c == Condition::GreaterThan && part.values[&t] > v;
        }
        else
        {
            return true;
        }
    }

    fn condition(&self) -> Option<(Condition, Target, u32)>
    {
        if self.condition.is_some()
        {
            return Some((
                self.condition.unwrap(),
                self.target.unwrap(),
                self.value.unwrap()
            ));
        }

        None
    }
}

impl Sorter
{
    pub fn run(&self) -> u32
    {
        let mut accepted_parts = vec![];

        for part in self.input_parts.iter()
        {
            if let Some(part) = self.run_part(&part, &self.workflows[START])
            {
                accepted_parts.push(part)
            }
        }

        accepted_parts.iter().map(|p| p.value()).sum()
    }

    fn run_part(&self, part: &Part, starting_workflow: &Workflow) -> Option<Part>
    {
        let mut workflow = starting_workflow;

        loop
        {
            if let Some(rule) = workflow.match_rule(part)
            {
                match rule.op
                {
                    Operation::Accept => return Some(part.clone()),
                    Operation::Reject => return None,
                    Operation::Jump => workflow = &self.workflows[&rule.jump_to.unwrap()]
                }
            }
            else
            {
                panic!("Workflow terminated without result: {:?}, {:?}", workflow, part);
            }
        }
    }

    pub fn accepted_part_combinations(&self) -> u64
    {
        let initial_part_range = PartRange::new(MIN..MAX);
        let combinations = self._accepted_part_combinations(&self.workflows[START], initial_part_range);

        combinations.iter().map(|pr| pr.value()).sum()
    }

    fn _accepted_part_combinations(&self, workflow: &Workflow, part_range: PartRange) -> Vec<PartRange>
    {
        let mut ranges = vec![];
        let mut part_range = part_range;

        // If workflow can only reject, no need to check condition
        if workflow.is_reject_all() { return ranges; }

        for rule in workflow.rules.iter()
        {
            match (rule.op, rule.condition())
            {
                (Operation::Accept, None) =>
                {
                    // If we accept all without condition
                    // Add current range to possibility and break, we don't need to continue
                    ranges.push(part_range);
                    break;
                },
                (Operation::Accept, Some((condition, target, value))) =>
                {
                    // If we accept with a condition we need to add the possibility and remove it from active ranges
                    let incl_range = self.ranges_for_condition(condition, value);

                    let mut valid_part_range = part_range.clone();
                    valid_part_range.values.get_mut(&target).unwrap().intersect(&incl_range);

                    ranges.push(valid_part_range);

                    let current_target_range = part_range.values.get_mut(&target).unwrap();
                    current_target_range.exclude(&incl_range);
                }
                // If there is no condition, it means all the remaining range is excluded -> break
                (Operation::Reject, None) => break,
                (Operation::Reject, Some((condition, target, value))) =>
                {
                    let current_target_range = part_range.values.get_mut(&target).unwrap();
                    let incl_range = self.ranges_for_condition(condition, value);

                    // We remove excl_range from active range
                    current_target_range.exclude(&incl_range);
                },
                (Operation::Jump, None) =>
                {
                    let jump_to = rule.jump_to.as_ref().unwrap();

                    // If there is no condition, we pass active ranges as-is
                    let mut valid_part_ranges = self._accepted_part_combinations(&self.workflows[jump_to], part_range.clone());
                    ranges.append(&mut valid_part_ranges);
                }
                (Operation::Jump, Some((condition, target, value))) =>
                {
                    // If we have a condition, we provide a restricted part range to recursion and we remove it from active ranges
                    let jump_to = rule.jump_to.as_ref().unwrap();
                    let incl_range = self.ranges_for_condition(condition, value);

                    let mut jump_part_range = part_range.clone();
                    jump_part_range.intersect_target(target, &incl_range);

                    let mut valid_part_ranges = self._accepted_part_combinations(
                        &self.workflows[jump_to],
                        jump_part_range
                    );

                    ranges.append(&mut valid_part_ranges);

                    let current_target_range = part_range.values.get_mut(&target).unwrap();
                    current_target_range.exclude(&incl_range);
                }
            }
        }

        ranges
    }

    fn ranges_for_condition(&self, condition: Condition, value: u32) -> Range<u32>
    {
        match condition
        {
            Condition::LesserThan => MIN..value,
            Condition::GreaterThan => value + 1..MAX
        }
    }
}

impl Part
{
    pub fn value(&self) -> u32
    {
        self.values.values().sum()
    }
}

impl Workflow
{
    fn match_rule(&self, part: &Part) -> Option<Rule>
    {
        for rule in &self.rules
        {
            if rule.is_match(part)
            {
                return Some(rule.clone());
            }
        }

        None
    }

    fn is_reject_all(&self) -> bool
    {
        self.rules.iter().all(|r| r.op == Operation::Reject)
    }
}

impl PartRange
{
    fn new(range: std::ops::Range<u32>) -> Self
    {
        PartRange {
            values: [
                (Target::X, MultiRange::from_range(range.clone())),
                (Target::S, MultiRange::from_range(range.clone())),
                (Target::A, MultiRange::from_range(range.clone())),
                (Target::M, MultiRange::from_range(range.clone()))
            ].into()
        }
    }

    fn value(&self) -> u64
    {
        self.values.values().fold(1, |acc, v| {
            acc * v.len() as u64
        })
    }

    fn intersect_target(&mut self, target: Target, incl_range: &Range<u32>)
    {
        let range = self.values.get_mut(&target).unwrap();
        range.intersect(&incl_range);
    }
}