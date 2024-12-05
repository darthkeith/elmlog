use crate::{
    heap::Heap,
    io::{self, LoadState, OpenDataFile},
};

/// Status of the user input string as a potential file name.
pub enum FileNameStatus {
    Empty,
    Exists,
    Valid,
}

/// Action to be performed with the user input string.
pub enum InputAction {
    Insert,
    Edit(usize),
    Save(FileNameStatus),
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

/// State of the entire application.
pub struct Model {
    pub state: SessionState,
    pub mode: Mode,
}

impl FileNameStatus {
    // Check the status of the given file name.
    fn check(file_name: &str) -> Self {
        if file_name.is_empty() {
            FileNameStatus::Empty
        } else if io::file_name_exists(file_name) {
            FileNameStatus::Exists
        } else {
            FileNameStatus::Valid
        }
    }
}

impl InputState {
    /// Create an `InputState` to insert an item.
    pub fn new_insert() -> Self {
        InputState {
            input: String::new(),
            action: InputAction::Insert,
        }
    }

    /// Create an `InputState` to edit the `label` of the item at `index`.
    pub fn new_edit(label: String, index: usize) -> Self {
        InputState {
            input: label,
            action: InputAction::Edit(index),
        }
    }

    /// Create an `InputState` to save a new file.
    pub fn new_save() -> Self {
        InputState {
            input: String::new(),
            action: InputAction::Save(FileNameStatus::Empty),
        }
    }

    /// Update the file name status if a file is being saved.
    pub fn update_status(mut self) -> Self {
        if let InputAction::Save(_) = self.action {
            let status = FileNameStatus::check(self.input.trim());
            self.action = InputAction::Save(status);
        }
        self
    }

    /// Check whether the user input is valid.
    pub fn is_valid(&self) -> bool {
        if self.input.is_empty() {
            return false;
        }
        match self.action {
            InputAction::Save(FileNameStatus::Valid) => true,
            InputAction::Save(_) => false,
            _ => true
        }
    }

    /// Append a character to the input text.
    pub fn append(mut self, c: char) -> Self {
        if !(self.input.is_empty() && c == ' ') {
            self.input.push(c);
            self.update_status()
        } else {
            self
        }
    }

    /// Pop a character from the input text.
    pub fn pop(mut self) -> Self {
        if let Some(_) = self.input.pop() {
            self.update_status()
        } else {
            self
        }
    }
}

impl SessionState {
    /// Return whether data was changed in the current session.
    pub fn is_changed(&self) -> bool {
        match &self.maybe_file {
            Some(open_file) => open_file.is_changed(),
            None => !matches!(self.heap, Heap::Empty),
        }
    }

    // Mark the session state as modified if a saved file exists.
    fn set_changed(&mut self) {
        if let Some(ref mut open_file) = self.maybe_file {
            open_file.set_changed();
        }
    }

    /// Insert `label` at the front of the heap.
    pub fn insert(mut self, label: String) -> Self {
        self.heap = self.heap.prepend(label);
        self.set_changed();
        self
    }

    /// Change the label of the item at `index` to `label`.
    pub fn edit(mut self, index: usize, label: String) -> Self {
        self.heap.set_label(index, label);
        self.set_changed();
        self
    }

    /// Delete the item at `index`.
    pub fn delete(mut self, index: usize) -> Self {
        self.heap = self.heap.delete(index);
        self.set_changed();
        self
    }

    /// Merge the first two roots in the heap.
    pub fn merge_pair(mut self, promote_first: bool) -> Self {
        self.heap = self.heap.merge_pair(promote_first);
        self.set_changed();
        self
    }
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

