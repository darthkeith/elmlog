mod heap;
mod ui;

use std::io;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::ui::view;

enum Mode {
    Normal,
    Input(String),
}

struct Model {
    heap: heap::Heap,
    mode: Mode,
    quit: bool,
}

enum Edit {
    AppendChar(char),
    PopChar,
}

enum Message {
    StartInput,
    EditInput(Edit),
    SubmitInput,
    Cancel,
    Quit,
    Nothing,
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

fn key_to_message(mode: &Mode, key: KeyCode) -> Message {
    match mode {
        Mode::Normal => match key {
            KeyCode::Char('i') => Message::StartInput,
            KeyCode::Char('q') => Message::Quit,
            _ => Message::Nothing,
        }
        Mode::Input(_) => match key {
            KeyCode::Char(c) => Message::EditInput(Edit::AppendChar(c)),
            KeyCode::Backspace => Message::EditInput(Edit::PopChar),
            KeyCode::Enter => Message::SubmitInput,
            KeyCode::Esc => Message::Cancel,
            _ => Message::Nothing,
        }
    }
}

fn handle_event(mode: &Mode) -> io::Result<Message> {
    let event::Event::Key(key) = event::read()? else {
        return Ok(Message::Nothing);
    };
    if key.kind != KeyEventKind::Press {
        return Ok(Message::Nothing);
    }
    Ok(key_to_message(mode, key.code))
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

