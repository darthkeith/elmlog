use std::io;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{
        block::{Block, BorderType},
        Paragraph,
        Widget,
    },
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

fn view(_model: &Model) -> Paragraph {
    let block = Block::bordered()
        .border_type(BorderType::Thick);
    Paragraph::new("Hello, world!")
        .centered()
        .block(block)
}

impl Widget for &Model {
    fn render(self, area: Rect, buf: &mut Buffer) {
        view(self).render(area,buf);
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
        terminal.draw(|frame| frame.render_widget(&model, frame.area()))?;
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

