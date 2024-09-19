use std::io;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    DefaultTerminal,
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
        let msg = handle_event()?;
        update(&mut model, msg)
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

