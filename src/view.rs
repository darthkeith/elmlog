mod style;

use ratatui::{
    layout::{Constraint, Layout},
    style::{Styled, Stylize},
    text::{Line, Span, Text},
    widgets::{
        block::Padding,
        Block,
        Borders,
        Paragraph,
        Wrap,
    },
    Frame,
};

use crate::heap::{
    HeapStatus,
    NodePosition,
    NodeType,
    PreOrderIter,
};
use crate::model::{
    Choice,
    InputAction,
    Mode,
    Model,
};

// Represents a text block used for tree drawing.
enum IndentBlock {
    Spacer,
    VertBar,
}

// Iterator type returning the strings used to display the forest.
struct ForestIter<'a> {
    prefix: Vec<IndentBlock>,
    label_iter: PreOrderIter<'a>,
}

impl<'a> ForestIter<'a> {
    fn new(model: &'a Model) -> Self {
        ForestIter {
            prefix: Vec::new(),
            label_iter: model.heap.iter(),
        }
    }
}

impl<'a> Iterator for ForestIter<'a> {
    type Item = (String, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let (label, pos) = self.label_iter.next()?;
        let NodePosition { node_type, is_last } = pos;
        let mut tree_row = String::new();
        if let NodeType::Root = node_type {
            self.prefix.clear();
            return Some((tree_row, label));
        }
        if let NodeType::Sibling = node_type {
            while let Some(IndentBlock::Spacer) = self.prefix.pop() {}
        }
        for block in &self.prefix {
            tree_row.push_str(match block {
                IndentBlock::Spacer => "   ",
                IndentBlock::VertBar => "│  ",
            });
        }
        if is_last {
            tree_row.push_str("└──");
            self.prefix.push(IndentBlock::Spacer);
        } else {
            tree_row.push_str("├──");
            self.prefix.push(IndentBlock::VertBar);
        }
        Some((tree_row, label))
    }
}

// Style text for the main area.
fn style_text(text: Text) -> Paragraph {
    let block = Block::new()
        .borders(Borders::NONE)
        .padding(Padding::uniform(1));
    Paragraph::new(text)
        .block(block)
        .left_aligned()
        .set_style(style::DEFAULT)
}

// Return the forest widget using the current `model`.
fn forest(model: &Model) -> Paragraph {
    let to_line = |(tree_row, label)| {
        Line::from(vec![
            Span::styled(tree_row, style::TREE),
            Span::styled(format!("{label} "), style::DEFAULT),
        ])
    };
    let lines = ForestIter::new(model)
        .map(to_line);
    let text = if let HeapStatus::SingleRoot = model.heap.status() {
        let lines2 = lines.enumerate()
            .map(|(i, line)| {
                match i {
                    0 => line.bold(),
                    _ => line,
                }
            });
        Text::from_iter(lines2)
    } else {
        Text::from_iter(lines)
    };
    style_text(text)
}

// Return the forest widget with indicies, highlighting the selected item.
fn indexed_forest(model: &Model, selected: usize) -> Paragraph {
    let idx_len = match model.heap.size() {
        0 => 0,
        n => (n - 1).to_string().len(),
    };
    let lines = ForestIter::new(model)
        .enumerate()
        .map(|(i, (tree_row, label))| {
            let idx = format!(" {i:>width$}   ", width = idx_len);
            let (tree_style, text_style) = match i == selected {
                true => (style::TREE_HL, style::DEFAULT_HL),
                false => (style::TREE, style::DEFAULT),
            };
            Line::from(vec!(
                Span::styled(idx, text_style),
                Span::styled(tree_row, tree_style),
                Span::styled(format!("{label} "), text_style),
            ))
        });
    style_text(Text::from_iter(lines))
}

// Return the text input widget given the `input` string.
fn text_input(input: &str) -> Paragraph {
    let content = format!("❯ {input}").into();
    let cursor = "█".set_style(style::CURSOR);
    let text = Line::from(vec![content, cursor])
        .set_style(style::DEFAULT)
        .into();
    style_text(text)
        .wrap(Wrap { trim: false })
}

// Return the compare widget given a choice between two items.
fn compare<'a>(choice: &Choice) -> Paragraph<'a> {
    let Choice { item1, item2, first_selected } = choice;
    let line1 = Line::from(format!(" {item1} "));
    let line2 = Line::from(format!(" {item2} "));
    let lines = match first_selected {
        true => vec![
            line1.set_style(style::DEFAULT_HL),
            line2,
        ],
        false => vec![
            line1,
            line2.set_style(style::DEFAULT_HL),
        ],
    };
    style_text(Text::from(lines))
}

