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
        SaveAction,
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
    pub const INVALID: &str = "Invalid Filename";
}
mod action {
    pub const LOAD: &str = "Loading";
    pub const QUIT: &str = "Quitting";
}
const LOAD: &str = "Open a file or start a new session";
const SELECT: &str = "Selected index: ";
const SELECTED: &str = "Enter command";
const COMPARE: &str = "Select item to promote";
const SAVE: &str = "Save changes?";

fn add_indent(text: &str) -> String {
    format!(" {}", text)
}

// Return a status bar Line with the `message`.
fn status(message: &str) -> Line {
    Line::from(add_indent(message))
}

// Return a status bar Line with the `message` and additional `info`.
fn status_info<'a>(message: &'a str, info: &'a str) -> Line<'a> {
    Line::from(format!(" {message} | [{info}]"))
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
                true => status_info(input::ADD, alert::EMPTY),
                false => status(input::ADD),
            }
            InputAction::Edit(_) => match input_state.input.is_empty() {
                true => status_info(input::EDIT, alert::EMPTY),
                false => status(input::EDIT),
            }
            InputAction::Save(filename_status, _) => match filename_status {
                FileNameStatus::Empty => {
                    status_info(input::FILENAME, alert::EMPTY)
                }
                FileNameStatus::Exists => {
                    status_info(input::FILENAME, alert::EXISTS)
                }
                FileNameStatus::Invalid => {
                    status_info(input::FILENAME, alert::INVALID)
                }
                FileNameStatus::Valid => status(input::FILENAME),
            }
        }
        Mode::Select(index) => status_select(*index),
        Mode::Selected(_) => status(SELECTED),
        Mode::Compare(_) => status(COMPARE),
        Mode::Save(save_state) => match save_state.action {
            SaveAction::Load => status_info(SAVE, action::LOAD),
            SaveAction::Quit => status_info(SAVE, action::QUIT),
        }
    }
    .left_aligned()
    .set_style(style::ACCENT)
}

