use crate::heap::{Heap, HeapStatus};
use crate::model::{Choice, Mode, Model, Selected};
use crate::message::Message;

// Append a digit to `index` if valid, otherwise return a fallback value.
fn find_new_index(index: usize, c: char, heap_size: usize) -> usize {
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

/// Return the next model based on the `message` and the `heap`.
pub fn update(message: Message, mut heap: Heap) -> Model {
    let mode = match message {
        Message::StartInput => Mode::Input(String::new()),
        Message::InputAppend(mut input, c) => {
            input.push(c);
            Mode::Input(input)
        }
        Message::InputPopChar(mut input) => {
            input.pop();
            Mode::Input(input)
        }
        Message::Insert(input) => {
            if let Some(label) = trim_input(&input) {
                heap = heap.prepend(label);
            }
            Mode::Normal
        }
        Message::StartSelect => {
            match heap.size() > 0 {
                true => Mode::Select(0),
                false => Mode::Normal
            }
        }
        Message::SelectAppend(index, c) => {
            let i = find_new_index(index, c, heap.size());
            Mode::Select(i)
        }
        Message::SelectDecrement(index) => {
            match index > 0 {
                true => Mode::Select(index - 1),
                false => Mode::Select(index),
            }
        }
        Message::SelectIncrement(index) => {
            match index + 1 < heap.size() {
                true => Mode::Select(index + 1),
                false => Mode::Select(index)
            }
        }
        Message::Delete(index) => {
            heap = heap.delete(index);
            Mode::Normal
        }
        Message::StartCompare => {
            match heap.status() {
                HeapStatus::MultiRoot(item1, item2) => Mode::Compare(
                    Choice {
                        item1: item1.to_string(),
                        item2: item2.to_string(),
                        selected: Selected::First,
                    }
                ),
                _ => Mode::Normal,
            }
        }
        Message::Toggle(Choice { item1, item2, selected }) => {
            let toggled = match selected {
                Selected::First => Selected::Second,
                Selected::Second => Selected::First,
            };
            Mode::Compare(Choice { item1, item2, selected: toggled })
        }
        Message::Compare(Choice { selected, .. }) => {
            let promote_first = match selected {
                Selected::First => true,
                Selected::Second => false,
            };
            heap = heap.merge_pair(promote_first);
            Mode::Normal
        }
        Message::Continue(mode) => mode,
        Message::Quit => Mode::Normal,
    };
    Model { heap, mode }
}

