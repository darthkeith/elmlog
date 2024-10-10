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

use crate::heap::{HeapStatus, NodePosition, NodeType};
use crate::model::{Mode, Model};

// Represents a text block used for tree drawing.
enum IndentBlock {
    Spacer,
    VertBar,
}

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
    let mut prefix: Vec<IndentBlock> = Vec::new();
    let mut lines: Vec<Line> = Vec::new();
    let highlight_idx = match model.mode {
        Mode::Select(index) => index,
        _ => model.heap.size(),  // No highlight
    };
    for (i, (label, pos)) in model.heap.iter().enumerate() {
        let NodePosition { node_type, is_last } = pos;
        let mut line = format!(" {i:>width$}   ", width = idx_len);
        if let NodeType::Root = node_type {
            prefix.clear();
            line.push_str(label);
        } else {
            if let NodeType::Sibling = node_type {
                while let Some(IndentBlock::Spacer) = prefix.pop() {}
            }
            for block in &prefix {
                line.push_str(match block {
                    IndentBlock::Spacer => "   ",
                    IndentBlock::VertBar => " │ ",
                });
            }
            if is_last {
                line.push_str(" └─");
                line.push_str(label);
                prefix.push(IndentBlock::Spacer);
            } else {
                line.push_str(" ├─");
                line.push_str(label);
                prefix.push(IndentBlock::VertBar);
            }
        }
        lines.push(
            if i == highlight_idx {
                line.add_modifier(Modifier::REVERSED).into()
            } else {
                line.into()
            }
        );
    }
    Text::from(lines)
        .left_aligned()
        .on_black()
}

// Return the status bar widget using the current `model`.
fn status_bar(model: &Model) -> Line {
    let status_msg = match model.mode {
        Mode::Normal => match model.heap.status() {
            HeapStatus::Empty => " Empty.".to_string(),
            HeapStatus::SingleRoot(_) => " Item selected.".to_string(),
            HeapStatus::MultiRoot => {
                let n = model.heap.root_count();
                format!(" {n} items to compare.")
            }
        }
        Mode::Input(ref label) => format!(" > {label}"),
        Mode::Select(ref index) => format!(" Select index: {index}"),
        Mode::Compare => " Select item to promote.".to_string(),
    };
    Line::from(status_msg)
        .left_aligned()
        .on_dark_gray()
}

// Convert key-command pairs into a command bar.
fn to_command_bar<'a>(pairs: Vec<(&'a str, &'a str)>) -> Line<'a> {
    let mut text_spans = Vec::new();
    for (key, command) in pairs {
        text_spans.push(format!(" {key} ").black().on_white().bold());
        text_spans.push(format!(" {command}").italic());
        text_spans.push("    ".into());
    }
    text_spans.pop();  // Remove extra spacer at end
    Line::from(text_spans)
        .centered()
        .on_black()
}

// Return the key-command pairs in normal mode.
fn normal_mode_commands(model: &Model) -> Vec<(&str, &str)> {
    let mut pairs = vec![("I", "Insert")];
    if model.heap.size() > 0 {
        pairs.push(("S", "Select"));
        if let HeapStatus::MultiRoot = model.heap.status() {
            pairs.push(("C", "Compare"));
        }
    }
    pairs.push(("Q", "Quit"));
    pairs
}

// Return the command bar widget based on the current `model`.
fn command_bar(model: &Model) -> Line {
    let pairs = match model.mode {
        Mode::Normal => normal_mode_commands(model),
        Mode::Input(_) => vec![
            ("Enter", "Submit"),
            ("Esc", "Cancel"),
        ],
        Mode::Select(_) => vec![
            ("D", "Delete"),
            ("Esc", "Cancel"),
        ],
        Mode::Compare => vec![
            ("Up", "First"),
            ("Down", "Second"),
            ("Esc", "Cancel"),
        ],
    };
    to_command_bar(pairs)
}

/// Render the UI on the `frame` using the current `model`.
pub fn view(model: &Model, frame: &mut Frame) {
    let [top_item_area, forest_area, status_bar_area, command_bar_area] =
        Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());
    frame.render_widget(top_item(model), top_item_area);
    frame.render_widget(forest(model), forest_area);
    frame.render_widget(status_bar(model), status_bar_area);
    frame.render_widget(command_bar(model), command_bar_area);
}

