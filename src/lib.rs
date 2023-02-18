use std::time::Duration;

pub mod blocks;
pub mod component;
pub mod keys;
pub mod rect;
pub mod split;
pub mod state;
pub mod style;
pub mod term;

pub mod shared {
    pub use shared_derive::Shared;
    /// Creates a `Rc` wrapper around your keymap which allows you to
    /// take a reference to a single allocated instance of your keymap.
    /// This is useful for when you have more than one component which
    /// performs input handling since you can clone a pointer to the same
    /// keymap and give out to each component.
    pub trait Shared {
        fn shared() -> Rc<Self>;
    }
}

/// Used for event polling with crossterm
pub(crate) static EVENT_TIMEOUT: Duration = Duration::from_millis(1000);
/// Default highlight symbol
pub static LIST_HIGHLIGHT_SYMBOL: &str = " > ";
