use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
};

/// Dim a blocks borders and contents
pub trait Dim {
    fn dim(self, dim: bool) -> Block<'static>;
}

impl Dim for Block<'static> {
    // FIXME: update this api to not need the dim parameter
    fn dim(self, dim: bool) -> Self {
        if !dim {
            return self;
        }
        let style = Style::default().fg(Color::Indexed(8));
        self.border_style(style).style(style)
    }
}

// FIXME: refactor all the below into cleaner, faster code
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

/// Basically same thing as just constructing the block
pub fn block_constructor(
    title: &'static str,
    border_style: Option<Style>,
    borders: Option<Borders>,
) -> Block {
    let mut block = Block::default().title(title);

    if let Some(s) = border_style {
        block = block.border_style(s);
    }

    if let Some(b) = borders {
        block = block.borders(b);
    }

    block
}
