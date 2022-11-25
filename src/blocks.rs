use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
};

pub fn bold_block(title: &'static str, border_fg: Color) -> Block {
    let border_style = Style::default().fg(border_fg).add_modifier(Modifier::BOLD);
    block_constructor(title, Some(border_style), Some(Borders::ALL))
}

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
