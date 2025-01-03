mod cmdbar;
mod forest;
mod statusbar;
mod style;

use std::cmp::min;

use ratatui::{
    layout::{Constraint, Layout},
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

// Calculate the scroll offset.
fn calculate_offset(area_height: u16, list_size: usize, index: usize) -> u16 {
    let area_height = area_height as usize;
    let centered = index.saturating_sub(area_height / 2);
    let max_offset = list_size.saturating_sub(area_height);
    min(centered, max_offset) as u16
}

// Style the `text` to display in the main area.
fn style_main(text: Text) -> Paragraph {
    let block = Block::new()
        .borders(Borders::NONE)
        .padding(Padding::uniform(1));
    Paragraph::new(text)
        .block(block)
        .left_aligned()
        .set_style(style::DEFAULT)
}

// Return the confirm widget.
fn confirm(confirm_state: &ConfirmState) -> Paragraph {
    let text = match confirm_state {
        ConfirmState::NewSession => Text::default(),
        ConfirmState::DeleteItem(label, _) => Text::from(label.as_str()),
        ConfirmState::DeleteFile(load_state) => Text::from(load_state.filename()),
    };
    style_main(text)
}

// Return the load widget.
fn load(load_state: &LoadState, area_height: u16) -> Paragraph {
    let lines = load_state.filename_iter()
        .map(|(filename, highlight)| {
            if highlight {
                Line::styled(format!(" {filename} "), style::DEFAULT_HL)
            } else {
                Line::from(filename)
            }
        });
    let size = load_state.size();
    let index = load_state.index();
    let offset = calculate_offset(area_height, size, index);
    style_main(Text::from_iter(lines))
    .scroll((offset, 0))
}

// Return the text input widget given the `input` string.
fn text_input(input: &str) -> Paragraph {
    let content = format!("❯ {input}").into();
    let cursor = "█".set_style(style::CURSOR);
    let text = Line::from(vec![content, cursor])
        .set_style(style::DEFAULT)
        .into();
    style_main(text)
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
    style_main(Text::from(lines))
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
    style_main(Text::from(lines))
}

/// Render the UI on the `frame` based on the current `model`.
pub fn view(model: &Model, frame: &mut Frame) {
    let [status_bar_area, main_area, command_bar_area] =
        Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(frame.area());
    frame.render_widget(status_bar(model), status_bar_area);
    let Model { state, mode } = model;
    let SessionState { heap, .. } = state;
    match mode {
        Mode::Confirm(confirm_state) => {
            frame.render_widget(confirm(confirm_state), main_area);
        }
        Mode::Load(load_state) => {
            let area_height = main_area.height.saturating_sub(2);
            frame.render_widget(load(load_state, area_height), main_area);
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

