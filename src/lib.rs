use std::time::Duration;

pub mod blocks;
pub mod component;
pub mod keymap;
pub mod rect;
pub mod split;
pub mod term;

pub(crate) static EVENT_TIMEOUT: Duration = Duration::from_millis(1000);
pub static LIST_HIGHLIGHT_SYMBOL: &str = " > ";
