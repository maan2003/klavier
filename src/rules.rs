mod remaper;
mod magic_shift;

use crate::ext::*;
use evdev::{InputEvent, Key};
use std::io;

pub use remaper::*;
pub use magic_shift::*;

pub trait Rule {
    // returns true if the event was handled
    fn handle_event(&mut self, event: &InputEvent) -> io::Result<Vec<InputEvent>>;
}

struct ModOrKey {
    key: Key,
    mod_key: Key,
    saw_other_key: bool,
    real_key: Key,
}

pub struct IfHeld {
    key: Key,
    rule: Box<dyn Rule>,
    held: bool,
}

impl Rule for ModOrKey {
    fn handle_event(&mut self, event: &InputEvent) -> io::Result<Vec<InputEvent>> {
        match event.key_event() {
            Some(KeyEvent::Press(key)) if key == self.real_key => Ok(vec![key_down(self.mod_key)]),
            Some(KeyEvent::Release(key)) if key == self.real_key && self.saw_other_key => {
                Ok(vec![key_up(self.mod_key)])
            }
            Some(KeyEvent::Release(key)) if key == self.real_key => Ok(vec![
                key_up(self.mod_key),
                key_down(self.key),
                key_up(self.key),
            ]),
            Some(KeyEvent::Release(_key)) => {
                self.saw_other_key = true;
                Ok(vec![*event])
            }
            _ => Ok(vec![*event]),
        }
    }
}

impl Rule for IfHeld {
    fn handle_event(&mut self, event: &InputEvent) -> io::Result<Vec<InputEvent>> {
        match event.key_event() {
            Some(KeyEvent::Press(key)) if key == self.key => {
                self.held = true;
                Ok(vec![])
            }
            Some(KeyEvent::Release(key)) if key == self.key => {
                self.held = false;
                Ok(vec![])
            }
            // ignore the holding of this key
            Some(KeyEvent::Hold(key)) if key == self.key => Ok(vec![]),
            _ => {
                if self.held {
                    self.rule.handle_event(event)
                } else {
                    Ok(vec![*event])
                }
            }
        }
    }
}

pub fn mod_or_key(real_key: Key, mod_key: Key, key: Key) -> Box<dyn Rule> {
    Box::new(ModOrKey {
        key,
        mod_key,
        real_key,
        saw_other_key: false,
    })
}
pub fn if_held(key: Key, rule: Box<dyn Rule>) -> Box<dyn Rule> {
    Box::new(IfHeld {
        key,
        rule,
        held: false,
    })
}
