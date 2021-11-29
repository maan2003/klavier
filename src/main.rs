#![feature(decl_macro)]

mod all_keys;
mod ext;
mod min_keys;
mod rules;

use evdev::Device;
use std::{collections::VecDeque, error::Error};

use min_keys::*;
use rules::*;

const DEVICE_PATH: &str = "/dev/input/by-path/platform-i8042-serio-0-event-kbd";

fn rules() -> Vec<Box<dyn Rule>> {
    let src = keys!(
        TAB     Q     W     E     R     T     Y     U     I     O     P      LBRACE  RBRACE
        CAPS    A     S     D     F     G     H     J     K     L     SMCLN  SQUOTE
        LSHFT   Z     X     C     V     B     N     M     COMMA DOT   SLASH  RSHFT
        LCTRL   LMETA LALT            SPACE               RALT  RCTRL
    );
    // the colemak dh layout
    let cmk = keys!(
        TAB     Q     W     F     P     B     J     L     U     Y     SMCLN  LBRACE  RBRACE
        CAPS    A     R     S     T     G     M     N     E     I     O      SQUOTE
        LSHFT   X     C     D     V     Z     K     H     COMMA DOT   SLASH  RSHFT
        LCTRL   LMETA LALT            SPACE               RALT  RCTRL
    );

    let ext = remap!(
        N => LEFT,
        U => UP,
        E => DOWN,
        I => RIGHT,
        O => BS,
    );

    vec![
        remap(&src, &cmk),
        magic_shift(),
        mod_or_key(CAPS, CAPS, F9),
        mod_or_key(RALT, RALT, ENTER),
        if_held(CAPS, ext),
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut dev = Device::open(DEVICE_PATH).unwrap();
    let mut out = evdev::uinput::VirtualDeviceBuilder::new()?
        .name("MAIN")
        .with_keys(&all_keys::all_keys())?
        .build()?;

    dev.grab()?;

    let mut rules = rules();

    let mut evs = VecDeque::new();
    loop {
        evs.extend(dev.fetch_events()?);
        for rule in &mut rules {
            // pop first evs.len() event, new events will be added by RuleCtx
            for _ in 0..evs.len() {
                let ev = evs.pop_front().unwrap();
                let mut ctx = RuleCtx::new(&mut evs);
                rule.event(&mut ctx, &ev)?;
            }
        }

        // emit the events at the end
        let slices = evs.as_slices();
        out.emit(slices.0)?;
        out.emit(slices.1)?;
    }
}
