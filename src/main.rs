mod heap;
mod msg;
mod ui;

use std::io;

use ratatui::DefaultTerminal;

use crate::ui::view;
use crate::msg::{handle_event, Edit, Message};

enum Mode {
    Normal,
    Input(String),
}

struct Model {
    heap: heap::Heap,
    mode: Mode,
    quit: bool,
}

impl Model {
    fn new() -> Self {
        Model {
            heap: heap::empty(),
            mode: Mode::Normal,
            quit: false,
        }
    }
}

// Trim the `input` string and return the result if non-empty.
fn trim_input(input: String) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

// Update the `model` based on the message.
fn update(mut model: Model, msg: Message) -> Model {
    match msg {
        Message::StartInput => model.mode = Mode::Input(String::new()),
        Message::EditInput(edit) => {
            if let Mode::Input(ref mut input) = model.mode {
                match edit {
                    Edit::AppendChar(c) => input.push(c),
                    Edit::PopChar => { input.pop(); }
                }
            }
        }
        Message::SubmitInput => {
            if let Mode::Input(input) = model.mode {
                if let Some(label) = trim_input(input) {
                    model.heap = model.heap.prepend(label);
                }
                model.mode = Mode::Normal;
            }
        }
        Message::Cancel => model.mode = Mode::Normal,
        Message::Quit => model.quit = true,
        Message::Nothing => (),
    }
    model
}

fn main_loop(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut model = Model::new();
    while !model.quit {
        terminal.draw(|frame| view(&model, frame))?;
        let msg = handle_event(&model.mode)?;
        model = update(model, msg);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let result = main_loop(terminal);
    ratatui::restore();
    result
}

