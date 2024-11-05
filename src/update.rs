use crate::{
    heap::HeapStatus,
    message::{
        CompareMsg,
        InputMsg,
        Message,
        NormalMsg,
        SelectedMsg,
        SelectMsg,
    },
    model::{
        Choice,
        InputAction,
        InputState,
        Mode,
        Model,
        SessionState,
    },
};

// Trim the `input` string and return the result if non-empty.
fn trim_input(input: String) -> Option<String> {
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
fn update_normal(msg: NormalMsg, state: SessionState) -> Model {
    let mode = match msg {
        NormalMsg::StartInput => {
            let input_state = InputState {
                input: String::new(),
                action: InputAction::Insert,
            };
            Mode::Input(input_state)
        }
        NormalMsg::StartSelect => {
            match state.heap.size() > 0 {
                true => Mode::Select(0),
                false => Mode::Normal,
            }
        }
        NormalMsg::StartCompare => {
            match state.heap.status() {
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
        NormalMsg::Quit => Mode::Save(true),
    };
    Model { state, mode }
}

// Return the next Model based on a message sent in Input mode.
fn update_input(
    msg: InputMsg,
    mut input_state: InputState,
    mut state: SessionState,
) -> Model {
    let mode = match msg {
        InputMsg::Append(c) => {
            if !(input_state.input.is_empty() && c == ' ') {
                input_state.input.push(c);
            }
            Mode::Input(input_state)
        }
        InputMsg::PopChar => {
            input_state.input.pop();
            Mode::Input(input_state)
        }
        InputMsg::Submit => {
            let InputState { input, action } = input_state;
            if let Some(label) = trim_input(input) {
                match action {
                    InputAction::Insert => {
                        state.heap = state.heap.prepend(label);
                    }
                    InputAction::Edit(index) => {
                        state.heap.set_label(index, label);
                    }
                }
            }
            Mode::Normal
        }
    };
    Model { state, mode }
}

// Return the next Model based on a message sent in Select mode.
fn update_select(msg: SelectMsg, index: usize, state: SessionState) -> Model {
    let mode = match msg {
        SelectMsg::Append(c) => {
            let i = append_index(index, c, state.heap.size());
            Mode::Select(i)
        }
        SelectMsg::Decrement => {
            match index > 0 {
                true => Mode::Select(index - 1),
                false => Mode::Select(index),
            }
        }
        SelectMsg::Increment => {
            match index + 1 < state.heap.size() {
                true => Mode::Select(index + 1),
                false => Mode::Select(index),
            }
        }
        SelectMsg::Confirm => Mode::Selected(index),
    };
    Model { state, mode }
}

// Return the next Model based on a message sent in Selected mode.
fn update_selected(
    msg: SelectedMsg,
    index: usize,
    mut state: SessionState,
) -> Model {
    let mode = match msg {
        SelectedMsg::Edit => {
            let input_state = InputState {
                input: state.heap.label_at(index).to_string(),
                action: InputAction::Edit(index),
            };
            Mode::Input(input_state)
        }
        SelectedMsg::Delete => {
            state.heap = state.heap.delete(index);
            Mode::Normal
        }
    };
    Model { state, mode }
}

// Return the next Model based on a message sent in Compare mode.
fn update_compare(
    msg: CompareMsg,
    choice: Choice,
    mut state: SessionState,
) -> Model {
    let Choice { item1, item2, first_selected } = choice;
    let mode = match msg {
        CompareMsg::Toggle => {
            let toggled = !first_selected;
            Mode::Compare(Choice { item1, item2, first_selected: toggled })
        }
        CompareMsg::Confirm => {
            state.heap = state.heap.merge_pair(first_selected);
            Mode::Normal
        }
    };
    Model { state, mode }
}

/// Return the next Model based on the `message` and the session `state`.
pub fn update(message: Message, state: SessionState) -> Model {
    match message {
        Message::Normal(msg) => update_normal(msg, state),
        Message::Input(msg, input_state) => update_input(msg, input_state, state),
        Message::Select(msg, index) => update_select(msg, index, state),
        Message::Selected(msg, index) => update_selected(msg, index, state),
        Message::Compare(msg, choice) => update_compare(msg, choice, state),
        Message::ToggleSave(save) => Model { state, mode: Mode::Save(!save) },
        Message::Quit(_) => Model { state, mode: Mode::Normal },
        Message::Continue(mode) => Model { state, mode },
    }
}

