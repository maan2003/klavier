mod if_held;
mod magic_shift;
mod mod_or_key;
mod remaper;

use evdev::{EventType, InputEvent, Key};
use std::{collections::VecDeque, io};

pub use {if_held::if_held, magic_shift::magic_shift, mod_or_key::mod_or_key, remaper::*};

pub trait Rule {
    // returns true if the event was handled
    fn event(&mut self, ctx: &mut RuleCtx, event: &InputEvent) -> io::Result<()>;
}

#[derive(Debug)]
pub struct RuleCtx<'a> {
    pub(crate) events: &'a mut VecDeque<InputEvent>,
}

impl<'a> RuleCtx<'a> {
    /// We will only push_back in the queue
    pub fn new(queue: &'a mut VecDeque<InputEvent>) -> Self {
        Self { events: queue }
    }

    pub fn forward(&mut self, event: InputEvent) {
        self.events.push_back(event);
    }

    pub fn key_down(&mut self, key: Key) {
        self.forward(InputEvent::new(EventType::KEY, key.code(), 1));
    }

    pub fn key_up(&mut self, key: Key) {
        self.forward(InputEvent::new(EventType::KEY, key.code(), 0));
    }

    pub fn key_hold(&mut self, key: Key) {
        self.forward(InputEvent::new(EventType::KEY, key.code(), 2));
    }

    pub fn key_tap(&mut self, key: Key) {
        self.key_down(key);
        self.key_up(key);
    }
}
