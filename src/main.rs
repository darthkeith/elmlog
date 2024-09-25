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

enum Mode {
    Normal,
    Input(String),
}

struct Model {
    heap: heap::Heap,
    mode: Mode,
    quit: bool,
}

enum Message {
    StartInput,
    AppendChar(char),
    SubmitInput,
    Quit,
    Nothing,
}

impl Model {
    fn new() -> Self {
        Model {
            heap: heap::empty(),
            mode: Mode::Normal,
            quit: false,
        }
    }
}

fn view(model: &Model, frame: &mut Frame) {
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
    let tree = Text::from_iter(heap::iter(&model.heap))
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

fn key_to_message(model: &Model, key: KeyCode) -> Message {
    match model.mode {
        Mode::Normal => match key {
            KeyCode::Char('i') => Message::StartInput,
            KeyCode::Char('q') => Message::Quit,
            _ => Message::Nothing,
        }
        Mode::Input(_) => match key {
            KeyCode::Char(c) => Message::AppendChar(c),
            KeyCode::Enter => Message::SubmitInput,
            _ => Message::Nothing,
        }
    }
}

fn handle_event(model: &Model) -> io::Result<Message> {
    let event::Event::Key(key) = event::read()? else {
        return Ok(Message::Nothing);
    };
    if key.kind != KeyEventKind::Press {
        return Ok(Message::Nothing);
    }
    Ok(key_to_message(model, key.code))
}

fn update(mut model: Model, msg: Message) -> Model {
    match model.mode {
        Mode::Normal => match msg {
            Message::StartInput => model.mode = Mode::Input(String::new()),
            Message::Quit => model.quit = true,
            _ => (),
        }
        Mode::Input(ref mut label) => match msg {
            Message::AppendChar(c) => label.push(c),
            Message::SubmitInput => {
                model.heap = heap::prepend(model.heap, label.clone());
                model.mode = Mode::Normal;
            }
            _ => (),
        }
    }
    model
}

fn main_loop(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut model = Model::new();
    while !model.quit {
        terminal.draw(|frame| view(&model, frame))?;
        let msg = handle_event(&model)?;
        model = update(model, msg);
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

