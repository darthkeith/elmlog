use std::io::Result;

use crossterm::event::{self, KeyCode, KeyEventKind};

use crate::{
    io::LoadState,
    model::{
        ConfirmState,
        FilenameState,
        LabelState,
        Model,
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

/// A message sent in Insert mode.
pub enum InsertMsg {
    Parent,
    Child,
    Before,
    After,
    Back,
}

/// A message sent in Move mode.
pub enum MoveMsg {
    Promote,
    Demote,
    Backward,
    Forward,
    Done,
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

/// A message sent in Save mode.
pub enum SaveMsg {
    Toggle,
    Confirm,
    Cancel,
}

/// A message sent in Filename Input mode.
pub enum FilenameMsg {
    Edit(InputEdit),
    Submit,
    Cancel,
}

/// A message sent in Confirm mode.
pub enum ConfirmMsg {
    Confirm,
    Cancel,
}

/// A message indicating changes to be made to the model.
pub enum Message {
    Load(LoadMsg, LoadState),
    Normal(NormalMsg, SessionState),
    Insert(InsertMsg, SessionState),
    Move(MoveMsg, SessionState),
    LabelInput(LabelMsg, LabelState),
    Save(SaveMsg, SaveState),
    FilenameInput(FilenameMsg, FilenameState),
    Confirm(ConfirmMsg, ConfirmState),
    Continue(Model),
}

// Map a `key` to a Message in Load mode.
fn to_load_msg(key: KeyCode, state: LoadState) -> Message {
    let msg = match key {
        KeyCode::Char('k') => LoadMsg::Decrement,
        KeyCode::Char('j') => LoadMsg::Increment,
        KeyCode::Char('n') => LoadMsg::New,
        KeyCode::Char('r') => LoadMsg::Rename,
        KeyCode::Char('d') => LoadMsg::Delete,
        KeyCode::Char('q') => LoadMsg::Quit,
        KeyCode::Down => LoadMsg::Increment,
        KeyCode::Up => LoadMsg::Decrement,
        KeyCode::Enter => LoadMsg::Open,
        _ => return Message::Continue(Model::Load(state)),
    };
    Message::Load(msg, state)
}

// Map a `key` to a Message in Normal mode.
fn to_normal_msg(key: KeyCode, state: SessionState) -> Message {
    let msg = match key {
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
        _ => return Message::Continue(Model::Normal(state)),
    };
    Message::Normal(msg, state)
}

// Map a `key` to a Message in Insert mode.
fn to_insert_msg(key: KeyCode, state: SessionState) -> Message {
    let msg = match key {
        KeyCode::Char('h') => InsertMsg::Parent,
        KeyCode::Char('l') => InsertMsg::Child,
        KeyCode::Char('k') => InsertMsg::Before,
        KeyCode::Char('j') => InsertMsg::After,
        KeyCode::Backspace => InsertMsg::Back,
        _ => return Message::Continue(Model::Insert(state)),
    };
    Message::Insert(msg, state)
}

// Map a `key` to a Message in Move mode.
fn to_move_msg(key: KeyCode, state: SessionState) -> Message {
    let msg = match key {
        KeyCode::Char('h') | KeyCode::Left => MoveMsg::Promote,
        KeyCode::Char('l') | KeyCode::Right => MoveMsg::Demote,
        KeyCode::Char('k') | KeyCode::Up => MoveMsg::Backward,
        KeyCode::Char('j') | KeyCode::Down => MoveMsg::Forward,
        KeyCode::Enter => MoveMsg::Done,
        _ => return Message::Continue(Model::Move(state)),
    };
    Message::Move(msg, state)
}

// Map a `key` to a Message in Label Input mode.
fn to_label_input_msg(key: KeyCode, state: LabelState) -> Message {
    let msg = match key {
        KeyCode::Char(c) => LabelMsg::Edit(InputEdit::Append(c)),
        KeyCode::Backspace => LabelMsg::Edit(InputEdit::PopChar),
        KeyCode::Enter => LabelMsg::Submit,
        KeyCode::Esc => LabelMsg::Cancel,
        _ => return Message::Continue(Model::LabelInput(state)),
    };
    Message::LabelInput(msg, state)
}

// Map a `key` to a Message in Save mode.
fn to_save_msg(key: KeyCode, state: SaveState) -> Message {
    let msg = match key {
        KeyCode::Char(' ') => SaveMsg::Toggle,
        KeyCode::Enter => SaveMsg::Confirm,
        KeyCode::Esc => SaveMsg::Cancel,
        _ => return Message::Continue(Model::Save(state)),
    };
    Message::Save(msg, state)
}

// Map a `key` to a Message in Filename Input mode.
fn to_filename_input_msg(key: KeyCode, state: FilenameState) -> Message {
    let msg = match key {
        KeyCode::Char(c) => FilenameMsg::Edit(InputEdit::Append(c)),
        KeyCode::Backspace => FilenameMsg::Edit(InputEdit::PopChar),
        KeyCode::Enter => FilenameMsg::Submit,
        KeyCode::Esc => FilenameMsg::Cancel,
        _ => return Message::Continue(Model::FilenameInput(state)),
    };
    Message::FilenameInput(msg, state)
}

// Map a `key` to a Message in Confirm mode.
fn to_confirm_msg(key: KeyCode, state: ConfirmState) -> Message {
    let msg = match key {
        KeyCode::Enter => ConfirmMsg::Confirm,
        KeyCode::Esc => ConfirmMsg::Cancel,
        _ => return Message::Continue(Model::Confirm(state)),
    };
    Message::Confirm(msg, state)
}

// Map a pressed `key` to a Message based on the current `model`.
fn key_to_message(model: Model, key: KeyCode) -> Message {
    match model {
        Model::Load(load_state) => to_load_msg(key, load_state),
        Model::Normal(session_state) => to_normal_msg(key, session_state),
        Model::Insert(session_state) => to_insert_msg(key, session_state),
        Model::Move(session_state) => to_move_msg(key, session_state),
        Model::LabelInput(label_state) => to_label_input_msg(key, label_state),
        Model::Save(save_state) => to_save_msg(key, save_state),
        Model::FilenameInput(filename_state) =>
            to_filename_input_msg(key, filename_state),
        Model::Confirm(confirm_state) => to_confirm_msg(key, confirm_state),
    }
}

/// Convert a user input event into a Message based on the current `model`.
pub fn handle_input(model: Model) -> Result<Message> {
    let event::Event::Key(key) = event::read()? else {
        return Ok(Message::Continue(model));
    };
    if key.kind != KeyEventKind::Press {
        return Ok(Message::Continue(model));
    }
    Ok(key_to_message(model, key.code))
}

