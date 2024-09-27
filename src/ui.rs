use ratatui::{
    layout::{Constraint, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::{
        block::Block,
        Borders,
        Paragraph,
    },
    Frame,
};

use crate::{Mode, Model};
use crate::heap::HeapStatus;

pub fn view(model: &Model, frame: &mut Frame) {
    let [top_item_area, tree_area, status_area, command_key_area] =
        Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());
    let top_item_str = match model.heap.status() {
        HeapStatus::SingleRoot(label) => format!(" {label} "),
        _ => "".to_string(),
    };
    let top_item = Paragraph::new(top_item_str.black().on_white().bold())
        .block(Block::new().borders(Borders::ALL))
        .centered()
        .on_black();
    let idx_len = match model.heap.size() {
        0 => 0,
        n => (n - 1).to_string().len(),
    };
    let tree_lines = model.heap.iter()
        .enumerate()
        .map(|(i, label)| format!(" {i:>width$}   {label}", width = idx_len));
    let tree = Text::from_iter(tree_lines)
        .left_aligned()
        .on_black();
    let status_msg = match model.mode {
        Mode::Normal => match model.heap.status() {
            HeapStatus::Empty => " Empty.".to_string(),
            HeapStatus::SingleRoot(_) => " Top item identified.".to_string(),
            HeapStatus::MultiRoot => " Merge to identify top item.".to_string(),
        }
        Mode::Input(ref label) => format!(" > {label}"),
    };
    let status = Line::from(status_msg)
        .left_aligned()
        .on_dark_gray();
    let command_keys = match model.mode {
        Mode::Normal => vec![
            " I ".black().on_white().bold(),
            " Insert    ".italic(),
            " D ".black().on_white().bold(),
            " Delete    ".italic(),
            " Q ".black().on_white().bold(),
            " Quit".italic(),
        ],
        Mode::Input(_) => vec![
            " Enter ".black().on_white().bold(),
            " Submit    ".italic(),
            " Esc ".black().on_white().bold(),
            " Cancel ".italic(),
        ],
    };
    let command_key = Line::from(command_keys)
        .centered()
        .on_black();
    frame.render_widget(top_item, top_item_area);
    frame.render_widget(tree, tree_area);
    frame.render_widget(status, status_area);
    frame.render_widget(command_key, command_key_area);
}

