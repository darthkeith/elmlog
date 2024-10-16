use crate::heap::{Heap, HeapStatus};
use crate::model::{
    Choice,
    InputAction,
    InputState,
    Mode,
    Model,
};
use crate::message::{
    Message,
    NormalMsg,
    InputMsg,
    SelectMsg,
    SelectedMsg,
    CompareMsg,
};

// Trim the `input` string and return the result if non-empty.
fn trim_input(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

// Append a digit to `index` if valid, otherwise return a fallback value.
fn append_index(index: usize, c: char, heap_size: usize) -> usize {
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

// Return the next Model based on a message sent in Normal mode.
fn update_normal(msg: NormalMsg, heap: Heap) -> Model {
    let mode = match msg {
        NormalMsg::StartInput => {
            let state = InputState {
                input: String::new(),
                action: InputAction::Insert,
            };
            Mode::Input(state)
        }
        NormalMsg::StartSelect => {
            match heap.size() > 0 {
                true => Mode::Select(0),
                false => Mode::Normal
            }
        }
        NormalMsg::StartCompare => {
            match heap.status() {
                HeapStatus::MultiRoot(item1, item2) => Mode::Compare(
                    Choice {
                        item1: item1.to_string(),
                        item2: item2.to_string(),
                        first_selected: true,
                    }
                ),
                _ => Mode::Normal,
            }
        }
        NormalMsg::Quit => Mode::Normal,
    };
    Model { heap, mode }
}

// Return the next Model based on a message sent in Input mode.
fn update_input(msg: InputMsg, mut state: InputState, mut heap: Heap) -> Model {
    let mode = match msg {
        InputMsg::Append(c) => {
            if !(state.input.is_empty() && c == ' ') {
                state.input.push(c);
            }
            Mode::Input(state)
        }
        InputMsg::PopChar => {
            state.input.pop();
            Mode::Input(state)
        }
        InputMsg::Submit => {
            if let Some(label) = trim_input(&state.input) {
                match state.action {
                    InputAction::Insert => {
                        heap = heap.prepend(label);
                    }
                    InputAction::Edit(index) => {
                        heap.set_label(index, label);
                    }
                }
            }
            Mode::Normal
        }
    };
    Model { heap, mode }
}

// Return the next Model based on a message sent in Select mode.
fn update_select(msg: SelectMsg, index: usize, heap: Heap) -> Model {
    let mode = match msg {
        SelectMsg::Append(c) => {
            let i = append_index(index, c, heap.size());
            Mode::Select(i)
        }
        SelectMsg::Decrement => {
            match index > 0 {
                true => Mode::Select(index - 1),
                false => Mode::Select(index),
            }
        }
        SelectMsg::Increment => {
            match index + 1 < heap.size() {
                true => Mode::Select(index + 1),
                false => Mode::Select(index)
            }
        }
        SelectMsg::Confirm => Mode::Selected(index),
    };
    Model { heap, mode }
}

// Return the next Model based on a message sent in Selected mode.
fn update_selected(msg: SelectedMsg, index: usize, mut heap: Heap) -> Model {
    let mode = match msg {
        SelectedMsg::Edit => {
            let state = InputState {
                input: heap.label_at(index).to_string(),
                action: InputAction::Edit(index),
            };
            Mode::Input(state)
        }
        SelectedMsg::Delete => {
            heap = heap.delete(index);
            Mode::Normal
        }
    };
    Model { heap, mode }
}

// Return the next Model based on a message sent in Compare mode.
fn update_compare(msg: CompareMsg, choice: Choice, mut heap: Heap) -> Model {
    let Choice { item1, item2, first_selected } = choice;
    let mode = match msg {
        CompareMsg::Toggle => {
            let toggled = !first_selected;
            Mode::Compare(Choice { item1, item2, first_selected: toggled })
        }
        CompareMsg::Confirm => {
            heap = heap.merge_pair(first_selected);
            Mode::Normal
        }
    };
    Model { heap, mode }
}

/// Return the next Model based on the `message` and the `heap`.
pub fn update(message: Message, heap: Heap) -> Model {
    match message {
        Message::Normal(msg) => update_normal(msg, heap),
        Message::Input(msg, state) => update_input(msg, state, heap),
        Message::Select(msg, index) => update_select(msg, index, heap),
        Message::Selected(msg, index) => update_selected(msg, index, heap),
        Message::Compare(msg, choice) => update_compare(msg, choice, heap),
        Message::Continue(mode) => Model { heap, mode },
    }
}

