use crate::{
    heap::Heap,
    io::{self, LoadState, OpenDataFile},
};

/// Action to be performed with the user input string.
pub enum InputAction {
    Insert,
    Edit(usize),
    Save,
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
    Load(LoadState),
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
    pub maybe_file: Option<OpenDataFile>,
}

impl SessionState {
    /// Return whether data was changed in the current session.
    pub fn is_changed(&self) -> bool {
        match &self.maybe_file {
            Some(open_file) => open_file.is_changed(),
            None => !matches!(self.heap, Heap::Empty),
        }
    }

    pub fn set_changed(&mut self) {
        if let Some(ref mut open_file) = self.maybe_file {
            open_file.set_changed();
        }
    }
}

/// State of the entire application.
pub struct Model {
    pub state: SessionState,
    pub mode: Mode,
}

impl Model {
    /// Return the initial Model.
    pub fn init() -> Self {
        let state = SessionState {
            heap: Heap::Empty,
            maybe_file: None,
        };
        let mode = match io::get_load_state() {
            Some(load_state) => Mode::Load(load_state),
            None => Mode::Normal,
        };
        Model { state, mode }
    }
}

