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
fn trim_input(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Update the `model` based on the `message`.
pub fn update(mut model: Model, message: Message) -> Model {
    match (message, &mut model.mode) {
        (Message::StartInput, Mode::Normal) => {
            model.mode = Mode::Input(String::new());
        }
        (Message::EditInput(edit), Mode::Input(input)) => {
            match edit {
                Edit::AppendChar(c) => input.push(c),
                Edit::PopChar => { input.pop(); }
            }
        }
        (Message::StartDelete, Mode::Normal) => {
            if model.heap.size() > 0 {
                model.mode = Mode::Delete(0);
            }
        }
        (Message::AppendDelete(c), Mode::Delete(index)) => {
            let i = new_index(c, *index, model.heap.size());
            model.mode = Mode::Delete(i);
        }
        (Message::Submit, Mode::Input(input)) => {
            if let Some(label) = trim_input(&input) {
                model.heap = model.heap.prepend(label);
            }
            model.mode = Mode::Normal;
        }
        (Message::Submit, Mode::Delete(index)) => {
            model.heap = model.heap.delete(*index);
            model.mode = Mode::Normal;
        }
        (Message::Cancel, Mode::Input(_) | Mode::Delete(_)) => {
            model.mode = Mode::Normal;
        }
        (Message::Quit, Mode::Normal) => model.quit = true,
        (Message::Nothing, _) => (),
        _ => panic!("Invalid message in current mode."),
    }
    model
}

