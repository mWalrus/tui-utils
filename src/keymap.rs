use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Keybind {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl Keybind {
    pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

pub fn key_match(ev: &KeyEvent, binding: &Keybind) -> bool {
    ev.code == binding.code && ev.modifiers == binding.modifiers
}

impl PartialEq for Keybind {
    fn eq(&self, other: &Self) -> bool {
        let ev: KeyEvent = self.into();
        let other: KeyEvent = other.into();
        ev == other
    }
}

impl From<&Keybind> for KeyEvent {
    fn from(other: &Keybind) -> Self {
        Self::new(other.code, other.modifiers)
    }
}
