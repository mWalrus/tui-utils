use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::error::Error;
use tui::{
    style::Color,
    widgets::{List, ListItem},
};
use tui_utils::{
    blocks,
    component::Component,
    keys::{key_match, Keybind},
    state::{Boundary, BoundedState, StateWrap},
    style, term, LIST_HIGHLIGHT_SYMBOL,
};

// example of how to define keybinds
struct KeyBinds {
    quit: Keybind,
    up: Keybind,
    down: Keybind,
    add: Keybind,
    top: Keybind,
    bottom: Keybind,
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
            up: Keybind {
                code: KeyCode::Up,
                modifiers: KeyModifiers::empty(),
            },
            down: Keybind {
                code: KeyCode::Down,
                modifiers: KeyModifiers::empty(),
            },
            add: Keybind {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::empty(),
            },
            top: Keybind {
                code: KeyCode::Char('t'),
                modifiers: KeyModifiers::empty(),
            },
            bottom: Keybind {
                code: KeyCode::Char('b'),
                modifiers: KeyModifiers::empty(),
            },
        }
    }
}

// example application struct
struct App {
    view: View,
}

enum AppMessage {
    Idle,
    Exit,
}

struct View {
    items: Vec<String>,
    state: BoundedState,
    binds: KeyBinds,
}

impl Component for View {
    type Message = AppMessage;
    fn draw<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, _dim: bool) {
        // map the items into `ListItem`s
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|i| ListItem::new(i.clone()))
            .collect();

        // create the list with `tui_utils` helpers
        let list = List::new(items)
            .block(blocks::default_block("List", Color::White))
            .highlight_style(style::highlight_style())
            .highlight_symbol(LIST_HIGHLIGHT_SYMBOL);

        f.render_stateful_widget(list, f.size(), self.state.inner());
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<Self::Message, Box<dyn Error>> {
        if key_match(&key, &self.binds.quit) {
            // unfocus the component if the quit key is pressed
            return Ok(AppMessage::Exit);
        } else if key_match(&key, &self.binds.up) {
            self.state.prev();
        } else if key_match(&key, &self.binds.down) {
            self.state.next();
        } else if key_match(&key, &self.binds.add) {
            // add a new item to the `items` member
            self.items.push(format!("Item {}", self.items.len() + 1));
            // update the boundary accordingly
            self.state.update_boundary_from_vec(&self.items);
        } else if key_match(&key, &self.binds.top) {
            self.state.first();
        } else if key_match(&key, &self.binds.bottom) {
            self.state.last();
        }
        Ok(AppMessage::Idle)
    }
}

fn main() {
    // dummy data for the list
    let items: Vec<String> = vec![
        "Item 1", "Item 2", "Item 3", "Item 4", "Item 5", "Item 6", "Item 7", "Item 8", "Item 9",
        "Item 10", "Item 11", "Item 12", "Item 13", "Item 14", "Item 15", "Item 16", "Item 17",
        "Item 18", "Item 19", "Item 20", "Item 21", "Item 22", "Item 23", "Item 24", "Item 25",
        "Item 26", "Item 27", "Item 28", "Item 29", "Item 30",
    ]
    .into_iter()
    .map(String::from) // conversion just for the example
    .collect();

    // take the upper bounds before moving `items` into `view`
    let boundary = Boundary::from(&items);

    // since selections can fail the bounds check that happens before
    // setting the selection, we have to handle the error.
    let state = match BoundedState::with_selection(boundary, StateWrap::Enable, 0) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let view = View {
        items,
        state,
        binds: KeyBinds::new(),
    };
    let mut app = App { view };

    // init the terminal
    let mut terminal = term::init().unwrap();

    loop {
        // draw the ui first
        terminal.draw(|f| app.view.draw(f, false)).unwrap();

        // then handle input events
        match term::poll_event() {
            Ok(Some(Event::Key(ev))) => match app.view.handle_input(ev) {
                Ok(AppMessage::Idle) => {}
                Ok(AppMessage::Exit) => break, // exit on focus release
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
