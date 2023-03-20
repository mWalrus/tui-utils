use ratatui::layout::Rect;

/// Produces a centered rect half the size of the width as well as the height.
pub fn centered_rect(size: Rect) -> Rect {
    let width = size.width / 2;
    let x = width / 2;
    let height = size.height / 2;
    let y = height / 2;

    Rect {
        x,
        y,
        width,
        height,
    }
}
