use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::error::Error;
use tui::{
    style::Color,
    widgets::{Clear, Paragraph},
};
use tui_utils::{
    blocks::{self, Dim},
    component::Component,
    keys::{key_match, Keybind},
    rect, term,
};

// example of how to define keybinds
struct KeyBinds {
    quit: Keybind,
    modal_open: Keybind, // add more here
}

impl KeyBinds {
    fn new() -> Self {
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

enum AppMessage {
    Idle,
    ShowModal,
    Back,
    Exit,
}

struct Main {
    binds: KeyBinds,
}

struct Modal {
    binds: KeyBinds,
}

impl Component for Main {
    type Message = AppMessage;
    fn draw<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, dim: bool) {
        let p = Paragraph::new("This is the main component")
            .block(blocks::default_block("Main", Color::White).dim(dim));
        f.render_widget(p, f.size());
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<Self::Message, Box<dyn Error>> {
        if key_match(&key, &self.binds.quit) {
            // unfocus the component if the quit key is pressed
            return Ok(AppMessage::Exit);
        } else if key_match(&key, &self.binds.modal_open) {
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
        if key_match(&key, &self.binds.quit) {
            // unfocus the component if the quit key is pressed
            return Ok(AppMessage::Back);
        }
        Ok(AppMessage::Idle)
    }
}

fn main() {
    let main = Main {
        binds: KeyBinds::new(),
    };
    let modal = Modal {
        binds: KeyBinds::new(),
    };
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
                // draw either of the sides
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
