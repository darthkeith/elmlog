mod io;
mod message;
mod model;
mod update;
mod view;
mod zipper;

use std::io::Result;

use ratatui::DefaultTerminal;

use crate::{
    io::{Command, execute_command},
    message::handle_input,
    update::update,
    view::view,
};

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    terminal.clear()?;
    let mut model = execute_command(Command::Load { quit: false }).unwrap();
    loop {
        terminal.draw(|frame| view(&model, frame))?;
        let message = handle_input(model)?;
        let command = update(message);
        model = match execute_command(command) {
            Some(model) => model,
            None => return Ok(()),
        }
    }
}

fn main() -> Result<()> {
    ratatui::run(run)
}
