use std::io;

use evdev::{InputEvent, Key};

use crate::key_state::{KeyEventExt, KeyState};

use super::{Rule, RuleCtx};

struct Macro {
    // key to start macro recording, end recording
    record_key: Key,
    // key to start macro playback
    play_key: Key,
    // are we recording a macro?
    recording: bool,
    // recorded macro - a list of InputEvents
    recorded_macro: Vec<InputEvent>,
}

impl Rule for Macro {
    fn event(&mut self, ctx: &mut RuleCtx, event: &InputEvent) -> io::Result<()> {
        let key = event.key();
        let state = event.key_state();

        if key == self.record_key {
            // if we are not recording, start on release of record key
            if !self.recording && state == Some(KeyState::Release) {
                self.recording = true;
                // clear the old recorded macro
                self.recorded_macro.clear();
            }

            // end recording if we're already recording and play key is released
            else if self.recording && state == Some(KeyState::Release) {
                self.recording = false;
            }
        } else if key == self.play_key {
            // play the recording of release of the play key
            if state == Some(KeyState::Release) {
                for event in &self.recorded_macro {
                    // forward the event to the next rule
                    ctx.forward(*event);
                }
            }
        }
        // we got a key event while recording, add it to the recorded_macro and forward it
        else if self.recording {
            self.recorded_macro.push(*event);
            ctx.forward(*event);
        }
        // we got a key event while not recording, forward it to the next rule
        else {
            ctx.forward(*event);
        }

        Ok(())
    }
}

pub fn rr_macro(record_key: Key, play_key: Key) -> Box<dyn Rule> {
    Box::new(Macro {
        record_key,
        play_key,
        recording: false,
        recorded_macro: Vec::new(),
    })
}
