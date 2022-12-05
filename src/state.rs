use thiserror::Error;
use tui::widgets::ListState;

#[derive(Error, Debug)]
pub enum StateError {
    #[error("Out of Bounds Error: state selection not within boundary range {}..{} (is: {})", bounds.0, bounds.1, actual)]
    OutOfBounds { bounds: Boundary, actual: usize },
}

/// Define a boundary which is to be used with `BoundedState`
#[derive(Debug, Copy, Clone)]
pub struct Boundary(usize, usize);

impl<T> From<&Vec<T>> for Boundary {
    fn from(s: &Vec<T>) -> Self {
        Self(0, s.len() - 1)
    }
}

/// A wrapper around `ListState` which can be provided a boundary.
/// When the selection of inner reaches the defined boundaries, we
/// can choose whether we should wrap around to the other end of
/// the stack using the `Wrap` enum.
pub struct BoundedState {
    inner: ListState,
    boundary: Boundary,
    wrap: StateWrap,
}

/// This is used in combination with `BoundedState` to dictate whether
/// stepping should wrap around to the start when reaching boundaries.
#[derive(PartialEq, Eq)]
pub enum StateWrap {
    Enable,
    Disable,
}

impl Default for StateWrap {
    fn default() -> Self {
        Self::Enable
    }
}

impl BoundedState {
    /// Creates a `BoundedState` with boundaries and optional wrapping configuration
    pub fn new(boundary: Boundary, wrap: StateWrap) -> Self {
        Self {
            inner: ListState::default(),
            boundary,
            wrap,
        }
    }

    /// Creates a new `BoundedState` with a selection. This selection is bounds checked and
    /// will fail to be set if detected out of bounds.
    pub fn with_selection(
        boundary: Boundary,
        wrap: StateWrap,
        sel: usize,
    ) -> Result<Self, StateError> {
        let mut state = Self::new(boundary, wrap);
        state.select(sel)?;
        Ok(state)
    }

    /// Retrieve a mutable reference to the inner `ListState`. This is useful for when you need to
    /// use the `ListState` for drawing a stateful widget.
    pub fn inner(&mut self) -> &mut ListState {
        &mut self.inner
    }

    pub fn next(&mut self) {
        self.next_n(1)
    }

    pub fn prev(&mut self) {
        self.prev_n(1)
    }

    /// Step "backwards" a set number of steps.
    pub fn prev_n(&mut self, n: usize) {
        let i = match self.inner.selected() {
            Some(i) => {
                // define what happens when reaching boundary
                let wrap_outcome = match self.wrap {
                    StateWrap::Enable => self.boundary.1,
                    StateWrap::Disable => self.boundary.0,
                };

                if i == self.boundary.0 {
                    wrap_outcome
                } else if i.saturating_sub(n) <= self.boundary.0 {
                    self.boundary.0
                } else {
                    self.boundary.0.max(i.saturating_sub(n))
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
                    StateWrap::Enable => self.boundary.0,
                    StateWrap::Disable => self.boundary.1,
                };

                if i == self.boundary.1 {
                    wrap_outcome
                } else if i.saturating_add(n) >= self.boundary.1 {
                    self.boundary.1
                } else {
                    self.boundary.1.min(i.saturating_add(n))
                }
            }
            None => 0,
        };
        self.inner.select(Some(i));
    }

    /// Set a selection. This will error if the selection provided is out of bounds.
    pub fn select(&mut self, selection: usize) -> Result<(), StateError> {
        if selection > self.boundary.1 || selection < self.boundary.0 {
            return Err(StateError::OutOfBounds {
                bounds: self.boundary,
                actual: selection,
            });
        }
        self.inner.select(Some(selection));
        Ok(())
    }

    /// Set new boundary constraints on the state
    pub fn update_boundary(&mut self, boundary: Boundary) {
        self.boundary = boundary;
    }

    /// Update the upper boundary and select the last element in the list.
    /// This is good for when you, for example, add a new item to the associated list
    /// and want to focus that item.
    pub fn update_upper_and_select(&mut self, upper: usize) {
        self.boundary.1 = upper;
        self.select(upper).expect("This should never fail");
    }

    /// Update the boundary definition using a `Vec<T>`
    pub fn update_boundary_from_vec<T>(&mut self, v: &Vec<T>) {
        self.boundary = Boundary::from(v);
        if let Some(s) = self.inner.selected() {
            if s < self.boundary.0 {
                self.inner.select(Some(self.boundary.0));
            } else if s > self.boundary.1 {
                self.inner.select(Some(self.boundary.1));
            }
        }
    }

    pub fn first(&mut self) {
        self.inner.select(Some(self.boundary.0));
    }

    pub fn last(&mut self) {
        self.inner.select(Some(self.boundary.1));
    }

    pub fn deselect(&mut self) {
        self.inner.select(None);
    }
}

#[cfg(test)]
mod tests {
    use super::{Boundary, BoundedState, StateWrap};

    #[test]
    #[should_panic]
    fn selection_out_of_bounds() {
        let boundary = Boundary(0, 10);
        let mut state = BoundedState::new(boundary, StateWrap::Enable);
        state.select(11).unwrap()
    }

    #[test]
    fn selection_on_edge() {
        let boundary = Boundary(0, 10);
        let mut state = BoundedState::new(boundary, StateWrap::Enable);
        state.select(10).unwrap()
    }

    #[test]
    fn update_bounds_from_vec() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let boundary = Boundary::from(&v);

        let mut state = BoundedState::new(boundary, StateWrap::Enable);
        state.last();

        assert_eq!(state.inner.selected(), Some(5));

        v.push(7);
        v.push(8);
        v.push(9);
        v.push(10);

        state.update_boundary_from_vec(&v);

        state.last();

        assert_eq!(state.inner.selected(), Some(9));
    }

    #[test]
    fn update_bounds_and_select_last() {
        let boundary = Boundary(0, 10);
        let mut state = BoundedState::new(boundary, StateWrap::Enable);

        assert_eq!(state.inner.selected(), None);

        state.update_upper_and_select(20);

        assert_eq!(state.inner.selected(), Some(20))
    }

    #[test]
    fn selection_within_bounds() {
        let boundary = Boundary(0, 10);
        let mut state = BoundedState::new(boundary, StateWrap::Enable);
        state.select(5).unwrap()
    }

    #[test]
    fn wrap_disabled_should_stay() {
        let boundary = Boundary(0, 10);
        let mut state = BoundedState::new(boundary, StateWrap::Disable);

        state.last();
        assert_eq!(state.inner.selected(), Some(10));

        // push boundary
        state.next();
        state.next();
        state.next();
        assert_eq!(state.inner.selected(), Some(10));
    }

    #[test]
    fn wrap_enabled_should_wrap() {
        let boundary = Boundary(0, 10);
        let mut state = BoundedState::new(boundary, StateWrap::Enable);

        state.last();
        assert_eq!(state.inner.selected(), Some(10));

        // push boundary
        state.next();
        assert_eq!(state.inner.selected(), Some(0));
    }

    #[test]
    fn decrease_bounds_size_should_correct_oob() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let boundary = Boundary::from(&v);
        let mut state = BoundedState::new(boundary, StateWrap::Enable);

        state.last();
        assert_eq!(state.inner.selected(), Some(5));

        v.pop();
        state.update_boundary_from_vec(&v);

        assert!(state.inner.selected().unwrap() <= v.len() - 1);
    }
}
