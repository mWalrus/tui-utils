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
            KeyCode::Char(c) if c == ' ' => '˽',
            KeyCode::Char(c) => c,
            KeyCode::Tab => '⇥',
            KeyCode::BackTab => '⇤',
            KeyCode::Esc => '⎋',
            KeyCode::Enter => '⏎',
            KeyCode::Up => '🡹',
            KeyCode::Down => '🡻',
            KeyCode::Left => '🡸',
            KeyCode::Right => '🡺',
            // unsupported characters
            _ => 'ⓧ',
        };
        match self.modifiers {
            KeyModifiers::SHIFT if self.code == KeyCode::BackTab => key.to_string(),
            KeyModifiers::SHIFT => format!("⇪{key}"),
            KeyModifiers::CONTROL => format!("^{key}"),
            _ => key.to_string(),
        }
    }
}

pub fn key_match(ev: &KeyEvent, binding: &Keybind) -> bool {
    ev.code == binding.code && ev.modifiers == binding.modifiers
}
