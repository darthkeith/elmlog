use ratatui::{
    style::{Styled, Stylize},
    text::Line,
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

/// Return the status bar widget based on the `model`.
pub fn status_bar(model: &Model) -> Line {
    let mut status = vec![" ".into()];
    match &model.mode {
        Mode::Load(_) => status.push("Select file to open.".into()),
        Mode::Normal => match model.state.heap.status() {
            HeapStatus::Empty => status.push("Empty.".into()),
            HeapStatus::SingleRoot => status.push("Item selected.".into()),
            HeapStatus::MultiRoot(..) => {
                status.push("Items to compare: ".into());
                let n = model.state.heap.root_count();
                status.push(n.to_string().set_style(style::NUMBER));
            }
        }
        Mode::Input(input_state) => match &input_state.action {
            InputAction::Insert => match input_state.input.is_empty() {
                true => status.push("Enter new item | [Empty]".into()),
                false => status.push("Enter new item".into()),
            }
            InputAction::Edit(_) => match input_state.input.is_empty() {
                true => status.push("Edit item | [Empty]".into()),
                false => status.push("Edit item".into()),
            }
            InputAction::Save(file_name_status) => match file_name_status {
                FileNameStatus::Empty => {
                    status.push("Enter file name | [Empty]".into())
                }
                FileNameStatus::Exists => {
                    status.push("Enter file name | [File Exists]".into())
                }
                FileNameStatus::Unique => {
                    status.push("Enter file name".into())
                }
            }
        }
        Mode::Select(index) => {
            status.push("Selected index: ".into());
            status.push(index.to_string().bold());
        }
        Mode::Selected(_) => status.push("Enter command.".into()),
        Mode::Compare(_) => status.push("Select item to promote.".into()),
        Mode::Save(_) => status.push("Save changes before quitting?".into()),
    };
    Line::from(status)
        .left_aligned()
        .set_style(style::ACCENT)
}

