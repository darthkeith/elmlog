mod heap;
mod io;
mod message;
mod model;
mod update;
mod view;

use std::io::Result;

use ratatui::DefaultTerminal;

use crate::{
    message::handle_event,
    model::Model,
    update::update,
    view::view,
};

fn main_loop(mut terminal: DefaultTerminal) -> Result<()> {
    let mut model = Model::init();
    loop {
        terminal.draw(|frame| view(&model, frame))?;
        let message = handle_event(model.mode)?;
        model = match update(message, model.state) {
            Some(m) => m,
            None => return Ok(()),
        }
    }
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let result = main_loop(terminal);
    ratatui::restore();
    result
}

