use thiserror::Error;
use tui::widgets::ListState;

#[derive(Error, Debug)]
pub enum StateError {
    #[error("state selection not within boundary range {0}..{1} (is: {2})")]
    OutOfBounds(usize, usize, usize),
}

/// A wrapper around `ListState` which can be provided boundaries.
/// When the selection of inner reaches the defined boundaries, we
/// can choose whether we should wrap around to the other end of
/// the stack using the `Wrap` enum.
pub struct BoundedState {
    inner: ListState,
    lower: usize,
    upper: usize,
    wrap: Wrap,
}

/// This is used in combination with `BoundedState` to dictate whether
/// stepping should wrap around to the start when reaching boundaries.
#[derive(PartialEq, Eq)]
pub enum Wrap {
    Enable,
    Disable,
}

impl Default for Wrap {
    fn default() -> Self {
        Self::Enable
    }
}

impl BoundedState {
    /// Creates a `BoundedState` with boundaries and optional wrapping configuration
    pub fn new(lower: usize, upper: usize, wrap: Option<Wrap>) -> Self {
        Self {
            inner: ListState::default(),
            lower,
            upper,
            wrap: wrap.unwrap_or_default(),
        }
    }

    /// Creates a new `BoundedState` with a selection. This selection is bounds checked and
    /// will fail to be set if detected out of bounds.
    pub fn with_selection(
        lower: usize,
        upper: usize,
        wrap: Option<Wrap>,
        sel: usize,
    ) -> Result<Self, StateError> {
        let mut state = Self::new(lower, upper, wrap);
        state.select(sel)?;
        Ok(state)
    }

    /// Retrieve a mutable reference to the inner `ListState`. This is useful for when you need to
    /// use the `ListState` for drawing a stateful widget.
    pub fn inner(&mut self) -> &mut ListState {
        &mut self.inner
    }

    /// Take one step "forwards".
    pub fn next(&mut self) {
        self.next_n(1)
    }

    /// Take one step "backwards".
    pub fn prev(&mut self) {
        self.prev_n(1)
    }

    /// Step "backwards" a set number of steps.
    pub fn prev_n(&mut self, n: usize) {
        let i = match self.inner.selected() {
            Some(i) => {
                // define what happens when reaching boundary
                let wrap_outcome = match self.wrap {
                    Wrap::Enable => self.upper,
                    Wrap::Disable => self.lower,
                };

                if i == self.lower {
                    wrap_outcome
                } else if i.saturating_sub(n) <= self.lower {
                    self.lower
                } else {
                    self.lower.max(i.saturating_sub(n))
                }
            }
            None => 0,
        };
        self.inner.select(Some(i));
    }

    /// Step "forwards" a set number of steps.
    pub fn next_n(&mut self, n: usize) {
        let i = match self.inner.selected() {
            Some(i) => {
                // define what happens when reaching boundary
                let wrap_outcome = match self.wrap {
                    Wrap::Enable => self.lower,
                    Wrap::Disable => self.upper,
                };

                if i == self.upper {
                    wrap_outcome
                } else if i.saturating_add(n) >= self.upper {
                    self.upper
                } else {
                    self.upper.min(i.saturating_add(n))
                }
            }
            None => 0,
        };
        self.inner.select(Some(i));
    }

    /// Set a selection. This will error if the selection provided is out of bounds.
    pub fn select(&mut self, selection: usize) -> Result<(), StateError> {
        if selection > self.upper || selection < self.lower {
            return Err(StateError::OutOfBounds(self.lower, self.upper, selection));
        }
        self.inner.select(Some(selection));
        Ok(())
    }
}
