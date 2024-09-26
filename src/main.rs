mod heap;

use heap::HeapStatus;
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

enum Edit {
    AppendChar(char),
    PopChar,
}

enum Message {
    StartInput,
    EditInput(Edit),
    SubmitInput,
    Cancel,
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
    let top_item_str = match heap::status(&model.heap) {
        HeapStatus::SingleRoot(label) => format!(" {label} "),
        _ => "".to_string(),
    };
    let top_item = Paragraph::new(top_item_str.black().on_white().bold())
        .block(Block::new().borders(Borders::ALL))
        .centered()
        .on_black();
    let tree = Text::from_iter(heap::iter(&model.heap))
        .left_aligned()
        .on_black();
    let status_msg = match model.mode {
        Mode::Normal => match heap::status(&model.heap) {
            HeapStatus::Empty => " Empty.".to_string(),
            HeapStatus::SingleRoot(_) => " Top item identified.".to_string(),
            HeapStatus::MultiRoot => " Merge to identify top item.".to_string(),
        }
        Mode::Input(ref label) => format!(" > {label}"),
    };
    let status = Line::from(status_msg)
        .left_aligned()
        .on_dark_gray();
    let command_keys = match model.mode {
        Mode::Normal => vec![
            " I ".black().on_white().bold(),
            " Insert    ".italic(),
            " D ".black().on_white().bold(),
            " Delete    ".italic(),
            " Q ".black().on_white().bold(),
            " Quit".italic(),
        ],
        Mode::Input(_) => vec![
            " Enter ".black().on_white().bold(),
            " Submit    ".italic(),
            " Esc ".black().on_white().bold(),
            " Cancel ".italic(),
        ],
    };
    let command_key = Line::from(command_keys)
        .centered()
        .on_black();
    frame.render_widget(top_item, top_item_area);
    frame.render_widget(tree, tree_area);
    frame.render_widget(status, status_area);
    frame.render_widget(command_key, command_key_area);
}

fn key_to_message(mode: &Mode, key: KeyCode) -> Message {
    match mode {
        Mode::Normal => match key {
            KeyCode::Char('i') => Message::StartInput,
            KeyCode::Char('q') => Message::Quit,
            _ => Message::Nothing,
        }
        Mode::Input(_) => match key {
            KeyCode::Char(c) => Message::EditInput(Edit::AppendChar(c)),
            KeyCode::Backspace => Message::EditInput(Edit::PopChar),
            KeyCode::Enter => Message::SubmitInput,
            KeyCode::Esc => Message::Cancel,
            _ => Message::Nothing,
        }
    }
}

fn handle_event(mode: &Mode) -> io::Result<Message> {
    let event::Event::Key(key) = event::read()? else {
        return Ok(Message::Nothing);
    };
    if key.kind != KeyEventKind::Press {
        return Ok(Message::Nothing);
    }
    Ok(key_to_message(mode, key.code))
}

fn update(mut model: Model, msg: Message) -> Model {
    match msg {
        Message::StartInput => model.mode = Mode::Input(String::new()),
        Message::EditInput(edit) => {
            if let Mode::Input(ref mut label) = model.mode {
                match edit {
                    Edit::AppendChar(c) => label.push(c),
                    Edit::PopChar => { label.pop(); }
                }
            }
        }
        Message::SubmitInput => {
            if let Mode::Input(label) = model.mode {
                model.heap = heap::prepend(model.heap, label);
                model.mode = Mode::Normal;
            }
        }
        Message::Cancel => model.mode = Mode::Normal,
        Message::Quit => model.quit = true,
        Message::Nothing => (),
    }
    model
}

fn main_loop(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut model = Model::new();
    while !model.quit {
        terminal.draw(|frame| view(&model, frame))?;
        let msg = handle_event(&model.mode)?;
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

