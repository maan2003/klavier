use evdev::{InputEvent, InputEventKind, Key};

pub enum KeyEvent {
    Press(Key),
    Hold(Key),
    Release(Key),
}

pub trait KeyEventExt {
    fn key_event(&self) -> Option<KeyEvent>;
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
