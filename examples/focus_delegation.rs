use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::{error::Error, rc::Rc};
use tui::{
    style::Color,
    widgets::{Clear, Paragraph},
};
use tui_utils::{
    blocks::{self, Dim},
    component::Component,
    keys::{key_match, Keybind, SharedKeys},
    rect, term,
};

// Example of how to define keybinds.
// Here we derive `SharedKeys` which will allow us to create
// a single instance of `Keymap` using `Keymap::shared()` which
// gives us an `Rc<Keymap>`.
#[derive(SharedKeys)]
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
    show_modal: bool,
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
        show_modal: false,
    };

    // init the terminal
    let mut terminal = term::init().unwrap();

    loop {
        // draw the ui first
        terminal
            .draw(|f| {
                if !app.show_modal {
                    // conditional dimming
                    app.main.draw(f, false);
                } else {
                    // conditional dimming
                    app.main.draw(f, true);
                    app.modal.draw(f, false);
                }
            })
            .unwrap();

        // then handle input events
        match term::poll_event() {
            Ok(Some(Event::Key(ev))) => {
                if !app.show_modal {
                    match app.main.handle_input(ev) {
                        Ok(AppMessage::ShowModal) => app.show_modal = true,
                        Ok(AppMessage::Exit) => break, // exit on focus release
                        Err(e) => {
                            term::restore_with_err(e).unwrap();
                            return;
                        }
                        _ => {}
                    }
                } else {
                    match app.modal.handle_input(ev) {
                        Ok(AppMessage::Back) => app.show_modal = false, // hide the modal
                        Err(e) => {
                            term::restore_with_err(e).unwrap();
                            return;
                        }
                        _ => {}
                    }
                }
            }
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
