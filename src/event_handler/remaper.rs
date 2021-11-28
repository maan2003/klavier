use evdev::{uinput::VirtualDevice, InputEvent, Key};
use std::{collections::HashMap, io};

use super::EventHandler;

pub struct Remaper {
    // a map of keycode to keycode
    remap: HashMap<u16, u16>,
}

pub type Map = Vec<Key>;

pub macro keys($($key:ident)*) {
    vec![$($key),*]
}

pub macro remap($($src:ident => $dst:ident),* $(,)?) {{
    let src = keys! {$($src)*};
    let dst = keys! {$($dst)*};
    remap(&src, &dst)
}}

pub fn remap(src: &Map, dst: &Map) -> Box<dyn EventHandler> {
    Box::new(Remaper {
        remap: src
            .iter()
            .zip(dst.iter())
            .map(|(s, d)| (s.code(), d.code()))
            .collect(),
    })
}

impl super::EventHandler for Remaper {
    fn handle_event(&mut self, event: &InputEvent) -> io::Result<Vec<InputEvent>> {
        if let Some(&new_code) = self.remap.get(&event.code()) {
            let new_event = InputEvent::new(event.event_type(), new_code, event.value());
            Ok(vec![new_event])
        } else {
            Ok(vec![*event])
        }
    }
}
