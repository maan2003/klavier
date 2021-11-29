mod if_held;
mod magic_shift;
mod mod_or_key;
mod remaper;

use evdev::InputEvent;
use std::io;

pub use {if_held::if_held, magic_shift::magic_shift, mod_or_key::mod_or_key, remaper::*};

pub trait Rule {
    // returns true if the event was handled
    fn handle_event(&mut self, event: &InputEvent) -> io::Result<Vec<InputEvent>>;
}
