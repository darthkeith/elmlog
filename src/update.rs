use crate::model::{Mode, Model};
use crate::msg::{Edit, Message};

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
pub fn update(mut model: Model, msg: Message) -> Model {
    match msg {
        Message::StartInput => model.mode = Mode::Input(String::new()),
        Message::EditInput(edit) => {
            if let Mode::Input(ref mut input) = model.mode {
                match edit {
                    Edit::AppendChar(c) => input.push(c),
                    Edit::PopChar => { input.pop(); }
                }
            }
        }
        Message::SubmitInput => {
            if let Mode::Input(input) = model.mode {
                if let Some(label) = trim_input(input) {
                    model.heap = model.heap.prepend(label);
                }
                model.mode = Mode::Normal;
            }
        }
        Message::Cancel => model.mode = Mode::Normal,
        Message::Quit => model.quit = true,
        Message::Nothing => (),
    }
    model
}

  
