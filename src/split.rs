use tui::layout::{Constraint, Direction, Layout, Rect};

/// Set the percentage ratio for the split
pub struct Ratio(u16, u16);

impl Default for Ratio {
    fn default() -> Self {
        Self(50, 50)
    }
}

impl Ratio {
    /// Set a specific ratio in percentage units
    /// ```
    /// use tui_utils::split::Ratio;
    ///
    /// // set the ratio to 70%/30%
    /// let ratio = Ratio::new(70, 30);
    /// ```
    pub fn new(first: u16, second: u16) -> Self {
        Self(first, second)
    }
}

/// Generate a vertically split layout in a rect with a defined ratio
pub fn v_split(rect: Rect, ratio: Ratio) -> Vec<Rect> {
    construct_split(rect, ratio, Direction::Horizontal)
}

/// Generate a horizontally split layout in a rect with a defined ratio
pub fn h_split(rect: Rect, ratio: Ratio) -> Vec<Rect> {
    construct_split(rect, ratio, Direction::Vertical)
}

fn construct_split(re: Rect, ra: Ratio, d: Direction) -> Vec<Rect> {
    Layout::default()
        .direction(d)
        .constraints([Constraint::Percentage(ra.0), Constraint::Percentage(ra.1)].as_ref())
        .split(re)
}
