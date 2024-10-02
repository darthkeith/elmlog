use crate::model::{Mode, Model};
use crate::message::{Edit, Message};

// Return the new index given the charcter to append and the current heap size.
fn new_index(c: char, index: usize, heap_size: usize) -> usize {
    if !c.is_ascii_digit() {
        return index;
    }
    let idx_str = format!("{index}{c}");
    if let Ok(new_index) = idx_str.parse::<usize>() {
        if new_index < heap_size {
            return new_index;
        }
    }
    let c_val = (c as usize) - ('0' as usize);
    if c_val < heap_size {
        return c_val;
    }
    return index;
}

// Trim the `input` string and return the result if non-empty.
fn trim_input(input: String) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Update the `model` based on the message.
pub fn update(mut model: Model, message: Message) -> Model {
    match message {
        Message::StartInput => model.mode = Mode::Input(String::new()),
        Message::EditInput(edit) => {
            if let Mode::Input(ref mut input) = model.mode {
                match edit {
                    Edit::AppendChar(c) => input.push(c),
                    Edit::PopChar => { input.pop(); }
                }
            }
        }
        Message::StartDelete => {
            if model.heap.size() != 0 {
                model.mode = Mode::Delete(0);
            }
        }
        Message::AppendDelete(c) => {
            if let Mode::Delete(index) = model.mode {
                let i = new_index(c, index, model.heap.size());
                model.mode = Mode::Delete(i);
            }
        }
        Message::Submit => {
            match model.mode {
                Mode::Normal => (),
                Mode::Input(input) => {
                    if let Some(label) = trim_input(input) {
                        model.heap = model.heap.prepend(label);
                    }
                }
                Mode::Delete(index) => {
                    model.heap = model.heap.delete(index);
                }
            }
            model.mode = Mode::Normal;
        }
        Message::Cancel => model.mode = Mode::Normal,
        Message::Quit => model.quit = true,
        Message::Nothing => (),
    }
    model
}

