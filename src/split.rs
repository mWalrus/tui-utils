use tui::layout::{Constraint, Direction, Layout, Rect};

/// Set the percentage ratio for the split
pub struct Ratio {
    left: u16,
    right: u16,
}

impl Default for Ratio {
    fn default() -> Self {
        Self {
            left: 50,
            right: 50,
        }
    }
}

impl Ratio {
    /// Set a specific ratio in percentage units
    /// ```
    /// // set the ratio to 70%/30%
    /// let ratio = Ratio::new(70, 30);
    /// ```
    pub fn new(left: u16, right: u16) -> Self {
        Self { left, right }
    }
}

/// Generate a vertically split layout in a rect with a defined ratio
pub fn v_split(rect: Rect, ratio: Ratio) -> Vec<Rect> {
    construct_split(rect, ratio, Direction::Vertical)
}

/// Generate a horizontally split layout in a rect with a defined ratio
pub fn h_split(rect: Rect, ratio: Ratio) -> Vec<Rect> {
    construct_split(rect, ratio, Direction::Horizontal)
}

fn construct_split(re: Rect, ra: Ratio, d: Direction) -> Vec<Rect> {
    Layout::default()
        .direction(d)
        .constraints(
            [
                Constraint::Percentage(ra.left),
                Constraint::Percentage(ra.right),
            ]
            .as_ref(),
        )
        .split(re)
}
