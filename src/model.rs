use crate::heap::Heap;

/// Indicates which of two items is selected.
pub enum Selected {
    First,
    Second,
}

/// Represents a choice between two items.
pub struct Choice {
    pub item1: String,
    pub item2: String,
    pub selected: Selected,
}

/// Represents the operational modes of the application.
pub enum Mode {
    Normal,
    Input(String),
    Select(usize),
    Compare(Choice)
}

/// Contains all application state.
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

