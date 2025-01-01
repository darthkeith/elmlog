use std::{
    io::Result,
    path::PathBuf,
};

use crossterm::event::{self, KeyCode, KeyEventKind};

use crate::{
    io::LoadState,
    model::{
        CompareState,
        ConfirmState,
        FilenameState,
        InputState,
        Mode,
        Model,
        PostSaveAction,
        SaveState,
        SessionState,
    },
};

/// A message sent in Load mode.
pub enum LoadMsg {
    Decrement,
    Increment,
    Open,
    New,
    Rename,
    Delete,
    Quit,
}

/// A message sent in Normal mode.
pub enum NormalMsg {
    Input,
    Select,
    Compare,
    Load,
    Quit,
}

/// Type of edit to apply to the user input text.
pub enum InputEdit {
    Append(char),
    PopChar,
}

/// A message sent in Input mode.
pub enum InputMsg {
    Edit(InputEdit),
    Submit,
    Cancel,
}

/// A message sent in Select mode.
pub enum SelectMsg {
    Append(char),
    Decrement,
    Increment,
    Confirm,
}

/// A message sent in Selected mode.
pub enum SelectedMsg {
    Edit,
    Delete,
}

/// A message sent in Compare mode.
pub enum CompareMsg {
    Toggle,
    Confirm,
}

/// A message sent in Save mode.
pub enum SaveMsg {
    Toggle,
    Confirm,
}

/// A message indicating changes to be made to the model.
pub enum Message {
    Confirm(bool, ConfirmState),
    Load(LoadMsg, LoadState),
    Normal(NormalMsg),
    Input(InputMsg, InputState),
    Select(SelectMsg, usize),
    Selected(SelectedMsg, usize),
    Compare(CompareMsg, CompareState),
    Save(SaveMsg, SaveState),
    Continue(Mode),
}

/// A message indicating an IO action to perform.
pub enum Command {
    None(Model),
    Load,
    InitSession(PathBuf),
    CheckFileExists(SessionState, FilenameState),
    Rename(SessionState, String, LoadState),
    SaveNew(SessionState, String, PostSaveAction),
    Save(SessionState, PostSaveAction),
    DeleteFile(LoadState),
    Quit,
}

// Map a `key` to a Message in Confirm mode.
fn to_confirm_msg(key: KeyCode, confirm_state: ConfirmState) -> Message {
    let confirm = match key {
        KeyCode::Enter => true,
        KeyCode::Esc => false,
        _ => return Message::Continue(Mode::Confirm(confirm_state)),
    };
    Message::Confirm(confirm, confirm_state)
}

// Map a `key` to a Message in Load mode.
fn to_load_msg(key: KeyCode, load_state: LoadState) -> Message {
    let load_msg = match key {
        KeyCode::Char('k') | KeyCode::Up => LoadMsg::Decrement,
        KeyCode::Char('j') | KeyCode::Down => LoadMsg::Increment,
        KeyCode::Enter => LoadMsg::Open,
        KeyCode::Char('n') => LoadMsg::New,
        KeyCode::Char('r') => LoadMsg::Rename,
        KeyCode::Char('d') => LoadMsg::Delete,
        KeyCode::Char('q') => LoadMsg::Quit,
        _ => return Message::Continue(Mode::Load(load_state)),
    };
    Message::Load(load_msg, load_state)
}

// Map a `key` to a Message in Normal mode.
fn to_normal_msg(key: KeyCode) -> Message {
    let normal_msg = match key {
        KeyCode::Char('a') => NormalMsg::Input,
        KeyCode::Char('s') => NormalMsg::Select,
        KeyCode::Char('c') => NormalMsg::Compare,
        KeyCode::Char('l') => NormalMsg::Load,
        KeyCode::Char('q') => NormalMsg::Quit,
        _ => return Message::Continue(Mode::Normal),
    };
    Message::Normal(normal_msg)
}

// Map a `key` to a Message in Input mode.
fn to_input_msg(key: KeyCode, input_state: InputState) -> Message {
    let input_msg = match key {
        KeyCode::Char(c) => InputMsg::Edit(InputEdit::Append(c)),
        KeyCode::Backspace => InputMsg::Edit(InputEdit::PopChar),
        KeyCode::Enter => InputMsg::Submit,
        KeyCode::Esc => InputMsg::Cancel,
        _ => return Message::Continue(Mode::Input(input_state)),
    };
    Message::Input(input_msg, input_state)
}

// Return to Normal mode on Esc, otherwise continue in the given `mode`.
fn default(key: KeyCode, mode: Mode) -> Message {
    Message::Continue(match key {
        KeyCode::Esc => Mode::Normal,
        _ => mode,
    })
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
fn to_compare_msg(key: KeyCode, compare_state: CompareState) -> Message {
    let compare_msg = match key {
        KeyCode::Char(' ') => CompareMsg::Toggle,
        KeyCode::Enter => CompareMsg::Confirm,
        _ => return default(key, Mode::Compare(compare_state)),
    };
    Message::Compare(compare_msg, compare_state)
}

// Map a `key` to a Message in Save mode.
fn to_save_msg(key: KeyCode, save_state: SaveState) -> Message {
    let save_msg = match key {
        KeyCode::Char(' ') => SaveMsg::Toggle,
        KeyCode::Enter => SaveMsg::Confirm,
        _ => return default(key, Mode::Save(save_state)),
    };
    Message::Save(save_msg, save_state)
}

// Map a pressed `key` to a Message based on the current `mode`.
fn key_to_message(mode: Mode, key: KeyCode) -> Message {
    match mode {
        Mode::Confirm(confirm_state) => to_confirm_msg(key, confirm_state),
        Mode::Load(load_state) => to_load_msg(key, load_state),
        Mode::Normal => to_normal_msg(key),
        Mode::Input(input) => to_input_msg(key, input),
        Mode::Select(index) => to_select_msg(key, index),
        Mode::Selected(index) => to_selected_msg(key, index),
        Mode::Compare(compare_state) => to_compare_msg(key, compare_state),
        Mode::Save(save_state) => to_save_msg(key, save_state),
    }
}

/// Convert a user input event into a Message based on the current `mode`.
pub fn handle_event(mode: Mode) -> Result<Message> {
    let event::Event::Key(key) = event::read()? else {
        return Ok(Message::Continue(mode));
    };
    if key.kind != KeyEventKind::Press {
        return Ok(Message::Continue(mode));
    }
    Ok(key_to_message(mode, key.code))
}

