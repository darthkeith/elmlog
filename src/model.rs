use crate::{
    heap::Heap,
    io::{self, OpenDataFile},
};

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
    Save(bool),
}

/// State that is persistent across modes within a given session.
pub struct SessionState {
    pub heap: Heap,
    pub open_file: OpenDataFile,
    pub changed: bool,
}

/// State of the entire application.
pub struct Model {
    pub state: SessionState,
    pub mode: Mode,
}

impl Model {
    /// Return the initial Model.
    pub fn init() -> Self {
        Model {
            state: io::init_state(),
            mode: Mode::Normal,
        }
    }
}

