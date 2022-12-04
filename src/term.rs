use crate::EVENT_TIMEOUT;
use crossterm::event::{self, Event};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io;
use thiserror::Error;
use tui::backend::CrosstermBackend;
use tui::Terminal;

#[derive(Error, Debug)]
pub enum TermError {
    #[error("event polling failed: {0}")]
    Poll(String),
    #[error("failed to restore terminal: {0}")]
    Restore(String),
    #[error("failed to init terminal: {0}")]
    Init(String),
}

/// Prepare the terminal by clearing the screen and hiding the cursor
pub fn init() -> Result<Terminal<CrosstermBackend<io::Stdout>>, TermError> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen)
        .map_err(|e| TermError::Init(e.to_string()))?;
    enable_raw_mode().map_err(|e| TermError::Init(e.to_string()))?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend).map_err(|e| TermError::Init(e.to_string()))?;
    terminal
        .hide_cursor()
        .map_err(|e| TermError::Init(e.to_string()))?;

    Ok(terminal)
}

// Restore the terminal and print an error
pub fn restore_with_err(e: Box<dyn Error>) -> Result<(), TermError> {
    restore()?;
    eprintln!("Application error: {e}");
    Ok(())
}

/// Restore the terminal to its initial state
pub fn restore() -> Result<(), TermError> {
    disable_raw_mode().map_err(|e| TermError::Restore(e.to_string()))?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)
        .map_err(|e| TermError::Restore(e.to_string()))?;

    Ok(())
}

/// Poll for crossterm events such as key events. This blocks the current
/// thread for 1 second before timeout, letting the application
/// continue execution.
pub fn poll_event() -> Result<Option<Event>, TermError> {
    let outcome = match event::poll(EVENT_TIMEOUT) {
        Ok(b) => b,
        Err(e) => Err(TermError::Poll(e.to_string()))?,
    };
    if outcome {
        Ok(Some(
            event::read().map_err(|e| TermError::Poll(e.to_string()))?,
        ))
    } else {
        Ok(None)
    }
}
