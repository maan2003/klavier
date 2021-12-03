use std::collections::HashSet;
use std::io;

use evdev::{EventType, InputEvent, Key};

use crate::key_state::{KeyEventExt, KeyState};
use crate::layer::Layer;

use super::{Rule, RuleCtx};

pub struct IfHeld {
    key: Key,
    then: Layer,
    or: Layer,
    held: bool,
    keys_down_then: HashSet<Key>,
}

impl Rule for IfHeld {
    fn event(&mut self, ctx: &mut RuleCtx, event: &InputEvent) -> io::Result<()> {
        let key = event.key();
        match event.key_state() {
            Some(KeyState::Press) if key == self.key => {
                self.held = true;
            }
            Some(KeyState::Release) if key == self.key => {
                for key in self.keys_down_then.iter() {
                    let event = InputEvent::new(EventType::KEY, key.code(), 0);
                    self.then.event(ctx, &event)?;
                }
                self.keys_down_then.clear();
                self.held = false;
            }
            // ignore the holding of this key
            Some(KeyState::Hold) if key == self.key => {}
            state => {
                if self.held {
                    self.then.event(ctx, event)?;
                    if let Some(KeyState::Press) = state {
                        self.keys_down_then.insert(key);
                    } else if let Some(KeyState::Release) = state {
                        let _ = self.keys_down_then.remove(&key);
                    }
                } else {
                    self.or.event(ctx, event)?;
                }
            }
        }
        Ok(())
    }
}

pub fn if_held(
    key: Key,
    then: impl IntoIterator<Item = Box<dyn Rule>>,
    or: impl IntoIterator<Item = Box<dyn Rule>>,
) -> Box<dyn Rule> {
    Box::new(IfHeld {
        key,
        then: Layer::new(then.into_iter().collect()),
        or: Layer::new(or.into_iter().collect()),
        held: false,
        keys_down_then: HashSet::new(),
    })
}
