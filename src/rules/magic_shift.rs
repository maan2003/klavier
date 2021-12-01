use std::{
    io,
    time::{Duration, SystemTime},
};

use crate::ext::*;
use crate::keys::LSHT;
use evdev::InputEvent;

use super::{Rule, RuleCtx};

// of course, shift + char works
// single tap => enable shift for single character
// double tap => keep shift enabled
//    -> tap again to disable shift
struct MagicShift {
    state: ShiftState,
}

enum ShiftState {
    None,
    Held(SystemTime),
    // shift got normally used, no need to handle anymore
    NormallyUsed,
    SingleTaped,
    DoubleTaped,
}

impl Rule for MagicShift {
    fn event(&mut self, ctx: &mut RuleCtx, event: &InputEvent) -> io::Result<()> {
        match (&self.state, event.key_event()) {
            // shift is pressed
            (ShiftState::None, Some(KeyEvent::Press(key))) if key == LSHT => {
                self.state = ShiftState::Held(event.timestamp());
                ctx.forward(*event);
            }
            // case 1: normal shift use
            // a key released when shift is held
            (ShiftState::Held(_), Some(KeyEvent::Release(key))) if key != LSHT => {
                self.state = ShiftState::NormallyUsed;
                ctx.forward(*event);
            }

            // after normal use, shift is released
            (ShiftState::NormallyUsed, Some(KeyEvent::Release(key))) if key == LSHT => {
                self.state = ShiftState::None;
                ctx.forward(*event);
            }

            // case 2: single tap
            // shift was held, and was not used as a modifier
            // make sure shift is not held for more than 500ms
            (ShiftState::Held(time), Some(KeyEvent::Release(key)))
                if key == LSHT
                    && event.timestamp().duration_since(*time).unwrap()
                        < Duration::from_millis(500) =>
            {
                self.state = ShiftState::SingleTaped;
                // don't send the key event
            }

            // other key is released after single tap
            (ShiftState::SingleTaped, Some(KeyEvent::Release(key))) if key != LSHT => {
                self.state = ShiftState::None;
                ctx.forward(*event);
                // now release shift
                ctx.key_up(LSHT);
            }

            // case 3: double tap
            // after single tap, shift is released again with no other keys pressed
            (ShiftState::SingleTaped, Some(KeyEvent::Release(key))) if key == LSHT => {
                self.state = ShiftState::DoubleTaped;
                // don't send any event
            }

            // on another shift release after double tap, disable the shift
            (ShiftState::DoubleTaped, Some(KeyEvent::Release(key))) if key == LSHT => {
                self.state = ShiftState::None;
                // lets release the shift by sending this event
                ctx.forward(*event);
            }

            _ => ctx.forward(*event),
        }
        Ok(())
    }
}

pub fn magic_shift() -> Box<dyn Rule> {
    Box::new(MagicShift {
        state: ShiftState::None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;
    use crate::keys::*;

    test_case! {
        other_keys_fail_thru
        rule = magic_shift(),
        input = (
            down C
            down D
            hold C
            up C
            up D
        ),
        output = (
            down C
            down D
            hold C
            up C
            up D
        ),
    }

    test_case! {
        normal_shift_use
        rule = magic_shift(),
        input = (
            down LSHT
            down A
            hold A
            up A
            up LSHT
        ),
        output = (
            down LSHT
            down A
            hold A
            up A
            up LSHT
        ),
    }

    test_case!(
        single_tap
        rule = magic_shift(),
        input = (
            down LSHT
            up LSHT
            down A
            hold A
            up A
            down A
            hold A
            up A
        ),
        output = (
            down LSHT
            down A
            hold A
            up A
            up LSHT
            down A
            hold A
            up A
        ),
    );

    test_case!(
        double_tap
        rule = magic_shift(),
        input = (
            down LSHT
            up LSHT
            down LSHT
            up LSHT
            down A
            hold A
            up A
            down B
            hold B
            up B
            down LSHT
            up LSHT
            // now release shift
            down A
            hold A
            up A
        ),
        output = (
            down LSHT
            // TODO: find a way to not send this
            down LSHT 
            down A
            hold A
            up A
            down B
            hold B
            up B
            // TODO: find a way to not send this
            down LSHT 
            up LSHT
            down A
            hold A
            up A
        ),
    );
}
