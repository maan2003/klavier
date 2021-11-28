use evdev::{uinput::VirtualDevice, InputEvent, Key};
use std::{collections::HashMap, io};

use super::EventHandler;

pub struct Remaper {
    src: HashMap<u16, usize>,
    dst: Vec<u16>,
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
        src: src
            .iter()
            .enumerate()
            .map(|(i, key)| (key.code(), i))
            .collect(),
        dst: dst.iter().map(|x| x.code()).collect(),
    })
}

impl super::EventHandler for Remaper {
    fn handle_event(
        &mut self,
        event: &InputEvent,
    ) -> io::Result<Vec<InputEvent>> {
        if let Some(i) = self.src.get(&event.code()) {
            let new_code = self.dst[*i];
            let new_event = InputEvent::new(event.event_type(), new_code, event.value());
            Ok(vec![new_event])
        } else {
            Ok(vec![*event])
        }
    }
}
