use ratatui::{
    style::Styled,
    text::Line,
};

use crate::{
    forest::Node,
    model::{
        ConfirmState,
        InputState,
        Mode,
        Model,
    },
    view::style,
};

type KeyPair<'a> = (&'a str, &'a str);

const JUMP: KeyPair = ("0-9", "Jump");
const DOWN_UP: KeyPair = ("J/K │ ↓/↑", "Down/Up");
const OPEN: KeyPair = ("Enter", "Open");
const SUBMIT: KeyPair = ("Enter", "Submit");
const CONFIRM: KeyPair = ("Enter", "Confirm");
const DONE: KeyPair = ("Enter", "Done");
const NEW: KeyPair = ("N", "New");
const LOAD: KeyPair = ("L", "Load");
const QUIT: KeyPair = ("Q", "Quit");
const ADD: KeyPair = ("A", "Add");
const SELECT: KeyPair = ("S", "Select");
const EDIT: KeyPair = ("E", "Edit");
const MOVE: KeyPair = ("M", "Move");
const RAISE: KeyPair = ("R", "Raise");
const FLATTEN: KeyPair = ("F", "Flatten");
const DOWN: KeyPair = ("J │ ↓", "Down");
const UP: KeyPair = ("K │ ↑", "Up");
const PROMOTE: KeyPair = ("H │ ←", "Promote");
const DEMOTE: KeyPair = ("L │ →", "Demote");
const RENAME: KeyPair = ("R", "Rename");
const DELETE: KeyPair = ("D", "Delete");
const TOGGLE: KeyPair = ("Space", "Toggle");
const CANCEL: KeyPair = ("Esc", "Cancel");

// Return the confirm mode key-command pairs.
fn confirm_mode_commands(confirm_state: &ConfirmState) -> Vec<KeyPair<'static>> {
    match confirm_state {
        ConfirmState::NewSession => vec![CONFIRM],
        _ => vec![CONFIRM, CANCEL],
    }
}

// Return the load mode key-command pairs.
fn load_mode_commands(file_count: usize) -> Vec<KeyPair<'static>> {
    let mut pairs = Vec::new();
    if file_count > 1 {
        pairs.extend(&[JUMP, DOWN_UP]);
    }
    pairs.extend(&[OPEN, NEW, RENAME, DELETE, QUIT]);
    pairs
}

// Return the normal mode key-command pairs.
fn normal_mode_commands(root: &Node) -> Vec<KeyPair> {
    let mut pairs = vec![ADD];
    if root.size() > 0 {
        pairs.push(SELECT);
    }
    pairs.extend(&[LOAD, QUIT]);
    pairs
}

// Return the input mode key-command pairs.
fn input_mode_commands(input_state: &InputState) -> Vec<KeyPair> {
    if input_state.is_valid() {
        vec![SUBMIT, CANCEL]
    } else {
        vec![CANCEL]
    }
}

// Return the select mode key-command pairs.
fn select_mode_commands(size: usize) -> Vec<KeyPair<'static>> {
    let mut pairs = Vec::new();
    if size > 1 {
        pairs.extend(&[JUMP, DOWN_UP]);
    }
    pairs.extend(&[CONFIRM, CANCEL]);
    pairs
}

// Convert key-command pairs into a command bar.
fn to_command_bar(pairs: Vec<KeyPair>) -> Line {
    let mut spans = Vec::new();
    for (key, command) in pairs {
        spans.push(format!(" {key} ").set_style(style::CMD_KEY));
        spans.push(format!(" {command}").set_style(style::CMD_NAME));
        spans.push("    ".into());
    }
    spans.pop();  // Remove extra spacer at end
    Line::from(spans)
        .centered()
        .set_style(style::ACCENT)
}

/// Return the command bar widget based on the current `model`.
pub fn command_bar(model: &Model) -> Line {
    let pairs = match &model.mode {
        Mode::Confirm(confirm_state) => confirm_mode_commands(confirm_state),
        Mode::Load(load_state) => load_mode_commands(load_state.size()),
        Mode::Normal => normal_mode_commands(&model.state.root),
        Mode::Input(input_state) => input_mode_commands(input_state),
        Mode::Select(_) => select_mode_commands(model.state.root.size()),
        Mode::Selected(_) => vec![EDIT, MOVE, RAISE, FLATTEN, DELETE, CANCEL],
        Mode::Move(_) => vec![DOWN, UP, PROMOTE, DEMOTE, DONE, CANCEL],
        Mode::Save(_) => vec![TOGGLE, CONFIRM, CANCEL],
    };
    to_command_bar(pairs)
}

