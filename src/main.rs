mod io;
mod message;
mod model;
mod update;
mod view;
mod zipper;

use std::io::Result;

use ratatui::DefaultTerminal;

use crate::{
    io::execute_command,
    message::{Command, handle_event},
    model::Model,
    update::update,
    view::view,
};

fn main_loop(mut terminal: DefaultTerminal) -> Result<()> {
    let mut model = execute_command(Command::Load).unwrap();
    loop {
        terminal.draw(|frame| view(&model, frame))?;
        let Model { state, mode } = model;
        let message = handle_event(mode)?;
        let command = update(message, state);
        model = match execute_command(command) {
            Some(updated_model) => updated_model,
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

