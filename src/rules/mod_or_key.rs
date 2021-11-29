use std::io;

use evdev::{InputEvent, Key};

use crate::ext::{key_down, key_up, KeyEvent, KeyEventExt};

use super::Rule;

struct ModOrKey {
    key: Key,
    mod_key: Key,
    saw_other_key: bool,
    real_key: Key,
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

pub fn mod_or_key(real_key: Key, mod_key: Key, key: Key) -> Box<dyn Rule> {
    Box::new(ModOrKey {
        key,
        mod_key,
        real_key,
        saw_other_key: false,
    })
}
