mod if_held;
mod magic_shift;
mod mod_or_key;
mod remaper;

use evdev::{EventType, InputEvent, Key};
use std::io;

pub use {if_held::if_held, magic_shift::magic_shift, mod_or_key::mod_or_key, remaper::*};

pub trait Rule {
    // returns true if the event was handled
    fn handle_event(
        &mut self,
        ctx: &mut RuleCtx,
        event: &InputEvent,
    ) -> io::Result<()>;
}

#[derive(Debug)]
pub struct RuleCtx {
    events: Vec<InputEvent>,
}

impl RuleCtx {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn forward(&mut self, event: InputEvent) {
        self.events.push(event);
    }

    pub fn key_down(&mut self, key: Key) {
        self.forward(InputEvent::new(EventType::KEY, key.code(), 1));
    }

    pub fn key_up(&mut self, key: Key) {
        self.forward(InputEvent::new(EventType::KEY, key.code(), 0));
    }

    pub fn key_tap(&mut self, key: Key) {
        self.key_down(key);
        self.key_up(key);
    }

    pub fn events(&self) -> &[InputEvent] {
        &self.events
    }
}
