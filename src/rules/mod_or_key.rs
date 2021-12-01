use std::io;

use evdev::{InputEvent, Key};

use crate::ext::{KeyEvent, KeyEventExt};

use super::{Rule, RuleCtx};

struct ModOrKey {
    key: Key,
    mod_key: Key,
    saw_other_key: bool,
    real_key: Key,
}

impl Rule for ModOrKey {
    fn event(&mut self, ctx: &mut RuleCtx, event: &InputEvent) -> io::Result<()> {
        match event.key_event() {
            Some(KeyEvent::Press(key)) if key == self.real_key => ctx.key_down(self.mod_key),
            Some(KeyEvent::Release(key)) if key == self.real_key && self.saw_other_key => {
                ctx.key_up(self.mod_key);
            }
            Some(KeyEvent::Release(key)) if key == self.real_key => {
                ctx.key_up(self.mod_key);
                ctx.key_tap(self.key);
            }
            Some(KeyEvent::Release(_key)) => {
                self.saw_other_key = true;
                ctx.forward(*event);
            }
            _ => {
                ctx.forward(*event);
            }
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::keys::*;
    use crate::test_util::*;

    test_case! {
        single_tap
        rule = mod_or_key(A, B, C),
        input = (
            down A
            up A
        ),
        output = (
            down B
            up B
            down C
            up C
        ),
    }
}
