mod cmdbar;
mod forest;
mod scroll;
mod statusbar;
mod style;

use std::cmp::min;

use ratatui::{
    layout::{Constraint, Layout},
    prelude::Rect,
    style::{Style, Styled},
    text::{Line, Text},
    widgets::{
        Padding,
        Block,
        Borders,
        Paragraph,
        Wrap,
    },
    Frame,
};

use crate::{
    model::{ConfirmState, LoadState, Model},
    view::scroll::{ScrollArea, ScrollContent},
};

use self::{
    cmdbar::command_bar,
    statusbar::status_bar,
};

const INDENT: &str      = "  ";
const SCROLL_HINT: &str = "  ...";

// Start index and indicators used for scrolling through a file list.
struct ScrollInfo {
    start: usize,
    more_above: bool,
    more_below: bool,
}

fn compute_scroll_info(
    area_height: usize,
    list_size: usize,
    index: usize
) -> ScrollInfo {
    let centered = index.saturating_sub(area_height / 2);
    let max_start = list_size.saturating_sub(area_height);
    let start = min(centered, max_start);
    ScrollInfo {
        start,
        more_above: start > 0,
        more_below: start < max_start,
    }
}

// Divide the `area` into top/bottom lines and middle area.
fn top_mid_bottom(area: Rect) -> [Rect; 3] {
    Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(area)
}

// Create a paragraph with the `text` and `padding`.
fn pad_main_paragraph(text: Text, padding: Padding) -> Paragraph {
    let block = Block::new()
        .borders(Borders::NONE)
        .padding(padding);
    Paragraph::new(text)
        .block(block)
        .left_aligned()
        .set_style(style::DEFAULT)
}

// Style the `text` to display in the main area.
fn main_paragraph(text: Text) -> Paragraph {
    pad_main_paragraph(text, Padding::uniform(1))
}

fn load<'a>(
    load_state: &'a LoadState,
    highlight: Style
) -> ScrollArea<'a, impl FnOnce(usize) -> ScrollContent<'a> + 'a> {
    let build = move |area_height| {
        let size = load_state.files.len();
        let index = load_state.index;
        let ScrollInfo {
            start,
            more_above,
            more_below,
        } = compute_scroll_info(area_height, size, index);
        let end = std::cmp::min(start + area_height, size);
        let selected = index - start;
        let lines = load_state.files[start..end]
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let line_style = if i == selected {
                    highlight
                } else {
                    style::DEFAULT
                };
                Line::styled(format!("{INDENT}{}", entry.name), line_style)
            });
        ScrollContent {
            text: Text::from_iter(lines),
            more_above,
            more_below,
        }
    };
    ScrollArea { build }
}

// Return Load mode widget with normal highlight.
fn load_normal<'a>(
    load_state: &'a LoadState
) -> ScrollArea<'a, impl FnOnce(usize) -> ScrollContent<'a> + 'a> {
    load(load_state, style::DEFAULT_HL)
}

// Return Load mode widget with highlight for deletion.
fn load_delete<'a>(
    load_state: &'a LoadState
) -> ScrollArea<'a, impl FnOnce(usize) -> ScrollContent<'a> + 'a> {
    load(load_state, style::DELETE)
}

// Return the text input widget given the `input` string.
fn text_input(input: &str) -> Paragraph<'static> {
    let content = format!("❯ {input}").into();
    let cursor = "█".set_style(style::CURSOR);
    let text = Line::from(vec![content, cursor])
        .set_style(style::DEFAULT)
        .into();
    main_paragraph(text)
        .wrap(Wrap { trim: false })
}

// Return the save query widget.
fn save_query(save: bool) -> Paragraph<'static> {
    let line1 = Line::from(" Save ");
    let line2 = Line::from(" Discard Changes ");
    let lines = match save {
        true => vec![
            line1.set_style(style::DEFAULT_HL),
            line2,
        ],
        false => vec![
            line1,
            line2.set_style(style::DEFAULT_HL),
        ],
    };
    main_paragraph(Text::from(lines))
}

/// Render the UI on the `frame` based on the current `model`.
pub fn view(model: &Model, frame: &mut Frame) {
    let [
        status_bar_area,
        main_area,
        command_bar_area
    ] = top_mid_bottom(frame.area());
    frame.render_widget(status_bar(model), status_bar_area);
    match model {
        Model::Load(load_state) =>
            frame.render_widget(load_normal(load_state), main_area),
        Model::Normal(state) => {
            let forest = forest::normal(state.focus());
            frame.render_widget(forest, main_area);
        }
        Model::Insert(state) => {
            let forest = forest::insert(state.focus());
            frame.render_widget(forest, main_area);
        }
        Model::Move(state) => {
            let forest = forest::move_mode(state.focus());
            frame.render_widget(forest, main_area);
        }
        Model::Save(save_state) =>
            frame.render_widget(save_query(save_state.save), main_area),
        Model::LabelInput(label_state) => {
            let focus = label_state.session.focus();
            let forest = forest::input(focus, &label_state.input);
            frame.render_widget(forest, main_area);
        }
        Model::FilenameInput(filename_state) =>
            frame.render_widget(text_input(&filename_state.input), main_area),
        Model::Confirm(confirm_state) => match confirm_state {
            ConfirmState::NewSession => {
                let empty = main_paragraph(Text::default());
                frame.render_widget(empty, main_area);
            }
            ConfirmState::DeleteItem(state) => {
                let forest = forest::delete(state.focus());
                frame.render_widget(forest, main_area);
            }
            ConfirmState::DeleteFile(load_state) =>
                frame.render_widget(load_delete(load_state), main_area),
        }
    }
    frame.render_widget(command_bar(model), command_bar_area);
}
