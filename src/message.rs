use std::io::Result;

use crossterm::event::{self, KeyCode, KeyEventKind};

use crate::{
    io::{FileEntry, LoadState},
    model::{
        ConfirmState,
        FilenameState,
        LabelState,
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
    Ascend,
    Descend,
    Previous,
    Next,
    Rename,
    Insert,
    Move,
    Nest,
    Flatten,
    Delete,
    Load,
    Quit,
}

/// Type of edit to apply to the user input text.
pub enum InputEdit {
    Append(char),
    PopChar,
}

/// A message sent in Label Input mode.
pub enum LabelMsg {
    Edit(InputEdit),
    Submit,
    Cancel,
}

/// A message sent in Filename Input mode.
pub enum FilenameMsg {
    Edit(InputEdit),
    Submit,
    Cancel,
}

/// A message sent in Move mode.
pub enum MoveMsg {
    Promote,
    Demote,
    Backward,
    Forward,
    Done,
}

/// A message sent in Insert mode.
pub enum InsertMsg {
    Parent,
    Child,
    Before,
    After,
    Back,
}

/// A message sent in Save mode.
pub enum SaveMsg {
    Toggle,
    Confirm,
    Cancel,
}

/// A message indicating changes to be made to the model.
pub enum Message {
    Confirm(bool, ConfirmState),
    Load(LoadMsg, LoadState),
    Normal(NormalMsg),
    LabelInput(LabelMsg, LabelState),
    FilenameInput(FilenameMsg, FilenameState),
    Move(MoveMsg),
    Insert(InsertMsg),
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
        KeyCode::Char('k') => LoadMsg::Decrement,
        KeyCode::Char('j') => LoadMsg::Increment,
        KeyCode::Char('n') => LoadMsg::New,
        KeyCode::Char('r') => LoadMsg::Rename,
        KeyCode::Char('d') => LoadMsg::Delete,
        KeyCode::Char('q') => LoadMsg::Quit,
        KeyCode::Down => LoadMsg::Increment,
        KeyCode::Up => LoadMsg::Decrement,
        KeyCode::Enter => LoadMsg::Open,
        _ => return Message::Continue(Mode::Load(load_state)),
    };
    Message::Load(load_msg, load_state)
}

// Map a `key` to a Message in Normal mode.
fn to_normal_msg(key: KeyCode) -> Message {
    let normal_msg = match key {
        KeyCode::Char('h') => NormalMsg::Ascend,
        KeyCode::Char('l') => NormalMsg::Descend,
        KeyCode::Char('k') => NormalMsg::Previous,
        KeyCode::Char('j') => NormalMsg::Next,
        KeyCode::Char('r') => NormalMsg::Rename,
        KeyCode::Char('i') => NormalMsg::Insert,
        KeyCode::Char('m') => NormalMsg::Move,
        KeyCode::Char('n') => NormalMsg::Nest,
        KeyCode::Char('f') => NormalMsg::Flatten,
        KeyCode::Char('d') => NormalMsg::Delete,
        KeyCode::Char('q') => NormalMsg::Quit,
        KeyCode::Left => NormalMsg::Ascend,
        KeyCode::Right => NormalMsg::Descend,
        KeyCode::Up => NormalMsg::Previous,
        KeyCode::Down => NormalMsg::Next,
        KeyCode::Backspace => NormalMsg::Load,
        _ => return Message::Continue(Mode::Normal),
    };
    Message::Normal(normal_msg)
}

// Map a `key` to a Message in Label Input mode.
fn to_label_input_msg(key: KeyCode, label_state: LabelState) -> Message {
    let label_msg = match key {
        KeyCode::Char(c) => LabelMsg::Edit(InputEdit::Append(c)),
        KeyCode::Backspace => LabelMsg::Edit(InputEdit::PopChar),
        KeyCode::Enter => LabelMsg::Submit,
        KeyCode::Esc => LabelMsg::Cancel,
        _ => return Message::Continue(Mode::LabelInput(label_state)),
    };
    Message::LabelInput(label_msg, label_state)
}

// Map a `key` to a Message in Filename Input mode.
fn to_filename_input_msg(key: KeyCode, filename_state: FilenameState) -> Message {
    let filename_msg = match key {
        KeyCode::Char(c) => FilenameMsg::Edit(InputEdit::Append(c)),
        KeyCode::Backspace => FilenameMsg::Edit(InputEdit::PopChar),
        KeyCode::Enter => FilenameMsg::Submit,
        KeyCode::Esc => FilenameMsg::Cancel,
        _ => return Message::Continue(Mode::FilenameInput(filename_state)),
    };
    Message::FilenameInput(filename_msg, filename_state)
}

// Map a `key` to a Message in Move mode.
fn to_move_msg(key: KeyCode) -> Message {
    let move_msg = match key {
        KeyCode::Char('h') | KeyCode::Left => MoveMsg::Promote,
        KeyCode::Char('l') | KeyCode::Right => MoveMsg::Demote,
        KeyCode::Char('k') | KeyCode::Up => MoveMsg::Backward,
        KeyCode::Char('j') | KeyCode::Down => MoveMsg::Forward,
        KeyCode::Enter => MoveMsg::Done,
        _ => return Message::Continue(Mode::Move),
    };
    Message::Move(move_msg)
}

// Map a `key` to a Message in Insert mode.
fn to_insert_msg(key: KeyCode) -> Message {
    let insert_msg = match key {
        KeyCode::Char('h') => InsertMsg::Parent,
        KeyCode::Char('l') => InsertMsg::Child,
        KeyCode::Char('k') => InsertMsg::Before,
        KeyCode::Char('j') => InsertMsg::After,
        KeyCode::Backspace => InsertMsg::Back,
        _ => return Message::Continue(Mode::Insert),
    };
    Message::Insert(insert_msg)
}

// Map a `key` to a Message in Save mode.
fn to_save_msg(key: KeyCode, save_state: SaveState) -> Message {
    let save_msg = match key {
        KeyCode::Char(' ') => SaveMsg::Toggle,
        KeyCode::Enter => SaveMsg::Confirm,
        KeyCode::Esc => SaveMsg::Cancel,
        _ => return Message::Continue(Mode::Save(save_state)),
    };
    Message::Save(save_msg, save_state)
}

// Map a pressed `key` to a Message based on the current `mode`.
fn key_to_message(mode: Mode, key: KeyCode) -> Message {
    match mode {
        Mode::Confirm(confirm_state) => to_confirm_msg(key, confirm_state),
        Mode::Load(load_state) => to_load_msg(key, load_state),
        Mode::Normal => to_normal_msg(key),
        Mode::LabelInput(input) => to_label_input_msg(key, input),
        Mode::FilenameInput(input) => to_filename_input_msg(key, input),
        Mode::Move => to_move_msg(key),
        Mode::Insert => to_insert_msg(key),
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

