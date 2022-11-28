use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::error::Error;
use tui::{style::Color, widgets::Paragraph};
use tui_utils::{
    blocks,
    component::{Component, Focus},
    keys::{key_match, Keybind},
    split::{self, Ratio},
    term,
};

// example of how to define keybinds
struct KeyBinds {
    quit: Keybind,
    // add more here
}

impl KeyBinds {
    fn new() -> Self {
        Self {
            // simple quit bind
            quit: Keybind {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::empty(),
            },
        }
    }
}

// example application struct
struct App {
    split: Split,
}

struct Split {
    text_left: String,
    text_right: String,
    binds: KeyBinds,
}

impl Component for Split {
    fn draw<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, _dim: bool) {
        // define a new ratio of 50/50
        let ratio = Ratio::new(50, 50);
        // create a vertical split using the frame size and ratio
        let chunks = split::v_split(f.size(), ratio);

        // paragraphs as an example using default blocks
        let left_p =
            Paragraph::new(&*self.text_left).block(blocks::default_block("Left", Color::White));
        let right_p =
            Paragraph::new(&*self.text_right).block(blocks::default_block("Right", Color::White));

        // render the widgets
        f.render_widget(left_p, chunks[0]);
        f.render_widget(right_p, chunks[1]);
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<Focus, Box<dyn Error>> {
        if key_match(&key, &self.binds.quit) {
            // unfocus the component if the quit key is pressed
            Ok(Focus::Release)
        } else {
            // otherwise we do nothing
            Ok(Focus::Keep)
        }
    }
}

fn main() {
    // init the terminal
    let mut terminal = term::init().unwrap();

    let split = Split {
        text_left: String::from("left side"),
        text_right: String::from("right side"),
        binds: KeyBinds::new(),
    };
    let mut app = App { split };

    loop {
        // draw the ui first
        terminal.draw(|f| app.split.draw(f, false)).unwrap();

        // then handle input events
        match term::poll_event() {
            Ok(Some(Event::Key(ev))) => match app.split.handle_input(ev) {
                Ok(Focus::Keep) => {}
                Ok(Focus::Release) => break, // exit on unfocus
                Err(e) => {
                    term::restore_with_err(e).unwrap();
                    return;
                }
            },
            // other term events, we dont handle them in this example
            Ok(Some(_)) => {}
            // no events were found
            Ok(None) => {}
            // something went wrong
            Err(e) => {
                term::restore_with_err(e.into()).unwrap();
                return;
            }
        }
    }

    // always restore the terminal before exiting
    term::restore().unwrap()
}
