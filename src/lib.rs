use std::time::Duration;

pub mod blocks;
pub mod keymap;
pub mod term;

pub(crate) static EVENT_TIMEOUT: Duration = Duration::from_millis(1000);
