use std::io;

use evdev::{InputEvent, Key};

use crate::key_state::{KeyState, KeyEventExt};
use crate::layer::Layer;

use super::{Rule, RuleCtx};

pub struct IfHeld {
    key: Key,
    then: Layer,
    or: Layer,
    held: bool,
}

impl Rule for IfHeld {
    fn event(&mut self, ctx: &mut RuleCtx, event: &InputEvent) -> io::Result<()> {
        let key = event.key();
        match event.key_state() {
            Some(KeyState::Press) if key == self.key => {
                self.held = true;
            }
            Some(KeyState::Release) if key == self.key => {
                self.held = false;
            }
            // ignore the holding of this key
            Some(KeyState::Hold) if key == self.key => {}
            _ => {
                if self.held {
                    self.then.event(ctx, event)?;
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
    })
}
