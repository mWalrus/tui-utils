use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
};

/// Dim a blocks borders and contents
pub trait Dim {
    fn dim(self, dim: bool) -> Block<'static>;
}

impl Dim for Block<'static> {
    fn dim(self, dim: bool) -> Self {
        if dim {
            let style = Style::default().fg(Color::Indexed(8));
            self.border_style(style).style(style)
        } else {
            self
        }
    }
}

/// Construct a block with a bold border and title
pub fn bold_block(title: &'static str, border_fg: Color) -> Block {
    let border_style = Style::default().fg(border_fg).add_modifier(Modifier::BOLD);
    block_constructor(title, Some(border_style), Some(Borders::ALL))
}

/// Construct a block with a border and title
pub fn default_block(title: &'static str, border_fg: Color) -> Block {
    let border_style = Style::default().fg(border_fg);
    block_constructor(title, Some(border_style), Some(Borders::ALL))
}

pub fn block_constructor(
    title: &'static str,
    border_style: Option<Style>,
    borders: Option<Borders>,
) -> Block {
    let mut block = Block::default().borders(Borders::ALL).title(title);

    if let Some(s) = border_style {
        block = block.border_style(s);
    }

    if let Some(b) = borders {
        block = block.borders(b);
    }

    block
}
