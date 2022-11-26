use crossterm::event::{Event, KeyCode, KeyModifiers};
use tui::{style::Color, widgets::Paragraph};
use tui_utils::{
    blocks,
    component::Component,
    keymap::{key_match, Keybind},
    split::{self, Ratio},
    term,
};

// FIXME: comment code better

struct Split {
    text_left: String,
    text_right: String,
    binds: KeyBinds,
}

impl Component for Split {
    fn draw<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, _dim: bool) {
        let ratio = Ratio::new(50, 50);
        let chunks = split::v_split(f.size(), ratio);
        let left_p =
            Paragraph::new(&*self.text_left).block(blocks::default_block("Left", Color::White));
        let right_p =
            Paragraph::new(&*self.text_right).block(blocks::default_block("Right", Color::White));
        f.render_widget(left_p, chunks[0]);
        f.render_widget(right_p, chunks[1]);
    }

    fn handle_input(&mut self, key: crossterm::event::KeyEvent) -> anyhow::Result<()> {
        // find out a better way to handle quit
        if key_match(&key, &self.binds.quit) {
            Err(anyhow::anyhow!("Exit application"))?
        }
        Ok(())
    }
}

struct App {
    split: Split,
}

struct KeyBinds {
    quit: Keybind,
}

impl KeyBinds {
    fn new() -> Self {
        Self {
            quit: Keybind {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::empty(),
            },
        }
    }
}

fn main() {
    let mut terminal = term::init_terminal().unwrap();

    let split = Split {
        text_left: String::from("left side"),
        text_right: String::from("right side"),
        binds: KeyBinds::new(),
    };
    let mut app = App { split };

    loop {
        terminal.draw(|f| app.split.draw(f, false)).unwrap();

        match term::poll_event() {
            Ok(Some(Event::Key(ev))) => {
                match app.split.handle_input(ev) {
                    Ok(()) => {}
                    Err(_) => break, // break for convenience
                }
            }
            Ok(None) | Ok(Some(_)) => {}
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        }
    }

    term::restore_terminal().unwrap()
}
