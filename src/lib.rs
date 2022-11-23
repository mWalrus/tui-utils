use crossterm::event::{self, Event};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io;
use std::time::Duration;
use tui::backend::{Backend, CrosstermBackend};
use tui::{Frame, Terminal};

pub(crate) static EVENT_TIMEOUT: Duration = Duration::from_millis(1000);
pub type TerminalResult<T> = std::result::Result<T, Box<dyn Error>>;

/// Prepare the terminal by flushing the screen and hiding the cursor
pub fn init_terminal() -> TerminalResult<Terminal<CrosstermBackend<io::Stdout>>> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

/// Restore the terminal to its initial state
pub fn restore_terminal() -> TerminalResult<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}

/// Poll for crossterm events such as key events
pub fn poll_event() -> TerminalResult<Option<Event>> {
    if event::poll(EVENT_TIMEOUT)? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}
