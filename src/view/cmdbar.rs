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
const EDIT: KeyPair = ("E", "Edit");
const MOVE: KeyPair = ("M", "Move");
const NEST: KeyPair = ("N", "Nest");
const FLATTEN: KeyPair = ("F", "Flatten");
const DOWN: KeyPair = ("J │ ↓", "Down");
const UP: KeyPair = ("K │ ↑", "Up");
const PROMOTE: KeyPair = ("H │ ←", "Promote");
const DEMOTE: KeyPair = ("L │ →", "Demote");
const RENAME: KeyPair = ("R", "Rename");
const INSERT: KeyPair = ("I", "Insert");
const PARENT: KeyPair = ("H", "Parent");
const CHILD: KeyPair = ("L", "Child");
const BEFORE: KeyPair = ("K", "Before");
const AFTER: KeyPair = ("J", "After");
const DELETE: KeyPair = ("D", "Delete");
const BACK: KeyPair = ("Bksp", "Back");
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
    let mut pairs = Vec::new();
    if root.size() > 1 {
        pairs.extend(&[JUMP, DOWN_UP]);
    }
    let cmd1 = match root.size() {
        0 => INSERT,
        _ => EDIT,
    };
    pairs.extend(&[cmd1, LOAD, QUIT]);
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
fn edit_mode_commands(size: usize) -> Vec<KeyPair<'static>> {
    let mut pairs = Vec::new();
    if size > 1 {
        pairs.extend(&[JUMP, DOWN_UP]);
    }
    pairs.extend(&[RENAME, MOVE, NEST, FLATTEN, INSERT, DELETE, BACK]);
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
        Mode::Normal(_) => normal_mode_commands(&model.state.root),
        Mode::Input(input_state) => input_mode_commands(input_state),
        Mode::Edit(_) => edit_mode_commands(model.state.root.size()),
        Mode::Move(_) => vec![DOWN, UP, PROMOTE, DEMOTE, DONE, CANCEL],
        Mode::Insert(_) => vec![PARENT, CHILD, BEFORE, AFTER, BACK],
        Mode::Save(_) => vec![TOGGLE, CONFIRM, CANCEL],
    };
    to_command_bar(pairs)
}

