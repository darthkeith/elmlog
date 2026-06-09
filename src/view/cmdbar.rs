use ratatui::{style::Styled, text::Line};

use crate::{
    model::{ConfirmState, FilenameState, LabelState, Model, SessionState},
    view::style,
};

type KeyPair<'a> = (&'a str, &'a str);

const LOAD_NAVIGATE: KeyPair = ("JK │ ↓↑", "Navigate");
const NAVIGATE: KeyPair = ("HJKL │ ←↓↑→", "Navigate");
const OPEN: KeyPair = ("Enter", "Open");
const NEW: KeyPair = ("N", "New");
const RENAME: KeyPair = ("R", "Rename");
const SUBMIT: KeyPair = ("Enter", "Submit");
const CONFIRM: KeyPair = ("Enter", "Confirm");
const DONE: KeyPair = ("Enter", "Done");
const QUIT: KeyPair = ("Q", "Quit");
const MOVE: KeyPair = ("M", "Move");
const NEST: KeyPair = ("N", "Nest");
const FLATTEN: KeyPair = ("F", "Flatten");
const DOWN: KeyPair = ("J │ ↓", "Down");
const UP: KeyPair = ("K │ ↑", "Up");
const PROMOTE: KeyPair = ("H │ ←", "Promote");
const DEMOTE: KeyPair = ("L │ →", "Demote");
const EDIT: KeyPair = ("E", "Edit");
const INSERT: KeyPair = ("I", "Insert");
const PARENT: KeyPair = ("H", "Parent");
const CHILD: KeyPair = ("L", "Child");
const BEFORE: KeyPair = ("K", "Before");
const AFTER: KeyPair = ("J", "After");
const DELETE: KeyPair = ("D", "Delete");
const UNDO: KeyPair = ("U", "Undo");
const REDO: KeyPair = ("R", "Redo");
const BACK: KeyPair = ("Bksp", "Back");
const TOGGLE: KeyPair = ("Space", "Toggle");
const CANCEL: KeyPair = ("Esc", "Cancel");

// Return the confirm mode key-command pairs.
fn confirm_mode_commands(
    confirm_state: &ConfirmState,
) -> Vec<KeyPair<'static>> {
    match confirm_state {
        ConfirmState::NewSession => vec![CONFIRM],
        _ => vec![CONFIRM, CANCEL],
    }
}

// Return the load mode key-command pairs.
fn load_mode_commands(file_count: usize) -> Vec<KeyPair<'static>> {
    let mut pairs = Vec::new();
    if file_count > 1 {
        pairs.push(LOAD_NAVIGATE);
    }
    pairs.extend(&[OPEN, NEW, RENAME, DELETE, QUIT]);
    pairs
}

// Return the normal mode key-command pairs.
fn normal_mode_commands(session: &SessionState) -> Vec<KeyPair<'static>> {
    let mut pairs = Vec::new();
    if session.focus().is_none() {
        pairs.push(INSERT);
    } else {
        pairs.extend(&[NAVIGATE, EDIT, MOVE, NEST, FLATTEN, INSERT, DELETE]);
    }
    if !session.undo_stack.is_empty() {
        pairs.push(UNDO);
    }
    if !session.redo_stack.is_empty() {
        pairs.push(REDO);
    }
    pairs.push(QUIT);
    pairs
}

// Return the input mode key-command pairs.
fn label_input_commands(label_state: &LabelState) -> Vec<KeyPair<'static>> {
    if label_state.input.is_empty() {
        vec![CANCEL]
    } else {
        vec![SUBMIT, CANCEL]
    }
}

// Return the input mode key-command pairs.
fn filename_input_commands(
    filename_state: &FilenameState,
) -> Vec<KeyPair<'static>> {
    if filename_state.is_valid() {
        vec![SUBMIT, CANCEL]
    } else {
        vec![CANCEL]
    }
}

// Construct the command bar widget from a sequence of key-command pairs.
fn to_command_bar(pairs: Vec<KeyPair>) -> Line {
    let mut spans = Vec::new();
    for (key, command) in pairs {
        spans.push(format!(" {key} ").set_style(style::CMD_KEY));
        spans.push(format!(" {command}").set_style(style::CMD_NAME));
        spans.push("    ".into());
    }
    spans.pop(); // Remove extra spacer at end
    Line::from(spans).centered().set_style(style::ACCENT)
}

/// Return the command bar widget based on the current `model`.
pub fn command_bar(model: &Model) -> Line<'static> {
    let pairs = match model {
        Model::Load(load_state) => load_mode_commands(load_state.files.len()),
        Model::Normal(state) => normal_mode_commands(state),
        Model::Insert(_) => vec![PARENT, CHILD, BEFORE, AFTER, BACK],
        Model::Move(_) => vec![DOWN, UP, PROMOTE, DEMOTE, DONE],
        Model::Save(_) => vec![TOGGLE, CONFIRM, CANCEL],
        Model::LabelInput(label_state) => label_input_commands(label_state),
        Model::FilenameInput(filename_state) => {
            filename_input_commands(filename_state)
        }
        Model::Confirm(confirm_state) => confirm_mode_commands(confirm_state),
    };
    to_command_bar(pairs)
}
