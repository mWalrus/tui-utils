use std::time::Duration;

pub mod blocks;
pub mod component;
pub mod keys;
pub mod rect;
pub mod split;
pub mod state;
pub mod style;
pub mod term;

/// Used for event polling with crossterm
pub(crate) static EVENT_TIMEOUT: Duration = Duration::from_millis(1000);
/// Default highlight symbol
pub static LIST_HIGHLIGHT_SYMBOL: &str = " > ";
