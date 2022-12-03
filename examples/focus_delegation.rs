use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::{error::Error, rc::Rc};
use tui::{
    style::Color,
    widgets::{Clear, Paragraph},
};
use tui_utils::{
    blocks::{self, Dim},
    component::Component,
    keys::{key_match, Keybind, Shared},
    rect, term,
};

// Example of how to define keybinds.
// Here we derive `Shared` which will allow us to create
// a single instance of `Keymap` using `Keymap::shared()` which
// gives us an `Rc<Keymap>`.
#[derive(Shared)]
struct Keymap {
    quit: Keybind,
    modal_open: Keybind, // add more here
}

impl Default for Keymap {
    fn default() -> Self {
        Self {
            // simple quit bind
            quit: Keybind {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::empty(),
            },
            modal_open: Keybind {
                code: KeyCode::Char(' '),
                modifiers: KeyModifiers::empty(),
            },
        }
    }
}

// example application struct
struct App {
    main: Main,
    modal: Modal,
    // dictates which side has focus
    state: AppState,
}

enum AppState {
    Main,
    Modal,
}

#[derive(Default)]
enum AppMessage {
    #[default]
    Idle,
    ShowModal,
    Back,
    Exit,
}

struct Main {
    keys: Rc<Keymap>,
}

struct Modal {
    keys: Rc<Keymap>,
}

impl Component for Main {
    type Message = AppMessage;
    fn draw<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, dim: bool) {
        let p = Paragraph::new("This is the main component")
            .block(blocks::default_block("Main", Color::White).dim(dim));
        f.render_widget(p, f.size());
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<Self::Message, Box<dyn Error>> {
        if key_match(&key, &self.keys.quit) {
            // unfocus the component if the quit key is pressed
            return Ok(AppMessage::Exit);
        } else if key_match(&key, &self.keys.modal_open) {
            // report that we want to open the modal
            return Ok(AppMessage::ShowModal);
        }
        Ok(AppMessage::Idle)
    }
}

impl Component for Modal {
    type Message = AppMessage;
    fn draw<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, _dim: bool) {
        let rect = rect::centered_rect(f.size());
        let p =
            Paragraph::new("This is the modal").block(blocks::default_block("Modal", Color::White));

        // clear the space the modal will take
        f.render_widget(Clear, rect);

        // render the widget
        f.render_widget(p, rect);
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<Self::Message, Box<dyn Error>> {
        if key_match(&key, &self.keys.quit) {
            // unfocus the component if the quit key is pressed
            return Ok(AppMessage::Back);
        }
        Ok(AppMessage::Idle)
    }
}

fn main() {
    // Create a shared keymap.
    // This allows us to take a reference to the keys and delegate
    // them across our components instead of cloning the entire
    // `Keymap` object each time or initializing a new instance.
    let keys = Keymap::shared();

    // clone gives us a pointer to the above created keymap
    let main = Main { keys: keys.clone() };
    let modal = Modal { keys: keys.clone() };

    let mut app = App {
        main,
        modal,
        state: AppState::Main,
    };

    // init the terminal
    let mut terminal = term::init().unwrap();

    loop {
        // draw the ui first
        terminal
            .draw(|f| {
                match app.state {
                    // conditional dimming
                    AppState::Main => app.main.draw(f, false),
                    AppState::Modal => {
                        // conditional dimming
                        app.main.draw(f, true);
                        app.modal.draw(f, false);
                    }
                }
            })
            .unwrap();

        // then handle input events
        let event_outcome = match term::poll_event() {
            Ok(Some(Event::Key(ev))) => match app.state {
                AppState::Main => app.main.handle_input(ev),
                AppState::Modal => app.modal.handle_input(ev),
            },
            // other term events, we dont handle them in this example
            Ok(Some(_)) => Ok(AppMessage::Idle),
            // no events were found
            Ok(None) => Ok(AppMessage::Idle),
            // something went wrong
            Err(e) => Err(e.into()),
        };

        match event_outcome {
            Ok(AppMessage::ShowModal) => app.state = AppState::Modal,
            Ok(AppMessage::Back) => app.state = AppState::Main, // hide the modal
            Ok(AppMessage::Exit) => break,                      // exit on focus release
            Ok(AppMessage::Idle) => {}
            Err(e) => {
                term::restore_with_err(e).unwrap();
                return;
            }
        }
    }

    // always restore the terminal before exiting
    term::restore().unwrap()
}
