use tui::style::{Color, Modifier, Style};

/// Simple highlight style to be used with stateful widgets
pub fn highlight_style() -> Style {
    Style::default()
        .bg(Color::Indexed(8))
        .add_modifier(Modifier::BOLD)
}
