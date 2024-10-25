mod heap;
mod message;
mod model;
mod update;
mod view;

use std::{
    fs::{self, File},
    io::Result,
    path::Path,
};

use ratatui::DefaultTerminal;

use crate::{
    heap::Heap,
    message::{Message, NormalMsg, handle_event},
    model::{Mode, Model},
    update::update,
    view::view,
};

const APP_DATA_FILE: &str = ".app_data";

// Construct a Heap from the saved data file if it exists.
fn init_heap() -> Heap {
    if !Path::new(APP_DATA_FILE).exists() {
        return Heap::Empty;
    }
    let buffer = fs::read(APP_DATA_FILE)
        .expect("Failed to read application data file.");
    bincode::deserialize(&buffer)
        .expect("Failed to deserialize data.")
}

// Save the Heap to the data file.
fn save_heap(heap: Heap) {
    let file = File::create(APP_DATA_FILE)
        .expect("Failed to create application data file.");
    bincode::serialize_into(file, &heap)
        .expect("Failed to serialise data.");
}

fn main_loop(mut terminal: DefaultTerminal) -> Result<()> {
    let mut model = Model {
        heap: init_heap(),
        mode: Mode::Normal,
    };
    loop {
        terminal.draw(|frame| view(&model, frame))?;
        let message = handle_event(model.mode)?;
        if let Message::Normal(NormalMsg::Quit) = message {
            save_heap(model.heap);
            return Ok(());
        }
        model = update(message, model.heap);
    }
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let result = main_loop(terminal);
    ratatui::restore();
    result
}

