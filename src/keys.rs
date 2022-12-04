use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};
pub use shared_derive::Shared;
use std::rc::Rc;

/// Creates a `Rc` wrapper around your keymap which allows you to
/// take a reference to a single allocated instance of your keymap.
/// This is useful for when you have more than one component which
/// performs input handling since you can clone a pointer to the same
/// keymap and give out to each component.
pub trait Shared {
    fn shared() -> Rc<Self>;
}

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

/// Helper function to figure out if a specific key was pressed.
pub fn key_match(ev: &KeyEvent, binding: &Keybind) -> bool {
    ev.code == binding.code && ev.modifiers == binding.modifiers
}
