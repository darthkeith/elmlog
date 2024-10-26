mod heap;
mod message;
mod model;
mod update;
mod view;

use std::{
    fs::{File, OpenOptions},
    io::{Read, Result, Seek, SeekFrom},
    path::Path,
};

use fs2::FileExt;
use ratatui::DefaultTerminal;

use crate::{
    heap::Heap,
    message::{Message, NormalMsg, handle_event},
    model::{Mode, Model},
    update::update,
    view::view,
};

const APP_DATA_FILE: &str = ".app_data";

// Return the initialized heap with its associated data file.
fn init() -> Result<(Heap, File)> {
    let file_path = Path::new(APP_DATA_FILE);
    let file_exists = file_path.exists();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;
    file.try_lock_exclusive()
        .expect("Application data file is currently locked");
    if !file_exists {
        return Ok((Heap::Empty, file));
    }
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let heap = bincode::deserialize(&buffer)
        .expect("Failed to deserialize data");
    Ok((heap, file))
}

// Save the `heap` to the data `file`.
fn save_heap(heap: Heap, mut file: File) -> Result<()> {
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    bincode::serialize_into(&file, &heap)
        .expect("Failed to serialise data");
    file.unlock()
}

fn main_loop(mut terminal: DefaultTerminal) -> Result<()> {
    let (heap, file) = init()?;
    let mut model = Model { heap, mode: Mode::Normal };
    loop {
        terminal.draw(|frame| view(&model, frame))?;
        let message = handle_event(model.mode)?;
        if let Message::Normal(NormalMsg::Quit) = message {
            return save_heap(model.heap, file);
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

