#![feature(decl_macro)]

mod event_handler;
mod ext;
mod min_keys;

use std::{error::Error, mem};
mod all_keys;
use evdev::Device;
use event_handler::{keys, mod_or_key, remap, EventHandler};
use min_keys::*;

use crate::event_handler::if_held;

const DEVICE_PATH: &str = "/dev/input/by-path/platform-i8042-serio-0-event-kbd";

fn rules() -> Vec<Box<dyn EventHandler>> {
    let src = keys!(
        ESC    F1   F2   F3   F4   F5   F6   F7   F8   F9   F10   F11   F12
        GRAVE   N1    N2    N3    N4    N5    N6    N7    N8    N9    N0         MINUS   EQUAL    BS
        TAB     Q     W     E     R     T     Y     U     I     O     P          LBRACE  RBRACE   BACKSLASH
        CAPS    A     S     D     F     G     H     J     K     L     SEMICOLON  APOSTROPHE       ENTER
        LSHIFT  Z     X     C     V     B     N     M     COMMA DOT   SLASH           RSHIFT
        LCTRL   LMETA LALT            SPACE               RALT  RCTRL
    );
    // the colemak dh layout
    let cmk = keys!(
        ESC    F1   F2   F3   F4   F5   F6   F7   F8   F9   F10   F11   F12
        GRAVE   N1    N2    N3    N4    N5    N6    N7    N8    N9    N0         MINUS   EQUAL    BS
        TAB     Q     W     F     P     B     J     L     U     Y     SEMICOLON  LBRACE  RBRACE   BACKSLASH
        CAPS    A     R     S     T     G     M     N     E     I     O          APOSTROPHE       ENTER
        LSHIFT  X     C     D     V     Z     K     H     COMMA DOT   SLASH           RSHIFT
        LCTRL   LMETA LALT            SPACE               RALT  RCTRL
    );

    let ext = remap!(
        O => BS,
        N => LEFT,
        U => UP,
        E => DOWN,
        I => RIGHT,
    );

    vec![
        remap(&src, &cmk),
        mod_or_key(CAPS, CAPS, F9),
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
    loop {
        let mut evs: Vec<_> = dev.fetch_events()?.collect();
        let mut next_evs = Vec::new();
        for rule in &mut rules {
            for ev in evs.drain(..) {
                let more_events = rule.handle_event(&ev)?;
                next_evs.extend(more_events);
            }
            mem::swap(&mut evs, &mut next_evs);
        }

        out.emit(&evs)?;
    }
}
