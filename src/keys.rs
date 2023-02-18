use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

// Carbon copy of crossterm's `KeyEvent` which allows for easier construction and also easy comparison when input handling.
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

impl ToString for Keybind {
    fn to_string(&self) -> String {
        let key = match self.code {
            KeyCode::Char(c) if c == ' ' => '\u{23b5}',
            KeyCode::Char(c) => c,
            KeyCode::Tab => '\u{21e5}',
            KeyCode::BackTab => '\u{21e4}',
            KeyCode::Esc => '\u{238b}',
            KeyCode::Enter => '\u{23ce}',
            KeyCode::Up => '\u{2191}',
            KeyCode::Down => '\u{2193}',
            KeyCode::Left => '\u{2190}',
            KeyCode::Right => '\u{2192}',
            _ => '\u{2327}',
        };
        match self.modifiers {
            KeyModifiers::SHIFT if self.code == KeyCode::BackTab => key.to_string(),
            KeyModifiers::SHIFT => format!("\u{21e7}{key}"),
            KeyModifiers::CONTROL => format!("^{key}"),
            _ => key.to_string(),
        }
    }
}

/// Helper function to figure out if a specific key was pressed.
pub fn key_match(ev: &KeyEvent, binding: &Keybind) -> bool {
    ev.code == binding.code && ev.modifiers == binding.modifiers
}
