use std::io;

use crossterm::event::{self, KeyCode, KeyEventKind};

use crate::model::{Choice, InputState, Mode};

// A message sent in Normal mode.
pub enum NormalMsg {
    StartInput,
    StartSelect,
    StartCompare,
    Quit,
}

// A message sent in Input mode.
pub enum InputMsg {
    Append(char),
    PopChar,
    Submit,
}

// A message sent in Select mode.
pub enum SelectMsg {
    Append(char),
    Decrement,
    Increment,
    Confirm,
}

// A message sent in Selected mode.
pub enum SelectedMsg {
    Edit,
    Delete,
}

// A message sent in Compare mode.
pub enum CompareMsg {
    Toggle,
    Confirm,
}

// Represents changes to be made to the model, grouped by mode.
pub enum Message {
    Normal(NormalMsg),
    Input(InputMsg, InputState),
    Select(SelectMsg, usize),
    Selected(SelectedMsg, usize),
    Compare(CompareMsg, Choice),
    Continue(Mode),
}

// Return to Normal mode on Esc, otherwise continue in the given `mode`.
fn default(key: KeyCode, mode: Mode) -> Message {
    Message::Continue(match key {
        KeyCode::Esc => Mode::Normal,
        _ => mode,
    })
}

// Map a `key` to a Message in Normal mode.
fn to_normal_msg(key: KeyCode) -> Message {
    let normal_msg = match key {
        KeyCode::Char('i') => NormalMsg::StartInput,
        KeyCode::Char('s') => NormalMsg::StartSelect,
        KeyCode::Char('c') => NormalMsg::StartCompare,
        KeyCode::Char('q') => NormalMsg::Quit,
        _ => return Message::Continue(Mode::Normal),
    };
    Message::Normal(normal_msg)
}

// Map a `key` to a Message in Input mode.
fn to_input_msg(key: KeyCode, state: InputState) -> Message {
    let input_msg = match key {
        KeyCode::Char(c) => InputMsg::Append(c),
        KeyCode::Backspace => InputMsg::PopChar,
        KeyCode::Enter if !state.input.is_empty() => InputMsg::Submit,
        _ => return default(key, Mode::Input(state)),
    };
    Message::Input(input_msg, state)
}

// Map a `key` to a Message in Select mode.
fn to_select_msg(key: KeyCode, index: usize) -> Message {
    let select_msg = match key {
        KeyCode::Char(c) => match c {
            'k' => SelectMsg::Decrement,
            'j' => SelectMsg::Increment,
            _ => SelectMsg::Append(c),
        }
        KeyCode::Up => SelectMsg::Decrement,
        KeyCode::Down => SelectMsg::Increment,
        KeyCode::Enter => SelectMsg::Confirm,
        _ => return default(key, Mode::Select(index)),
    };
    Message::Select(select_msg, index)
}

// Map a `key` to a Message in Selected mode.
fn to_selected_msg(key: KeyCode, index: usize) -> Message {
    let selected_msg = match key {
        KeyCode::Char('e') => SelectedMsg::Edit,
        KeyCode::Char('d') => SelectedMsg::Delete,
        _ => return default(key, Mode::Selected(index)),
    };
    Message::Selected(selected_msg, index)
}

// Map a `key` to a Message in Compare mode.
fn to_compare_msg(key: KeyCode, choice: Choice) -> Message {
    let compare_msg = match key {
        KeyCode::Char(' ') => CompareMsg::Toggle,
        KeyCode::Enter => CompareMsg::Confirm,
        _ => return default(key, Mode::Compare(choice)),
    };
    Message::Compare(compare_msg, choice)
}

// Map a pressed `key` to a Message based on the current `mode`.
fn key_to_message(mode: Mode, key: KeyCode) -> Message {
    match mode {
        Mode::Normal => to_normal_msg(key),
        Mode::Input(input) => to_input_msg(key, input),
        Mode::Select(index) => to_select_msg(key, index),
        Mode::Selected(index) => to_selected_msg(key, index),
        Mode::Compare(choice) => to_compare_msg(key, choice),
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

