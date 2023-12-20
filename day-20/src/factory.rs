use std::collections::{HashMap, VecDeque, HashSet};

use num::integer::lcm;

use crate::modules::{FlipFlop, Conjunction, Broadcaster, Module, ModuleKind};

const BUTTON: &str = "button";
const START: &str = "broadcaster";

pub struct Factory
{
    modules: HashMap<String, Box<dyn Module>>,
    bus: Bus
}

pub struct Bus
{
    queue: VecDeque<Message>,
    history: HashMap<MessageKind, usize>
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum MessageKind
{
    LowPulse,
    HighPulse
}

#[derive(Clone)]
pub struct Message
{
    pub kind: MessageKind,
    pub src: String,
    dest: Vec<String>
}

impl Message
{
    pub fn new(kind: MessageKind, src: String, dest: &Vec<String>) -> Self
    {
        Self { kind, dest: dest.clone(), src }
    }
}

impl From<&str> for Factory
{
    fn from(value: &str) -> Self
    {
        let modules = value.split("\n")
            .map(|line| {
                let mut parts: std::str::Split<'_, &str> = line.split(" -> ");

                let module_name_and_kind = parts.next().unwrap();
                let outputs = parts.next().unwrap().split(", ").map(|str| str.to_string()).collect();

                let kind = &module_name_and_kind[0..1];
                let name = &module_name_and_kind[1..];

                let module : Box<dyn Module> = match kind
                {
                    "%" => Box::new(FlipFlop::new(name, outputs)),
                    "&" => Box::new(Conjunction::new(name, outputs)),
                    _ =>   Box::new(Broadcaster::new(module_name_and_kind, outputs))
                };

                module
            })
            .map(|m| (m.name().to_string(), m))
            .collect();

        let mut factory = Factory { modules, bus: Bus::new() };

        factory.update_conjunction_modules_inputs();
        // TODO: for conjunction module we need to find all inputs for memory

        factory
    }
}

impl Factory
{
    pub fn run(&mut self, n_iterations: usize)
    {
        self.bus.reset();

        for _ in 0..n_iterations
        {
            // Send without counting message in history
            self.bus.send(Message::new(MessageKind::LowPulse, BUTTON.to_string(), &vec![START.to_string()]));
            self.run_once();
        }
    }

    pub fn run_until_low_rx(&mut self) -> usize
    {
        // Based on GraphViz analysis
        // let nand_before_rx = "ll";
        let counters = [
            ("zz", "ff"),
            ("lp", "tj"),
            ("fn", "th"),
            ("tp", "hb"),
        ];

        // All counters are flipped (through a NAND with 1 input)

        // Se wee need to search what make ll send LOW
        // -> All inputs at HIGH
        // -> Input at LOW (flipped NAND)
        // -> Counters inputs at HIGH

        // We need to isolate each cycle by removing output for NAND gates

        for (_, output) in counters
        {
            self.modules.get_mut(output).unwrap().clear_outputs();
        }

        let mut iterations = HashMap::new();

        // Now we run each branch to find the lower iteration were output is low
        for (input, output) in counters
        {
            verbose!("Finding cycle for {}", output);

            let mut n = 0;
            self.bus.reset();

            loop
            {
                n += 1;

                if n % 1000 == 0
                {
                    verbose!("[{}] {}", output, n);
                }

                self.bus.send(Message::new(MessageKind::LowPulse, BUTTON.to_string(), &vec![input.to_string()]));
                self.run_once();

                let output_module = self.modules[output].as_any().downcast_ref::<Conjunction>().unwrap();

                // Check if state for output is OK
                if output_module.state_all_high()
                {
                    verbose!("[{}] Found cycle at {}", output, n);
                    break;
                }
            }

            iterations.insert(output.to_string(), n);
        }

        let min_cycle = iterations.values().fold(1, |acc, c| lcm(acc, *c));

        return min_cycle;
    }

    fn run_once(&mut self)
    {
        debug!("--- Button pressed ---");
        while let Some(message) = self.bus.pop()
        {
            for dest in &message.dest
            {
                debug!("{}({:?}) [{:?}] -> {}({:?})",
                    message.src,
                    self.modules.get(&message.src).map_or(BUTTON, |m| m.name()),
                    message.kind,
                    dest,
                    self.modules.get(dest).map_or(ModuleKind::Unknown, |m| m.kind())
                );

                // Only process message for existing destinations
                if self.modules.contains_key(dest)
                {
                    self.modules.get_mut(dest).unwrap().run(&message, &mut self.bus);
                }
            }
        }

        debug!("--- Iteration done ---\n ");
    }

    pub fn low_pulses(&self) -> usize
    {
        self.bus.message_kind_count(MessageKind::LowPulse)
    }

    pub fn high_pulses(&self) -> usize
    {
        self.bus.message_kind_count(MessageKind::HighPulse)
    }

    fn update_conjunction_modules_inputs(&mut self)
    {
        let mut inputs: HashMap<String, HashSet<String>> = HashMap::new();

        for (name, module) in self.modules.iter()
        {
            for dest in module.outputs()
            {
                inputs.entry(dest.clone()).or_insert(HashSet::new()).insert(name.clone());
            }
        }

        // Prepopulate conjunction modules inputs (to have computation of "all low" done correctly and not only on node that already have communicated)
        for (name, module) in self.modules.iter_mut().filter(|(_, m)| m.kind() == ModuleKind::Conjunction)
        {
            let module = module.as_any_mut().downcast_mut::<Conjunction>().unwrap();

            module.set_inputs(
                &inputs.get(name).map_or(vec![], |set| {
                    set.iter().map(|str| str.to_string()).collect()
                })
            );
        }
    }
}

impl Bus
{
    pub fn new() -> Self
    {
        Bus { queue: VecDeque::new(), history: HashMap::new() }
    }

    pub fn reset(&mut self)
    {
        self.history.clear();
        self.queue.clear();
    }

    pub fn send(&mut self, message: Message)
    {
        *self.history.entry(message.kind).or_insert(0) += message.dest.len();

        self.queue.push_back(message);
    }

    pub fn pop(&mut self) -> Option<Message>
    {
        self.queue.pop_front()
    }

    pub fn message_kind_count(&self, kind: MessageKind) -> usize
    {
        *self.history.get(&kind).unwrap_or(&0)
    }
}