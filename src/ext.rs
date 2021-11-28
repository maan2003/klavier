use evdev::{uinput::VirtualDevice, EventType, InputEvent, InputEventKind, Key};
use std::io;

pub enum KeyEvent {
    Press(Key),
    Hold(Key),
    Release(Key),
}

pub trait KeyEventExt {
    fn key_event(&self) -> Option<KeyEvent>;
}

pub fn key_down(key: Key) -> InputEvent {
    InputEvent::new(EventType::KEY, key.code(), 1)
}

pub fn key_up(key: Key) -> InputEvent {
    InputEvent::new(EventType::KEY, key.code(), 0)
}

impl KeyEventExt for InputEvent {
    fn key_event(&self) -> Option<KeyEvent> {
        match self.kind() {
            InputEventKind::Key(key) if self.value() == 1 => Some(KeyEvent::Press(key)),
            InputEventKind::Key(key) if self.value() == 0 => Some(KeyEvent::Release(key)),
            InputEventKind::Key(key) if self.value() == 2 => Some(KeyEvent::Hold(key)),
            _ => None,
        }
    }
}
