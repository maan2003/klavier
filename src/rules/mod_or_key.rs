use std::io;

use evdev::{InputEvent, Key};

use crate::key_state::{KeyState, KeyEventExt};

use super::{Rule, RuleCtx};

struct ModOrKey {
    key: Key,
    mod_key: Key,
    saw_other_key: bool,
    real_key: Key,
}

impl Rule for ModOrKey {
    fn event(&mut self, ctx: &mut RuleCtx, event: &InputEvent) -> io::Result<()> {
        let key = event.key();
        match event.key_state() {
            Some(KeyState::Press) if key == self.real_key => {
                ctx.key_down(self.mod_key);
                self.saw_other_key = false;
            }
            Some(KeyState::Release) if key == self.real_key => {
                ctx.key_up(self.mod_key);
                if !self.saw_other_key {
                    ctx.key_tap(self.key);
                }
            }
            Some(KeyState::Hold) if key == self.real_key => {}
            Some(KeyState::Press) if key != self.real_key => {
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

    test_case! {
        hold_is_ignored
        rule = mod_or_key(A, B, C),
        input = (
            down A
            hold A
            up A
        ),
        output = (
            down B
            up B
            down C
            up C
        ),
    }

    test_case! {
        works_as_mod
        rule = mod_or_key(A, B, C),
        input = (
            down A
            down D
            up D
            up A
        ),
        output = (
            down B
            down D
            up D
            up B
        ),
    }

    test_case! {
        other_keys_are_bypassed
        rule = mod_or_key(A, B, C),
        input = (
            down D
            down E
            down G
            hold G
            up G
            up E
            up D
        ),
        output = (
            down D
            down E
            down G
            hold G
            up G
            up E
            up D
        ),
    }
}
