use tui::style::{Color, Modifier, Style};

pub fn highlight_style() -> Style {
    Style::default()
        .bg(Color::Indexed(8))
        .add_modifier(Modifier::BOLD)
}
