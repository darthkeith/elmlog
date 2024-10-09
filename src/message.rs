use std::io;

use crossterm::event::{self, KeyCode, KeyEventKind};

use crate::model::Mode;

/// Type indicating changes to be made to the model.
pub enum Message {
    StartInput,
    InputAppend(String, char),
    InputPopChar(String),
    Insert(String),
    StartSelect,
    SelectAppend(usize, char),
    SelectDecrement(usize),
    SelectIncrement(usize),
    Delete(usize),
    StartMerge,
    Merge(bool),
    Continue(Mode),
    Quit,
}

// Convert a `key` press into a Message based on the current `mode`.
fn key_to_message(mode: Mode, key: KeyCode) -> Message {
    match mode {
        Mode::Normal => match key {
            KeyCode::Char('i') => Message::StartInput,
            KeyCode::Char('s') => Message::StartSelect,
            KeyCode::Char('m') => Message::StartMerge,
            KeyCode::Char('q') => Message::Quit,
            _ => Message::Continue(Mode::Normal),
        }
        Mode::Input(input) => match key {
            KeyCode::Char(c) => Message::InputAppend(input, c),
            KeyCode::Backspace => Message::InputPopChar(input),
            KeyCode::Enter => Message::Insert(input),
            KeyCode::Esc => Message::Continue(Mode::Normal),
            _ => Message::Continue(Mode::Input(input)),
        }
        Mode::Select(index) => match key {
            KeyCode::Char('d') => Message::Delete(index),
            KeyCode::Char(c) => Message::SelectAppend(index, c),
            KeyCode::Up => Message::SelectDecrement(index),
            KeyCode::Down => Message::SelectIncrement(index),
            KeyCode::Esc => Message::Continue(Mode::Normal),
            _ => Message::Continue(Mode::Select(index)),
        }
        Mode::Merge => match key {
            KeyCode::Up => Message::Merge(true),
            KeyCode::Down => Message::Merge(false),
            KeyCode::Esc => Message::Continue(Mode::Normal),
            _ => Message::Continue(Mode::Merge),
        }
    }
}

/// Convert a user input event into a Message based on the current `mode`.
pub fn handle_event(mode: Mode) -> io::Result<Message> {
    let event::Event::Key(key) = event::read()? else {
        return Ok(Message::Continue(mode));
    };
    if key.kind != KeyEventKind::Press {
        return Ok(Message::Continue(mode));
    }
    Ok(key_to_message(mode, key.code))
}

