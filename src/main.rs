mod heap;
mod io;
mod message;
mod model;
mod update;
mod view;

use std::io::Result;

use ratatui::DefaultTerminal;

use crate::{
    message::{Message, NormalMsg, handle_event},
    model::Model,
    update::update,
    view::view,
};

fn main_loop(mut terminal: DefaultTerminal) -> Result<()> {
    let mut model = Model::init();
    loop {
        terminal.draw(|frame| view(&model, frame))?;
        let message = handle_event(model.mode)?;
        if let Message::Normal(NormalMsg::Quit) = message {
            return io::save(model.state);
        }
        model = update(message, model.state);
    }
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let result = main_loop(terminal);
    ratatui::restore();
    result
}

