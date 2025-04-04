use std::io::Result;

use crossterm::event::{self, KeyCode, KeyEventKind};

use crate::{
    io::{FileEntry, LoadState},
    model::{
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
    Append(char),
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
    Insert,
    Edit,
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

/// A message sent in Edit mode.
pub enum EditMsg {
    Append(char),
    Decrement,
    Increment,
    Rename,
    Move,
    Nest,
    Flatten,
    Insert,
    Delete,
}

/// A message sent in Move mode.
pub enum MoveMsg {
    Forward,
    Backward,
    Promote,
    Demote,
    Done,
}

/// A message sent in Insert mode.
pub enum InsertMsg {
    Parent,
    Child,
    Before,
    After,
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
    Edit(EditMsg, usize),
    Move(MoveMsg, usize),
    Insert(InsertMsg, usize),
    Save(SaveMsg, SaveState),
    Continue(Mode),
}

/// A message indicating an IO action to perform.
pub enum Command {
    None(Model),
    Load,
    InitSession(FileEntry),
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
        KeyCode::Char(c) => match c {
            'k' => LoadMsg::Decrement,
            'j' => LoadMsg::Increment,
            'n' => LoadMsg::New,
            'r' => LoadMsg::Rename,
            'd' => LoadMsg::Delete,
            'q' => LoadMsg::Quit,
            _ => LoadMsg::Append(c),
        },
        KeyCode::Up => LoadMsg::Decrement,
        KeyCode::Down => LoadMsg::Increment,
        KeyCode::Enter => LoadMsg::Open,
        _ => return Message::Continue(Mode::Load(load_state)),
    };
    Message::Load(load_msg, load_state)
}

// Map a `key` to a Message in Normal mode.
fn to_normal_msg(key: KeyCode) -> Message {
    let normal_msg = match key {
        KeyCode::Char('i') => NormalMsg::Insert,
        KeyCode::Char('e') => NormalMsg::Edit,
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

// Map a `key` to a Message in Edit mode.
fn to_edit_msg(key: KeyCode, index: usize) -> Message {
    let edit_msg = match key {
        KeyCode::Char(c) => match c {
            'k' => EditMsg::Decrement,
            'j' => EditMsg::Increment,
            'r' => EditMsg::Rename,
            'm' => EditMsg::Move,
            'n' => EditMsg::Nest,
            'f' => EditMsg::Flatten,
            'i' => EditMsg::Insert,
            'd' => EditMsg::Delete,
            _ => EditMsg::Append(c),
        },
        KeyCode::Up => EditMsg::Decrement,
        KeyCode::Down => EditMsg::Increment,
        _ => return default(key, Mode::Edit(index)),
    };
    Message::Edit(edit_msg, index)
}

// Map a `key` to a Message in Move mode.
fn to_move_msg(key: KeyCode, index: usize) -> Message {
    let move_msg = match key {
        KeyCode::Char('j') | KeyCode::Down => MoveMsg::Forward,
        KeyCode::Char('k') | KeyCode::Up => MoveMsg::Backward,
        KeyCode::Char('h') | KeyCode::Left => MoveMsg::Promote,
        KeyCode::Char('l') | KeyCode::Right => MoveMsg::Demote,
        KeyCode::Enter => MoveMsg::Done,
        _ => return default(key, Mode::Move(index)),
    };
    Message::Move(move_msg, index)
}

// Map a `key` to a Message in Insert mode.
fn to_insert_msg(key: KeyCode, index: usize) -> Message {
    let insert_msg = match key {
        KeyCode::Char('h') => InsertMsg::Parent,
        KeyCode::Char('l') => InsertMsg::Child,
        KeyCode::Char('k') => InsertMsg::Before,
        KeyCode::Char('j') => InsertMsg::After,
        _ => return default(key, Mode::Insert(index)),
    };
    Message::Insert(insert_msg, index)
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
        Mode::Edit(index) => to_edit_msg(key, index),
        Mode::Move(index) => to_move_msg(key, index),
        Mode::Insert(index) => to_insert_msg(key, index),
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

