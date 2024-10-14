use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Stylize},
    text::{Line, Text},
    widgets::{
        block::Padding,
        Block,
        Borders,
        Paragraph
    },
    Frame,
};

use crate::heap::{HeapStatus, NodePosition, NodeType, PreOrderIter};
use crate::model::{Choice, Mode, Model, Selected};

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
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let (label, pos) = self.label_iter.next()?;
        let NodePosition { node_type, is_last } = pos;
        if let NodeType::Root = node_type {
            self.prefix.clear();
            return Some(label.into());
        }
        let mut line = String::new();
        if let NodeType::Sibling = node_type {
            while let Some(IndentBlock::Spacer) = self.prefix.pop() {}
        }
        for block in &self.prefix {
            line.push_str(match block {
                IndentBlock::Spacer => "   ",
                IndentBlock::VertBar => " │ ",
            });
        }
        if is_last {
            line.push_str(" └─");
            self.prefix.push(IndentBlock::Spacer);
        } else {
            line.push_str(" ├─");
            self.prefix.push(IndentBlock::VertBar);
        }
        line.push_str(label);
        Some(line)
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
        .on_black()
}

// Return the forest widget using the current `model`.
fn forest(model: &Model) -> Paragraph {
    let lines = ForestIter::new(model)
        .map(|s| Line::from(s));
    style_text(Text::from_iter(lines))
}

// Return the forest widget with indicies, highlighting the selected item.
fn indexed_forest(model: &Model, selected: usize) -> Paragraph {
    let idx_len = match model.heap.size() {
        0 => 0,
        n => (n - 1).to_string().len(),
    };
    let lines = ForestIter::new(model)
        .enumerate()
        .map(|(i, s)| {
            let line = format!(" {i:>width$}   {s} ", width = idx_len);
            if i == selected {
                line.add_modifier(Modifier::REVERSED).into()
            } else {
                Line::from(line)
            }
        });
    style_text(Text::from_iter(lines))
}

// Return the text input widget given the `input` string.
fn text_input(input: &str) -> Paragraph {
    let text = format!(" > {input} ")
        .on_dark_gray()
        .into();
    style_text(text)
}

// Return the compare widget given a choice between two items.
fn compare<'a>(choice: &Choice) -> Paragraph<'a> {
    let Choice { item1, item2, selected } = choice;
    let line1 = Line::from(format!(" {item1} "));
    let line2 = Line::from(format!(" {item2} "));
    let lines = match selected {
        Selected::First => vec![
            line1.add_modifier(Modifier::REVERSED),
            line2,
        ],
        Selected::Second => vec![
            line1,
            line2.add_modifier(Modifier::REVERSED),
        ],
    };
    style_text(Text::from(lines))
}

// Return the status bar widget based on the current `model`.
fn status_bar(model: &Model) -> Line {
    let mut message = vec![" ".into()];
    match &model.mode {
        Mode::Normal => match model.heap.status() {
            HeapStatus::Empty => message.push("Empty.".into()),
            HeapStatus::SingleRoot => message.push("Item selected.".into()),
            HeapStatus::MultiRoot(..) => {
                message.push("Items to compare: ".into());
                let n = model.heap.root_count();
                message.push(n.to_string().bold());
            }
        }
        Mode::Input(_) => message.push("Enter new item.".into()),
        Mode::Select(index) => {
            message.push("Selected index: ".into());
            message.push(index.to_string().bold());
        }
        Mode::Compare(_) => message.push("Select item to promote.".into()),
    };
    Line::from(message)
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

// Return the select mode key-command pairs.
fn select_mode_commands(model: &Model) -> Vec<(&str, &str)> {
    let mut pairs = Vec::new();
    if model.heap.size() > 1 {
        pairs.push(("0-9", "Jump"));
        pairs.push(("Space │ ↓", "Down"));
        pairs.push(("Bksp │ ↑", "Up"));
    }
    pairs.push(("D", "Delete"));
    pairs.push(("Esc", "Cancel"));
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
        Mode::Select(_) => select_mode_commands(model),
        Mode::Compare(_) => vec![
            ("Tab", "Toggle"),
            ("Enter", "Confirm"),
            ("Esc", "Cancel"),
        ],
    };
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
        Mode::Input(input) => {
            frame.render_widget(text_input(input), main_area);
        }
        Mode::Select(index) => {
            frame.render_widget(indexed_forest(model, *index), main_area);
        }
        Mode::Compare(choice) => {
            frame.render_widget(compare(choice), main_area);
        }
    }
    frame.render_widget(status_bar(model), status_bar_area);
    frame.render_widget(command_bar(model), command_bar_area);
}

