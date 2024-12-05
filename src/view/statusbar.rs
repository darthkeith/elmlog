use ratatui::{
    style::{Styled, Stylize},
    text::{Line, Span},
};

use crate::{
    heap::HeapStatus,
    model::{
        FileNameStatus,
        InputAction,
        Mode,
        Model,
    },
    view::style
};

mod normal {
    pub const EMPTY: &str = "Empty";
    pub const SINGLE: &str = "Item selected";
    pub const MULTI: &str = "Items to compare: ";
}
mod input {
    pub const ADD: &str = "Enter new item";
    pub const EDIT: &str = "Edit item";
    pub const FILENAME: &str = "Enter file name";
}
mod alert {
    pub const EMPTY: &str = "Empty";
    pub const EXISTS: &str = "File Exists";
}
const LOAD: &str = "Select file to open.";
const SELECT: &str = "Selected index: ";
const SELECTED: &str = "Enter command";
const COMPARE: &str = "Select item to promote";
const SAVE: &str = "Save changes before quitting?";

fn add_indent(text: &str) -> String {
    format!(" {}", text)
}

// Return a status bar Line with the given message.
fn status(msg: &str) -> Line {
    Line::from(add_indent(msg))
}

// Return a status bar Line with the given message and alert.
fn status_alert<'a>(msg: &'a str, alert: &'a str) -> Line<'a> {
    Line::from(format!(" {msg} | [{alert}]"))
}

// Return the status bar Line in Normal mode when there are `n` roots.
fn status_normal_multi(n: usize) -> Line<'static> {
    Line::from(vec![
        add_indent(normal::MULTI).into(),
        Span::styled(n.to_string(), style::NUMBER),
    ])
}

// Return the status bar Line in Select mode with `index` selected.
fn status_select(index: usize) -> Line<'static> {
    Line::from(vec![
        add_indent(SELECT).into(),
        index.to_string().bold(),
    ])
}

/// Return the status bar widget based on the `model`.
pub fn status_bar(model: &Model) -> Line {
    match &model.mode {
        Mode::Load(_) => status(LOAD),
        Mode::Normal => match model.state.heap.status() {
            HeapStatus::Empty => status(normal::EMPTY),
            HeapStatus::SingleRoot => status(normal::SINGLE),
            HeapStatus::MultiRoot(..) => {
                let n = model.state.heap.root_count();
                status_normal_multi(n)
            }
        }
        Mode::Input(input_state) => match &input_state.action {
            InputAction::Add => match input_state.input.is_empty() {
                true => status_alert(input::ADD, alert::EMPTY),
                false => status(input::ADD),
            }
            InputAction::Edit(_) => match input_state.input.is_empty() {
                true => status_alert(input::EDIT, alert::EMPTY),
                false => status(input::EDIT),
            }
            InputAction::Save(file_name_status) => match file_name_status {
                FileNameStatus::Empty => {
                    status_alert(input::FILENAME, alert::EMPTY)
                }
                FileNameStatus::Exists => {
                    status_alert(input::FILENAME, alert::EXISTS)
                }
                FileNameStatus::Valid => status(input::FILENAME),
            }
        }
        Mode::Select(index) => status_select(*index),
        Mode::Selected(_) => status(SELECTED),
        Mode::Compare(_) => status(COMPARE),
        Mode::Save(_) => status(SAVE),
    }
    .left_aligned()
    .set_style(style::ACCENT)
}

