mod cmdbar;
mod forest;
mod statusbar;
mod style;

use std::cmp::min;

use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect, Widget},
    style::Styled,
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
        CompareState,
        ConfirmState,
        Mode,
        Model,
        SessionState,
    },
};

use self::{
    cmdbar::command_bar,
    forest::{
        forest_normal,
        forest_select,
        forest_selected,
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
    area_height: u16,
    list_size: usize,
    index: usize
) -> ScrollInfo {
    let area_height = area_height as usize;
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

impl<'a> Widget for Scroll<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top_line, mid_area, bottom_line] = top_mid_bottom(area);
        let Scroll { text, list_size, index } = self;
        let ScrollInfo { offset, is_more_above, is_more_below } =
            compute_scroll_info(mid_area.height, list_size, index);
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

// Return the confirm widget.
fn confirm(confirm_state: &ConfirmState) -> Paragraph {
    let text = match confirm_state {
        ConfirmState::NewSession => Text::default(),
        ConfirmState::DeleteItem(label, _) => Text::from(label.as_str()),
        ConfirmState::DeleteFile(load_state) => Text::from(load_state.filename()),
    };
    main_paragraph(text)
}

// Return the load widget.
fn load(load_state: &LoadState) -> Scroll {
    let selected = load_state.index();
    let index_len = match load_state.size() {
        0 => 0,
        n => (n - 1).to_string().len(),
    };
    let lines = load_state.filename_iter()
        .enumerate()
        .map(|(i, filename)| {
            let line_style = match i == selected {
                true => style::DEFAULT_HL,
                false => style::DEFAULT,
            };
            let text = format!(" {i:>width$}   {filename}", width = index_len);
            Line::styled(text, line_style)
        });
    Scroll {
        text: Text::from_iter(lines),
        list_size: load_state.size(),
        index: load_state.index(),
    }
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

// Return the compare widget given a choice between two items.
fn compare<'a>(cmp_state: &CompareState) -> Paragraph<'a> {
    let CompareState { item1, item2, first } = cmp_state;
    let line1 = Line::from(format!(" {item1} "));
    let line2 = Line::from(format!(" {item2} "));
    let lines = match first {
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
    let SessionState { heap, .. } = state;
    match mode {
        Mode::Confirm(confirm_state) => {
            frame.render_widget(confirm(confirm_state), main_area);
        }
        Mode::Load(load_state) => {
            frame.render_widget(load(load_state), main_area);
        }
        Mode::Normal => {
            frame.render_widget(forest_normal(heap), main_area);
        }
        Mode::Input(input_state) => {
            frame.render_widget(text_input(input_state.input()), main_area);
        }
        Mode::Select(index) => {
            frame.render_widget(forest_select(heap, *index), main_area);
        }
        Mode::Selected(index) => {
            frame.render_widget(forest_selected(heap, *index), main_area);
        }
        Mode::Compare(compare_state) => {
            frame.render_widget(compare(compare_state), main_area);
        }
        Mode::Save(save_state) => {
            frame.render_widget(save_query(save_state.save), main_area);
        }
    }
    frame.render_widget(command_bar(model), command_bar_area);
}

