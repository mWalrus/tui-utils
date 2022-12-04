use std::error::Error;

use crossterm::event::KeyEvent;
use tui::{backend::Backend, Frame};

/// Trait for implementing components
pub trait Component {
    /// Set this to your defined application message enum.
    ///
    /// It can also be left empty:
    /// ```
    /// type Message = ();
    /// ```
    type Message: Default;
    /// Handle all draw logic like constructing widgets and so on.
    /// `dim` can be set if you want to gray out the widgets. Note
    /// that this requires importing the `Dim` trait to be able to
    /// apply the dimming on `Block`s
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, dim: bool);
    /// Take care of any input handling here. This method is not
    /// required when implementing `Component` in case your component
    /// does not require input handling.
    fn handle_input(&mut self, _key: KeyEvent) -> Result<Self::Message, Box<dyn Error>> {
        Ok(Default::default())
    }
}
