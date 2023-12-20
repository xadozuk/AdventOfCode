use std::{collections::HashMap, any::Any};

use crate::factory::{Bus, Message, MessageKind};

#[derive(Debug, PartialEq, Eq)]
pub enum ModuleKind
{
    FlipFlop,
    Conjunction,
    Broadcaster,
    Unknown
}

pub trait Module
{
    fn new(name: &str, outputs: Vec<String>) -> Self where Self : Sized;

    fn name(&self) -> &str;
    fn kind(&self) -> ModuleKind;

    fn outputs(&self) -> &Vec<String>;
    fn clear_outputs(&mut self);

    fn run(&mut self, message: &Message, bus: &mut Bus);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct FlipFlop
{
    name: String,
    outputs: Vec<String>,
    state: bool,
}

pub struct Conjunction
{
    name: String,
    outputs: Vec<String>,
    state: HashMap<String, MessageKind>,
}

impl Conjunction
{
    pub fn set_inputs(&mut self, inputs: &Vec<String>)
    {
        self.state.clear();

        for input in inputs
        {
            self.state.insert(input.to_string(), MessageKind::LowPulse);
        }
    }

    pub fn state_all_high(&self) -> bool
    {
        self.state.values().all(|m| *m == MessageKind::HighPulse)
    }
}

pub struct Broadcaster
{
    name: String,
    outputs: Vec<String>
}

impl Module for FlipFlop
{
    fn new(name: &str, outputs: Vec<String>) -> Self where Self : Sized
    {
        Self { name: name.to_string(), outputs, state: false }
    }

    fn name(&self) -> &str
    {
        &self.name
    }

    fn kind(&self) -> ModuleKind
    {
        ModuleKind::FlipFlop
    }

    fn outputs(&self) -> &Vec<String>
    {
        &self.outputs
    }

    fn run(&mut self, message: &Message, bus: &mut Bus)
    {
        if message.kind == MessageKind::LowPulse
        {
            self.state = !self.state;

            let message_kind = match self.state
            {
                true => MessageKind::HighPulse,
                false => MessageKind::LowPulse
            };

            bus.send(Message::new(message_kind, self.name().to_string(), &self.outputs))
        }
    }

    fn as_any(&self) -> &dyn Any
    {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any
    {
        self
    }

    fn clear_outputs(&mut self)
    {
        self.outputs.clear();
    }
}

impl Module for Conjunction
{
    fn new(name: &str, outputs: Vec<String>) -> Self where Self : Sized
    {
        Self { name: name.to_string(), outputs, state: HashMap::new() }
    }

    fn name(&self) -> &str
    {
        &self.name
    }

    fn kind(&self) -> ModuleKind
    {
        ModuleKind::Conjunction
    }

    fn outputs(&self) -> &Vec<String>
    {
        &self.outputs
    }

    fn run(&mut self, message: &Message, bus: &mut Bus)
    {
        *self.state.entry(message.src.clone()).or_insert(MessageKind::LowPulse) = message.kind;

        let message_kind =
            if self.state.values().all(|k| *k == MessageKind::HighPulse) { MessageKind::LowPulse }
            else { MessageKind::HighPulse };

        bus.send(Message::new(message_kind, self.name().to_string(), &self.outputs));
    }

    fn as_any(&self) -> &dyn Any
    {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any
    {
        self
    }

    fn clear_outputs(&mut self)
    {
        self.outputs.clear();
    }
}

impl Module for Broadcaster
{
    fn new(name: &str, outputs: Vec<String>) -> Self where Self : Sized
    {
        Self { name: name.to_string(), outputs }
    }

    fn name(&self) -> &str
    {
        &self.name
    }

    fn kind(&self) -> ModuleKind
    {
        ModuleKind::Broadcaster
    }

    fn outputs(&self) -> &Vec<String>
    {
        &self.outputs
    }

    fn run(&mut self, message: &Message, bus: &mut Bus)
    {
        bus.send(Message::new(message.kind, self.name().to_string(), &self.outputs));
    }

    fn as_any(&self) -> &dyn Any
    {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any
    {
        self
    }

    fn clear_outputs(&mut self)
    {
        self.outputs.clear();
    }
}