use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::error::Error;
use tui::{style::Color, widgets::Paragraph};
use tui_utils::{
    blocks,
    component::Component,
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

#[derive(Default)]
enum AppMessage {
    #[default]
    Idle,
    Exit,
}

struct Split {
    text_left: String,
    text_right: String,
    binds: KeyBinds,
}

impl Component for Split {
    type Message = AppMessage;
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

    fn handle_input(&mut self, key: KeyEvent) -> Result<Self::Message, Box<dyn Error>> {
        if key_match(&key, &self.binds.quit) {
            // unfocus the component if the quit key is pressed
            Ok(AppMessage::Exit)
        } else {
            // otherwise we do nothing
            Ok(AppMessage::Idle)
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
        let event_outcome = match term::poll_event() {
            Ok(Some(Event::Key(ev))) => app.split.handle_input(ev),
            // other term events, we dont handle them in this example
            Ok(Some(_)) => Ok(AppMessage::Idle),
            // no events were found
            Ok(None) => Ok(AppMessage::Idle),
            // something went wrong
            Err(e) => Err(e.into()),
        };

        match event_outcome {
            Ok(AppMessage::Idle) => {}
            Ok(AppMessage::Exit) => break,
            Err(e) => {
                term::restore_with_err(e).unwrap();
                return;
            }
        }
    }

    // always restore the terminal before exiting
    term::restore().unwrap()
}
