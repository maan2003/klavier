use std::io;

use evdev::{InputEvent, Key};

use crate::ext::{KeyEvent, KeyEventExt};

use super::Rule;

struct IfHeld {
    key: Key,
    rule: Box<dyn Rule>,
    held: bool,
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

pub fn if_held(key: Key, rule: Box<dyn Rule>) -> Box<dyn Rule> {
    Box::new(IfHeld {
        key,
        rule,
        held: false,
    })
}
