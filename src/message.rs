use std::io;

use crossterm::event::{self, KeyCode, KeyEventKind};

use crate::model::Mode;

/// Represents types of edits made to the user input string.
pub enum Edit {
    AppendChar(char),
    PopChar,
}

/// Type indicating changes to be made to the model.
pub enum Message {
    StartInput,
    EditInput(Edit),
    Insert,
    StartSelect,
    AppendSelect(char),
    DecrementIndex,
    IncrementIndex,
    Delete,
    StartMerge,
    SelectFirst,
    SelectSecond,
    Cancel,
    Quit,
    Nothing,
}

// Convert a `key` press into a Message based on the current `mode`.
fn key_to_message(mode: &Mode, key: KeyCode) -> Message {
    match mode {
        Mode::Normal => match key {
            KeyCode::Char('i') => Message::StartInput,
            KeyCode::Char('s') => Message::StartSelect,
            KeyCode::Char('m') => Message::StartMerge,
            KeyCode::Char('q') => Message::Quit,
            _ => Message::Nothing,
        }
        Mode::Input(_) => match key {
            KeyCode::Char(c) => {
                Message::EditInput(Edit::AppendChar(c))
            }
            KeyCode::Backspace => {
                Message::EditInput(Edit::PopChar)
            }
            KeyCode::Enter => Message::Insert,
            KeyCode::Esc => Message::Cancel,
            _ => Message::Nothing,
        }
        Mode::Select(_) => match key {
            KeyCode::Char('d') => Message::Delete,
            KeyCode::Char(c) => Message::AppendSelect(c),
            KeyCode::Up => Message::DecrementIndex,
            KeyCode::Down => Message::IncrementIndex,
            KeyCode::Esc => Message::Cancel,
            _ => Message::Nothing,
        }
        Mode::Merge => match key {
            KeyCode::Up => Message::SelectFirst,
            KeyCode::Down => Message::SelectSecond,
            _ => Message::Nothing,
        }
    }
}

/// Convert a user input event into a Message based on the current `mode`.
pub fn handle_event(mode: &Mode) -> io::Result<Message> {
    let event::Event::Key(key) = event::read()? else {
        return Ok(Message::Nothing);
    };
    if key.kind != KeyEventKind::Press {
        return Ok(Message::Nothing);
    }
    Ok(key_to_message(mode, key.code))
}

