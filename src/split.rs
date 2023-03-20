use std::rc::Rc;

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Set the percentage ratio for the split
#[derive(Debug)]
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
        // no need to normalize the ratio
        if first + second <= 100 {
            return Self(first, second);
        }

        // cast to f32
        let first = first as f32;
        let second = second as f32;

        // normalize the ratio
        let norm = 1.0 / (first + second);
        let first = ((first * norm) * 100.0).ceil();
        let second = ((second * norm) * 100.0).floor();

        Self(first as u16, second as u16)
    }
}

/// Generate a vertically split layout in a rect with a defined ratio
pub fn v_split(rect: Rect, ratio: Ratio) -> Rc<[Rect]> {
    construct_split(rect, ratio, Direction::Horizontal)
}

/// Generate a horizontally split layout in a rect with a defined ratio
pub fn h_split(rect: Rect, ratio: Ratio) -> Rc<[Rect]> {
    construct_split(rect, ratio, Direction::Vertical)
}

fn construct_split(re: Rect, ra: Ratio, d: Direction) -> Rc<[Rect]> {
    Layout::default()
        .direction(d)
        .constraints([Constraint::Percentage(ra.0), Constraint::Percentage(ra.1)].as_ref())
        .split(re)
}

#[cfg(test)]
mod tests {
    use crate::split::Ratio;

    #[test]
    fn normalize_input() {
        let r = Ratio::new(150, 75);
        assert!(r.0 + r.1 == 100);
    }

    #[test]
    fn zero_zero_should_remain() {
        let r = Ratio::new(0, 0);
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 0);
    }

    #[test]
    fn normalize_if_sum_gt_100() {
        let r = Ratio::new(50, 60);
        assert!(r.0 + r.1 == 100);
    }
    #[test]
    fn retain_ratio_if_sum_is_100() {
        let r = Ratio::new(40, 60);
        assert_eq!(r.0, 40);
        assert_eq!(r.1, 60);
        assert!(r.0 + r.1 == 100);
    }
    #[test]
    fn retain_ratio_if_lt_100() {
        let r = Ratio::new(20, 20);
        assert_eq!(r.0, 20);
        assert_eq!(r.1, 20);
        assert!(r.0 + r.1 == 40);
    }
}
