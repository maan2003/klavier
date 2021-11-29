use std::io;

use evdev::{InputEvent, Key};

use crate::ext::{KeyEvent, KeyEventExt};

use super::{Rule, RuleCtx};

struct IfHeld {
    key: Key,
    rule: Box<dyn Rule>,
    held: bool,
}

impl Rule for IfHeld {
    fn event(&mut self, ctx: &mut RuleCtx, event: &InputEvent) -> io::Result<()> {
        match event.key_event() {
            Some(KeyEvent::Press(key)) if key == self.key => {
                self.held = true;
            }
            Some(KeyEvent::Release(key)) if key == self.key => {
                self.held = false;
            }
            // ignore the holding of this key
            Some(KeyEvent::Hold(key)) if key == self.key => {},
            _ => {
                if self.held {
                    self.rule.event(ctx, event)?;
                } else {
                    ctx.forward(*event);
                }
            }
        }
        Ok(())
    }
}

pub fn if_held(key: Key, rule: Box<dyn Rule>) -> Box<dyn Rule> {
    Box::new(IfHeld {
        key,
        rule,
        held: false,
    })
}
