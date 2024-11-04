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

use crate::{
    heap::{
        Heap,
        HeapStatus,
        NodePosition,
        NodeType,
        PreOrderIter,
    },
    model::{
        Choice,
        InputAction,
        Mode,
        Model,
        SessionState,
    },
};

// Represents a text block used for tree drawing.
enum IndentBlock {
    Spacer,
    VertBar,
}

// Indicates what style to apply to a label.
enum LabelType {
    SingleRoot,
    Root,
    Child,
}

// Iterator type returning the strings used to display the forest.
struct ForestIter<'a> {
    prefix: Vec<IndentBlock>,
    label_iter: PreOrderIter<'a>,
    single_root: bool,
}

impl<'a> ForestIter<'a> {
    fn new(heap: &'a Heap) -> Self {
        ForestIter {
            prefix: Vec::new(),
            label_iter: heap.iter(),
            single_root: matches!(heap.status(), HeapStatus::SingleRoot),
        }
    }
}

impl<'a> Iterator for ForestIter<'a> {
    type Item = (String, &'a str, LabelType);

    fn next(&mut self) -> Option<Self::Item> {
        let (label, pos) = self.label_iter.next()?;
        let NodePosition { node_type, is_last } = pos;
        let mut tree_row = String::new();
        if let NodeType::Root = node_type {
            self.prefix.clear();
            let label_type = match self.single_root {
                true => LabelType::SingleRoot,
                false => LabelType::Root,
            };
            return Some((tree_row, label, label_type));
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
        Some((tree_row, label, LabelType::Child))
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

// Return the forest widget based on the current `heap`.
fn forest(heap: &Heap) -> Paragraph {
    let lines = ForestIter::new(heap)
        .map(|(tree_row, label, label_type)| {
            let label_style = match label_type {
                LabelType::SingleRoot => style::SINGLE_ROOT,
                LabelType::Root => style::ROOT,
                LabelType::Child => style::DEFAULT,
            };
            Line::from(vec![
                Span::styled(tree_row, style::TREE),
                Span::styled(format!("{label} "), label_style),
            ])
        });
    style_text(Text::from_iter(lines))
}

// Return the forest widget with indicies, highlighting the selected item.
fn indexed_forest(heap: &Heap, selected: usize) -> Paragraph {
    let index_len = match heap.size() {
        0 => 0,
        n => (n - 1).to_string().len(),
    };
    let lines = ForestIter::new(heap)
        .enumerate()
        .map(|(i, (tree_row, label, label_type))| {
            let (tree_style, label_style) = if i == selected {(
                style::TREE_HL,
                match label_type {
                    LabelType::SingleRoot => style::SINGLE_ROOT_HL,
                    LabelType::Root => style::ROOT_HL,
                    LabelType::Child => style::DEFAULT_HL,
                })
            } else {(
                style::TREE,
                match label_type {
                    LabelType::SingleRoot => style::SINGLE_ROOT,
                    LabelType::Root => style::ROOT,
                    LabelType::Child => style::DEFAULT,
                })
            };
            let index = format!(" {i:>width$}   ", width = index_len);
            Line::from(vec!(
                Span::styled(index, label_style),
                Span::styled(tree_row, tree_style),
                Span::styled(format!("{label} "), label_style),
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
        Mode::Normal => match model.state.heap.status() {
            HeapStatus::Empty => status.push("Empty.".into()),
            HeapStatus::SingleRoot => status.push("Item selected.".into()),
            HeapStatus::MultiRoot(..) => {
                status.push("Items to compare: ".into());
                let n = model.state.heap.root_count();
                status.push(n.to_string().set_style(style::NUMBER));
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
fn normal_mode_commands(heap: &Heap) -> Vec<(&str, &str)> {
    let mut pairs = vec![("I", "Insert")];
    if heap.size() > 0 {
        pairs.push(("S", "Select"));
        if let HeapStatus::MultiRoot(..) = heap.status() {
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
        Mode::Normal => normal_mode_commands(&model.state.heap),
        Mode::Input(input_state) => {
            input_mode_commands(input_state.input.is_empty())
        }
        Mode::Select(_) => select_mode_commands(model.state.heap.size()),
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
    let Model { state, mode } = model;
    let SessionState { heap, .. } = state;
    match mode {
        Mode::Normal => {
            frame.render_widget(forest(heap), main_area);
        }
        Mode::Input(input_state) => {
            frame.render_widget(text_input(&input_state.input), main_area);
        }
        Mode::Select(index) | Mode::Selected(index) => {
            frame.render_widget(indexed_forest(heap, *index), main_area);
        }
        Mode::Compare(choice) => {
            frame.render_widget(compare(choice), main_area);
        }
    }
    frame.render_widget(status_bar(model), status_bar_area);
    frame.render_widget(command_bar(model), command_bar_area);
}

