mod heap;
use std::io;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::{
        block::Block,
        Borders,
        Paragraph,
    },
    DefaultTerminal,
    Frame,
};

struct Model {
    quit: bool,
}

enum Message {
    Quit,
    Nothing,
}

impl Model {
    fn new() -> Self {
        Model {
            quit: false,
        }
    }
}

fn view(_model: &Model, frame: &mut Frame) {
    let [top_item_area, tree_area, status_area, command_key_area] =
        Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());
    let top_item = Paragraph::new("Top Item".bold())
        .block(Block::new().borders(Borders::ALL))
        .centered()
        .on_black();
    let tree = Text::from(vec![
            Line::from(" 0  Item A"),
            Line::from(" 1  ├──Item B"),
            Line::from(" 2  ├──Item C"),
            Line::from(" 3  │  └──Item D"),
            Line::from(" 4  └──Item E"),
            Line::from(" 5     ├──Item F"),
            Line::from(" 6     └──Item G"),
        ])
        .left_aligned()
        .on_black();
    let status = Line::from(" Top item selected.")
        .left_aligned()
        .on_dark_gray();
    let command_key = Line::from(vec![
            " I ".black().on_white().bold(),
            " Insert    ".italic(),
            " D ".black().on_white().bold(),
            " Delete    ".italic(),
            " Q ".black().on_white().bold(),
            " Quit".italic(),
        ])
        .centered()
        .on_black();
    frame.render_widget(top_item, top_item_area);
    frame.render_widget(tree, tree_area);
    frame.render_widget(status, status_area);
    frame.render_widget(command_key, command_key_area);
}

fn handle_event() -> io::Result<Message> {
    let event::Event::Key(key) = event::read()? else {
        return Ok(Message::Nothing);
    };
    if key.kind != KeyEventKind::Press {
        return Ok(Message::Nothing);
    }
    match key.code {
        KeyCode::Char('q') => Ok(Message::Quit),
        _ => Ok(Message::Nothing),
    }
}

fn update(model: &mut Model, msg: Message) {
    if let Message::Quit = msg {
        model.quit = true;
    }
}

fn main_loop(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut model = Model::new();
    while !model.quit {
        terminal.draw(|frame| view(&model, frame))?;
        let msg = handle_event()?;
        update(&mut model, msg);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let result = main_loop(terminal);
    ratatui::restore();
    result
}

