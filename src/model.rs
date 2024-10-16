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

/// Indicates which of two items is selected.
pub enum Selected {
    First,
    Second,
}

/// A choice between two items.
pub struct Choice {
    pub item1: String,
    pub item2: String,
    pub selected: Selected,
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

impl Model {
    /// Construct a model storing the initial application state.
    pub fn new() -> Self {
        Model {
            heap: Heap::Empty,
            mode: Mode::Normal,
        }
    }
}

