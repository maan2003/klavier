// not really needed, just using for ease of use
#![feature(decl_macro)]

mod ext;
mod keys;
mod layer;
mod rules;

use evdev::uinput::VirtualDeviceBuilder;
use evdev::{AttributeSet, Device};
use layer::Layer;
use std::error::Error;

use keys::*;
use rules::*;

const DEVICE_PATH: &str = "/dev/input/by-path/platform-i8042-serio-0-event-kbd";

fn rules() -> Vec<Box<dyn Rule>> {
    let src = keys!(
        TAB     Q     W     E     R     T     Y     U     I     O     P      LBRACE  RBRACE
        CAPS    A     S     D     F     G     H     J     K     L     SMCLN  SQT
        LSHT    Z     X     C     V     B     N     M     COMA  DOT   SLSH   RSHT
        LCTRL   SUP  LALT            SPC                RALT  RCTRL
    );
    // the colemak dh layout
    let cmk = keys!(
        TAB     Q     W     F     P     B     J     L     U     Y     SMCLN  LBRACE  RBRACE
        CAPS    A     R     S     T     G     M     N     E     I     O      SQT
        LSHT    X     C     D     V     Z     K     H     COMA  DOT   SLSH   RSHT
        LCTRL   SUP  LALT            SPC                RALT  RCTRL
    );

    let ext = remap!(
        S => SUP,
        T => LCTRL,
        R => LSHT,
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
        mod_or_key(RALT, RALT, RET),
        if_held(CAPS, [ext], []),
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut dev = Device::open(DEVICE_PATH).unwrap();
    let mut all_keys = AttributeSet::new();
    for key in ALL_KEYS {
        all_keys.insert(*key);
    }

    let mut out = VirtualDeviceBuilder::new()?
        .name("MAIN")
        .with_keys(&all_keys)?
        .build()?;

    dev.grab()?;

    let mut root_layer = Layer::new(rules());
    loop {
        root_layer.event_device(&mut dev, &mut out)?;
    }
}
