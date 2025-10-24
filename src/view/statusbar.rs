use ratatui::{
    style::{Styled, Stylize},
    text::{Line, Span},
};

use crate::{
    model::{
        ConfirmState,
        FilenameAction,
        FilenameStatus,
        LabelAction,
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
    pub const RENAME_ITEM: &str = "Rename item";
    pub const INSERT: &str = "Insert item";
    pub const RENAME_FILE: &str = "Rename file";
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

/// Return the status bar widget based on the `model`.
pub fn status_bar(model: &Model) -> Line {
    let content = match model {
        Model::Load(_) => status(LOAD),
        Model::Normal(state) => status_normal(state.get_filename()),
        Model::Insert(_) => status(INSERT),
        Model::Move(_) => status(MOVE),
        Model::Save(save_state) => {
            let info = match save_state.post_save {
                PostSaveAction::Load => post_save::LOAD,
                PostSaveAction::Quit => post_save::QUIT,
            };
            status_info(SAVE, Some(info))
        }
        Model::LabelInput(label_state) => {
            let message = match label_state.action {
                LabelAction::Insert => input::INSERT,
                LabelAction::Rename => input::RENAME_ITEM,
            };
            let info = if label_state.input.is_empty() {
                Some(alert::EMPTY)
            } else {
                None
            };
            status_info(message, info)
        }
        Model::FilenameInput(filename_state) => {
            let message = match filename_state.action {
                FilenameAction::Rename(_) => input::RENAME_FILE,
                FilenameAction::SaveNew { .. } => input::SAVENEW,
            };
            let info = match filename_state.status {
                FilenameStatus::Empty => Some(alert::EMPTY),
                FilenameStatus::Exists => Some(alert::EXISTS),
                FilenameStatus::Invalid => Some(alert::INVALID),
                FilenameStatus::Valid => None,
            };
            status_info(message, info)
        }
        Model::Confirm(confirm_state) => match confirm_state {
            ConfirmState::NewSession => status(confirm::NEW),
            ConfirmState::DeleteItem(_) => status(confirm::DELETE_ITEM),
            ConfirmState::DeleteFile(_) => status(confirm::DELETE_FILE),
        }
    };
    let mut spans = vec![" ".into()];
    spans.extend(content);
    Line::from(spans)
        .left_aligned()
        .set_style(style::ACCENT)
}

