use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Stylize},
    text::{Line, Text},
    widgets::{
        block::Block,
        Paragraph,
    },
    Frame,
};

use crate::model::{
    heap::HeapStatus,
    Mode,
    Model,
};

// Return the top item widget using the current `model`.
fn top_item(model: &Model) -> Paragraph {
    let top_item_str = match model.heap.status() {
        HeapStatus::SingleRoot(label) => format!(" {label} "),
        _ => String::new(),
    };
    Paragraph::new(top_item_str.black().on_white().bold())
        .block(Block::bordered())
        .centered()
        .on_black()
}

// Return the forest widget using the current `model`.
fn forest(model: &Model) -> Text {
    let idx_len = match model.heap.size() {
        0 => 0,
        n => (n - 1).to_string().len(),
    };
    let format_line = |(i, label)| {
        format!(" {i:>width$}   {label}", width = idx_len)
    };
    let highlight = |index| {
        move |(i, label)| {
            if i == index {
                format_line((i, label)).add_modifier(Modifier::REVERSED)
            } else {
                format_line((i, label)).into()
            }
        }
    };
    let indexed_labels = model.heap.iter().enumerate();
    let forest_lines = match model.mode {
        Mode::Delete(index) => {
            Text::from_iter(indexed_labels.map(highlight(index)))
        }
        _ => Text::from_iter(indexed_labels.map(format_line)),
    };
    forest_lines
        .left_aligned()
        .on_black()
}

// Return the status widget using the current `model`.
fn status(model: &Model) -> Line {
    let status_msg = match model.mode {
        Mode::Normal => match model.heap.status() {
            HeapStatus::Empty => " Empty.".to_string(),
            HeapStatus::SingleRoot(_) => " Top item identified.".to_string(),
            HeapStatus::MultiRoot => " Merge to identify top item.".to_string(),
        }
        Mode::Input(ref label) => format!(" > {label}"),
        Mode::Delete(ref index) => format!(" Select index: {index}"),
        Mode::Merge => " Select item to promote.".to_string(),
    };
    Line::from(status_msg)
        .left_aligned()
        .on_dark_gray()
}

// Return the command key widget using the current `model`.
fn command_key(model: &Model) -> Line {
    let command_keys = match model.mode {
        Mode::Normal => vec![
            " I ".black().on_white().bold(),
            " Insert    ".italic(),
            " D ".black().on_white().bold(),
            " Delete    ".italic(),
            " M ".black().on_white().bold(),
            " Merge    ".italic(),
            " Q ".black().on_white().bold(),
            " Quit".italic(),
        ],
        Mode::Input(_) | Mode::Delete(_) => vec![
            " Enter ".black().on_white().bold(),
            " Submit    ".italic(),
            " Esc ".black().on_white().bold(),
            " Cancel ".italic(),
        ],
        Mode::Merge => vec![
            " Up ".black().on_white().bold(),
            " First    ".italic(),
            " Down ".black().on_white().bold(),
            " Second".italic(),
        ],
    };
    Line::from(command_keys)
        .centered()
        .on_black()
}

/// Render the UI on the `frame` using the current `model`.
pub fn view(model: &Model, frame: &mut Frame) {
    let [top_item_area, forest_area, status_area, command_key_area] =
        Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());
    frame.render_widget(top_item(model), top_item_area);
    frame.render_widget(forest(model), forest_area);
    frame.render_widget(status(model), status_area);
    frame.render_widget(command_key(model), command_key_area);
}

