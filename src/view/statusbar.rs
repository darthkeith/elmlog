use ratatui::{
    style::{Styled, Stylize},
    text::{Line, Span},
};

use crate::{
    heap::HeapStatus,
    model::{
        ConfirmState,
        FilenameAction,
        FilenameStatus,
        InputState,
        LabelAction,
        Mode,
        Model,
        PostSaveAction,
    },
    view::style
};

mod confirm {
    pub const NEW: &str = "No saved files, starting new session...";
    pub const DELETE_ITEM: &str = "Delete item?";
    pub const DELETE_FILE: &str = "Delete file?";
}
mod normal {
    pub const EMPTY: &str = "Empty";
    pub const SINGLE: &str = "Item selected";
    pub const MULTI: &str = "Items to compare: ";
}
mod input {
    pub const ADD: &str = "Add item";
    pub const EDIT: &str = "Edit item";
    pub const RENAME: &str = "Rename file";
    pub const SAVENEW: &str = "Save new file";
}
mod alert {
    pub const EMPTY: &str = "Empty";
    pub const EXISTS: &str = "File Exists";
    pub const INVALID: &str = "Invalid Filename";
}
mod post_save {
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
fn status_info<'a>(message: &'a str, info: Option<&'a str>) -> Line<'a> {
    match info {
        Some(info) => Line::from(format!(" {message} | [{info}]")),
        None => status(message),
    }
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
        Mode::Confirm(confirm_state) => match confirm_state {
            ConfirmState::NewSession => status(confirm::NEW),
            ConfirmState::DeleteItem(..) => status(confirm::DELETE_ITEM),
            ConfirmState::DeleteFile(_) => status(confirm::DELETE_FILE),
        }
        Mode::Load(_) => status(LOAD),
        Mode::Normal => match model.state.heap.status() {
            HeapStatus::Empty => status(normal::EMPTY),
            HeapStatus::SingleRoot => status(normal::SINGLE),
            HeapStatus::MultiRoot(..) => {
                let n = model.state.heap.root_count();
                status_normal_multi(n)
            }
        }
        Mode::Input(InputState::Label(label_state)) => {
            let message = match label_state.action {
                LabelAction::Add => input::ADD,
                LabelAction::Edit(_) => input::EDIT,
            };
            let info = match label_state.is_empty() {
                true => Some(alert::EMPTY),
                false => None,
            };
            status_info(message, info)
        }
        Mode::Input(InputState::Filename(filename_state)) => {
            let message = match filename_state.action {
                FilenameAction::Rename(_) => input::RENAME,
                FilenameAction::SaveNew(_) => input::SAVENEW,
            };
            let info = match filename_state.status {
                FilenameStatus::Empty => Some(alert::EMPTY),
                FilenameStatus::Exists => Some(alert::EXISTS),
                FilenameStatus::Invalid => Some(alert::INVALID),
                FilenameStatus::Valid => None,
            };
            status_info(message, info)
        }
        Mode::Select(index) => status_select(*index),
        Mode::Selected(_) => status(SELECTED),
        Mode::Compare(_) => status(COMPARE),
        Mode::Save(save_state) => {
            let info = match save_state.post_save {
                PostSaveAction::Load => post_save::LOAD,
                PostSaveAction::Quit => post_save::QUIT,
            };
            status_info(SAVE, Some(info))
        }
    }
    .left_aligned()
    .set_style(style::ACCENT)
}

