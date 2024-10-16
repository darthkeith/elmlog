mod heap;
mod model;
mod message;
mod update;
mod view;

use std::io;

use ratatui::DefaultTerminal;

use crate::model::Model;
use crate::message::{Message, NormalMsg, handle_event};
use crate::update::update;
use crate::view::view;

fn main_loop(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut model = Model::new();
    loop {
        terminal.draw(|frame| view(&model, frame))?;
        let message = handle_event(model.mode)?;
        if let Message::Normal(NormalMsg::Quit) = message { break; }
        model = update(message, model.heap);
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