// Return the status bar widget based on the current `model`.
fn status_bar(model: &Model) -> Line {
    let mut status = vec![" ".into()];
    match &model.mode {
        Mode::Normal => match model.heap.status() {
            HeapStatus::Empty => status.push("Empty.".into()),
            HeapStatus::SingleRoot => status.push("Item selected.".into()),
            HeapStatus::MultiRoot(..) => {
                status.push("Items to compare: ".into());
                let n = model.heap.root_count();
                status.push(n.to_string().bold());
            }
        }
        Mode::Input(state) => match state.action {
            InputAction::Insert => status.push("Enter new item.".into()),
            InputAction::Edit(_) => status.push("Edit item.".into()),
        }
        Mode::Select(index) => {
            status.push("Selected index: ".into());
            status.push(index.to_string().bold());
        }
        Mode::Selected(_) => status.push("Enter command.".into()),
        Mode::Compare(_) => status.push("Select item to promote.".into()),
    };
    Line::from(status)
        .left_aligned()
        .set_style(style::ACCENT)
}

// Return the normal mode key-command pairs.
fn normal_mode_commands(model: &Model) -> Vec<(&str, &str)> {
    let mut pairs = vec![("I", "Insert")];
    if model.heap.size() > 0 {
        pairs.push(("S", "Select"));
        if let HeapStatus::MultiRoot(..) = model.heap.status() {
            pairs.push(("C", "Compare"));
        }
    }
    pairs.push(("Q", "Quit"));
    pairs
}

// Return the input mode key-command pairs.
fn input_mode_commands(empty: bool) -> Vec<(&'static str, &'static str)> {
    match empty {
        true => Vec::new(),
        false => vec![("Enter", "Submit")],
    }
}

// Return the select mode key-command pairs.
fn select_mode_commands(size: usize) -> Vec<(&'static str, &'static str)> {
    let mut pairs = Vec::new();
    if size > 1 {
        pairs.push(("0-9", "Jump"));
        pairs.push(("K │ ↑", "Up"));
        pairs.push(("J │ ↓", "Down"));
    }
    pairs.push(("Enter", "Confirm"));
    pairs
}

// Convert key-command pairs into a command bar.
fn to_command_bar<'a>(pairs: Vec<(&'a str, &'a str)>) -> Line<'a> {
    let mut text_spans = Vec::new();
    for (key, command) in pairs {
        text_spans.push(format!(" {key} ").set_style(style::CMD_KEY));
        text_spans.push(format!(" {command}").set_style(style::CMD_NAME));
        text_spans.push("    ".into());
    }
    text_spans.pop();  // Remove extra spacer at end
    Line::from(text_spans)
        .centered()
        .set_style(style::DEFAULT)
}

// Return the command bar widget based on the current `model`.
fn command_bar(model: &Model) -> Line {
    let mut pairs = match &model.mode {
        Mode::Normal => normal_mode_commands(model),
        Mode::Input(state) => input_mode_commands(state.input.is_empty()),
        Mode::Select(_) => select_mode_commands(model.heap.size()),
        Mode::Selected(_) => vec![
            ("E", "Edit"),
            ("D", "Delete"),
        ],
        Mode::Compare(_) => vec![
            ("Space", "Toggle"),
            ("Enter", "Confirm"),
        ],
    };
    if !matches!(model.mode, Mode::Normal) {
        pairs.push(("Esc", "Cancel"));
    }
    to_command_bar(pairs)
}

/// Render the UI on the `frame` based on the current `model`.
pub fn view(model: &Model, frame: &mut Frame) {
    let [main_area, status_bar_area, command_bar_area] =
        Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());
    match &model.mode {
        Mode::Normal => {
            frame.render_widget(forest(model), main_area);
        }
        Mode::Input(state) => {
            frame.render_widget(text_input(&state.input), main_area);
        }
        Mode::Select(index) | Mode::Selected(index) => {
            frame.render_widget(indexed_forest(model, *index), main_area);
        }
        Mode::Compare(choice) => {
            frame.render_widget(compare(choice), main_area);
        }
    }
    frame.render_widget(status_bar(model), status_bar_area);
    frame.render_widget(command_bar(model), command_bar_area);
}

