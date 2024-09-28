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
    SubmitInput,
    Cancel,
    Quit,
    Nothing,
}

// Convert a `key` press into a Message based on the current `mode`.
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

