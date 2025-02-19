use ratatui::{
    style::{Styled, Stylize},
    text::{Line, Span},
};

use crate::{
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
mod input {
    pub const ADD: &str = "Add item";
    pub const EDIT: &str = "Edit item";
    pub const INSERT: &str = "Insert item";
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
const MOVE: &str = "Move subtree";
const INSERT: &str = "Enter position to insert new item";
const SAVE: &str = "Save changes?";
const UNTITLED: &str = "Untitled";

fn info(text: &str) -> Span {
    format!("[{text}]").into()
}

// Status bar Line with the `message`.
fn status(text: &str) -> Vec<Span> {
    vec![text.into()]
}

// Status bar Line with the `message` and additional info if present.
fn status_info<'a>(message: &'a str, maybe_info: Option<&'a str>) -> Vec<Span<'a>> {
    match maybe_info {
        Some(text) => vec![message.into(), " | ".into(), info(text)],
        None => status(message),
    }
}

// Normal mode status bar Line with the filename, if it exists.
fn status_normal(maybe_filename: Option<&str>) -> Vec<Span> {
    vec![match maybe_filename {
        Some(filename) => filename.bold(),
        None => info(UNTITLED),
    }]
}

// Select mode status bar Line showing the selected `index`.
fn status_select(index: usize) -> Vec<Span<'static>> {
    vec![
        SELECT.into(),
        index.to_string().bold(),
    ]
}

/// Return the status bar widget based on the `model`.
pub fn status_bar(model: &Model) -> Line {
    let content = match &model.mode {
        Mode::Confirm(confirm_state) => match confirm_state {
            ConfirmState::NewSession => status(confirm::NEW),
            ConfirmState::DeleteItem(..) => status(confirm::DELETE_ITEM),
            ConfirmState::DeleteFile(_) => status(confirm::DELETE_FILE),
        }
        Mode::Load(_) => status(LOAD),
        Mode::Normal => status_normal(model.get_filename()),
        Mode::Input(InputState::Label(label_state)) => {
            let message = match label_state.action {
                LabelAction::Add => input::ADD,
                LabelAction::Edit(_) => input::EDIT,
                LabelAction::Insert (..) => input::INSERT,
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
        Mode::Move(_) => status(MOVE),
        Mode::Insert(_) => status(INSERT),
        Mode::Save(save_state) => {
            let info = match save_state.post_save {
                PostSaveAction::Load => post_save::LOAD,
                PostSaveAction::Quit => post_save::QUIT,
            };
            status_info(SAVE, Some(info))
        }
    };
    let mut spans = vec![" ".into()];
    spans.extend(content);
    Line::from(spans)
    .left_aligned()
    .set_style(style::ACCENT)
}

