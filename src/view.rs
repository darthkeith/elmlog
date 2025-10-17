mod cmdbar;
mod forest;
mod statusbar;
mod style;

use std::cmp::min;

use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect, Widget},
    style::{Style, Styled},
    text::{Line, Text},
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
    io::LoadState,
    model::{
        ConfirmState,
        Mode,
        Model,
        SessionState,
    },
};

use self::{
    cmdbar::command_bar,
    forest::{
        forest_delete,
        forest_edit,
        forest_input,
        forest_normal,
    },
    statusbar::status_bar,
};

// Scroll offset and flags for scrolling indicators.
struct ScrollInfo {
    offset: u16,
    is_more_above: bool,
    is_more_below: bool,
}

// A widget containing scrolling text.
struct Scroll<'a> {
    text: Text<'a>,
    list_size: usize,
    index: usize,
}

// Calculate the scroll offset and other scroll info.
fn compute_scroll_info(
    area_height: usize,
    list_size: usize,
    index: usize
) -> ScrollInfo {
    let centered = index.saturating_sub(area_height / 2);
    let max_offset = list_size.saturating_sub(area_height);
    let offset = min(centered, max_offset);
    ScrollInfo {
        offset: offset as u16,
        is_more_above: offset > 0,
        is_more_below: offset < max_offset,
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

impl Widget for Scroll<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top_line, mid_area, bottom_line] = top_mid_bottom(area);
        let Scroll { text, list_size, index } = self;
        let ScrollInfo { offset, is_more_above, is_more_below } =
            compute_scroll_info(mid_area.height as usize, list_size, index);
        main_paragraph_scroll(text)
            .scroll((offset, 0))
            .render(mid_area, buf);
        let scroll_hint = |is_more: bool| if is_more { " ..." } else { "" };
        Text::from(scroll_hint(is_more_above))
            .style(style::DEFAULT)
            .render(top_line, buf);
        Text::from(scroll_hint(is_more_below))
            .style(style::DEFAULT)
            .render(bottom_line, buf);
    }
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

// Style the `text` to display in the main area for scrolling.
fn main_paragraph_scroll(text: Text) -> Paragraph {
    pad_main_paragraph(text, Padding::horizontal(1))
}

// Return Load mode Scroll with selected file highlighted.
fn load(load_state: &LoadState, highlight: Style) -> Scroll {
    let selected = load_state.index();
    let lines = load_state.filename_iter()
        .enumerate()
        .map(|(i, filename)| {
            let text = format!(" {filename} ");
            let line_style = if i == selected { highlight } else { style::DEFAULT };
            Line::styled(text, line_style)
        });
    Scroll {
        text: Text::from_iter(lines),
        list_size: load_state.size(),
        index: load_state.index(),
    }
}

// Return Load mode Scroll with normal highlight.
fn load_normal(load_state: &LoadState) -> Scroll {
    load(load_state, style::DEFAULT_HL)
}

// Return Load mode Scroll with selected file highlighted in red for deletion.
fn load_delete(load_state: &LoadState) -> Scroll {
    load(load_state, style::DELETE)
}

// Return the text input widget given the `input` string.
fn text_input(input: &str) -> Paragraph {
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
    let Model { state, mode } = model;
    let SessionState { focus, .. } = state;
    match mode {
        Mode::Confirm(confirm_state) => match confirm_state {
            ConfirmState::NewSession => {
                let empty = main_paragraph(Text::default());
                frame.render_widget(empty, main_area);
            }
            ConfirmState::DeleteItem => {
                frame.render_widget(forest_delete(focus.as_ref()), main_area);
            }
            ConfirmState::DeleteFile(load_state) => {
                frame.render_widget(load_delete(load_state), main_area);
            }
        }
        Mode::Load(load_state) => {
            frame.render_widget(load_normal(load_state), main_area);
        }
        Mode::Normal => {
            frame.render_widget(forest_normal(focus.as_ref()), main_area);
        }
        Mode::LabelInput(label_state) => {
            let forest = forest_input(focus.as_ref(), &label_state.input);
            frame.render_widget(forest, main_area);
        }
        Mode::FilenameInput(filename_state) => {
            frame.render_widget(text_input(&filename_state.input), main_area);
        }
        Mode::Edit | Mode::Move | Mode::Insert => {
            frame.render_widget(forest_edit(focus.as_ref()), main_area);
        }
        Mode::Save(save_state) => {
            frame.render_widget(save_query(save_state.save), main_area);
        }
    }
    frame.render_widget(command_bar(model), command_bar_area);
}

