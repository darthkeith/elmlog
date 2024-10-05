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
    StartDelete,
    AppendDelete(char),
    DecrementIndex,
    IncrementIndex,
    StartMerge,
    SelectFirst,
    SelectSecond,
    Submit,
    Cancel,
    Quit,
    Nothing,
}

// Convert a `key` press into a Message based on the current `mode`.
fn key_to_message(mode: &Mode, key: KeyCode) -> Message {
    match (mode, key) {
        (Mode::Normal, KeyCode::Char('i')) => Message::StartInput,
        (Mode::Normal, KeyCode::Char('d')) => Message::StartDelete,
        (Mode::Normal, KeyCode::Char('m')) => Message::StartMerge,
        (Mode::Normal, KeyCode::Char('q')) => Message::Quit,
        (Mode::Input(_), KeyCode::Char(c)) => {
            Message::EditInput(Edit::AppendChar(c))
        }
        (Mode::Input(_), KeyCode::Backspace) => {
            Message::EditInput(Edit::PopChar)
        }
        (Mode::Delete(_), KeyCode::Char(c)) => Message::AppendDelete(c),
        (Mode::Delete(_), KeyCode::Up) => Message::DecrementIndex,
        (Mode::Delete(_), KeyCode::Down) => Message::IncrementIndex,
        (Mode::Merge, KeyCode::Up) => Message::SelectFirst,
        (Mode::Merge, KeyCode::Down) => Message::SelectSecond,
        (Mode::Input(_) | Mode::Delete(_), KeyCode::Enter) => Message::Submit,
        (Mode::Input(_) | Mode::Delete(_), KeyCode::Esc) => Message::Cancel,
        _ => Message::Nothing,
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

