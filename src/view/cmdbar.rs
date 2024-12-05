use ratatui::{
    style::Styled,
    text::Line,
};

use crate::{
    heap::{
        Heap,
        HeapStatus,
    },
    model::{
        InputState,
        Mode,
        Model,
    },
    view::style,
};

const JUMP: (&str, &str) = ("0-9", "Jump");
const DOWN_UP: (&str, &str) = ("J/K │ ↓/↑", "Down/Up");
const OPEN: (&str, &str) = ("Enter", "Open");
const SUBMIT: (&str, &str) = ("Enter", "Submit");
const CONFIRM: (&str, &str) = ("Enter", "Confirm");
const NEW: (&str, &str) = ("N", "New");
const QUIT: (&str, &str) = ("Q", "Quit");
const INSERT: (&str, &str) = ("I", "Insert");
const SELECT: (&str, &str) = ("S", "Select");
const COMPARE: (&str, &str) = ("C", "Compare");
const EDIT: (&str, &str) = ("E", "Edit");
const DELETE: (&str, &str) = ("D", "Delete");
const TOGGLE: (&str, &str) = ("Space", "Toggle");
const CANCEL: (&str, &str) = ("Esc", "Cancel");

// Return the load mode key-command pairs.
fn load_mode_commands(file_count: usize) -> Vec<(&'static str, &'static str)> {
    let mut pairs = Vec::new();
    if file_count > 1 {
        pairs.push(DOWN_UP);
    }
    pairs.extend(&[OPEN, NEW, QUIT]);
    pairs
}

// Return the normal mode key-command pairs.
fn normal_mode_commands(heap: &Heap) -> Vec<(&str, &str)> {
    let mut pairs = vec![INSERT];
    if heap.size() > 0 {
        pairs.push(SELECT);
        if let HeapStatus::MultiRoot(..) = heap.status() {
            pairs.push(COMPARE);
        }
    }
    pairs.push(QUIT);
    pairs
}

// Return the input mode key-command pairs.
fn input_mode_commands(input_state: &InputState) -> Vec<(&'static str, &'static str)> {
    if input_state.is_valid() {
        vec![SUBMIT]
    } else {
        Vec::new()
    }
}

// Return the select mode key-command pairs.
fn select_mode_commands(size: usize) -> Vec<(&'static str, &'static str)> {
    let mut pairs = Vec::new();
    if size > 1 {
        pairs.extend(&[JUMP, DOWN_UP]);
    }
    pairs.push(CONFIRM);
    pairs
}

// Convert key-command pairs into a command bar.
fn to_command_bar<'a>(pairs: Vec<(&'a str, &'a str)>) -> Line<'a> {
    let mut spans = Vec::new();
    for (key, command) in pairs {
        spans.push(format!(" {key} ").set_style(style::CMD_KEY));
        spans.push(format!(" {command}").set_style(style::CMD_NAME));
        spans.push("    ".into());
    }
    spans.pop();  // Remove extra spacer at end
    Line::from(spans)
        .centered()
        .set_style(style::DEFAULT)
}

/// Return the command bar widget based on the current `model`.
pub fn command_bar(model: &Model) -> Line {
    let mut pairs = match &model.mode {
        Mode::Load(load_state) => load_mode_commands(load_state.size()),
        Mode::Normal => normal_mode_commands(&model.state.heap),
        Mode::Input(input_state) => input_mode_commands(input_state),
        Mode::Select(_) => select_mode_commands(model.state.heap.size()),
        Mode::Selected(_) => vec![EDIT, DELETE],
        Mode::Compare(_) | Mode::Save(_) => vec![TOGGLE, CONFIRM],
    };
    match &model.mode {
        Mode::Load(_) | Mode::Normal => (),
        _ => pairs.push(CANCEL),
    }
    to_command_bar(pairs)
}

