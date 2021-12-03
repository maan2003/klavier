use evdev::{InputEvent, InputEventKind, Key};

#[derive(Debug)]
pub enum KeyState {
    Press,
    Release,
    Hold,
}

pub trait KeyEventExt {
    fn key_state(&self) -> Option<KeyState>;
    fn key(&self) -> Key;
}

impl KeyEventExt for InputEvent {
    fn key_state(&self) -> Option<KeyState> {
        match self.kind() {
            InputEventKind::Key(key) if self.value() == 1 => Some(KeyState::Press),
            InputEventKind::Key(key) if self.value() == 0 => Some(KeyState::Release),
            InputEventKind::Key(key) if self.value() == 2 => Some(KeyState::Hold),
            _ => None,
        }
    }

    fn key(&self) -> Key {
        Key::new(self.code())
    }
}
