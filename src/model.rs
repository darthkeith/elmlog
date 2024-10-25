use crate::heap::Heap;

/// Action to be performed with the user input string.
pub enum InputAction {
    Insert,
    Edit(usize),
}

/// Current user input and action to be performed with it.
pub struct InputState {
    pub input: String,
    pub action: InputAction,
}

/// A choice between two items.
pub struct Choice {
    pub item1: String,
    pub item2: String,
    pub first_selected: bool,
}

/// Operational modes of the application.
pub enum Mode {
    Normal,
    Input(InputState),
    Select(usize),
    Selected(usize),
    Compare(Choice),
}

/// State of the entire application.
pub struct Model {
    pub heap: Heap,
    pub mode: Mode,
}

