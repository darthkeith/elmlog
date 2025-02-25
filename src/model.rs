use crate::{
    forest::Node,
    io::{LoadState, OpenDataFile},
};

/// Action to be confirmed in Confirm mode.
pub enum ConfirmState {
    NewSession,
    DeleteItem(String, usize),
    DeleteFile(LoadState),
}

/// Position to insert new node relative to selected node.
pub enum InsertPosition {
    Parent,
    Child,
    Before,
    After,
}

/// Action to perform with the user input label string.
pub enum LabelAction {
    Edit(usize),
    Insert(usize, InsertPosition),
}

/// Current user input label and action to be performed with it.
pub struct LabelState {
    pub input: String,
    pub action: LabelAction,
}

/// Action to perform after saving.
pub enum PostSaveAction {
    Load,
    Quit,
}

/// Action to perform with the user input filename string.
pub enum FilenameAction {
    Rename(LoadState),
    SaveNew(PostSaveAction),
}

/// Status of the user input filename string.
pub enum FilenameStatus {
    Empty,
    Exists,
    Invalid,
    Valid,
}

/// Current user input filename with status and next action to be performed.
pub struct FilenameState {
    pub input: String,
    pub action: FilenameAction,
    pub status: FilenameStatus,
}

/// Input mode state, storing either a label or filename input.
pub enum InputState {
    Label(LabelState),
    Filename(FilenameState),
}

/// User's current save choice and subsequent action.
pub struct SaveState {
    pub save: bool,
    pub post_save: PostSaveAction,
}

/// Operational modes of the application.
pub enum Mode {
    Confirm(ConfirmState),
    Load(LoadState),
    Normal,
    Input(InputState),
    Select(usize),
    Selected(usize),
    Move(usize),
    Insert(usize),
    Save(SaveState),
}

/// State that is persistent across modes within a given session.
pub struct SessionState {
    pub root: Node,
    pub maybe_file: Option<OpenDataFile>,
}

/// State of the entire application.
pub struct Model {
    pub state: SessionState,
    pub mode: Mode,
}

impl LabelState {
    /// Append a character to the input string, not starting with whitespace.
    pub fn append(mut self, c: char) -> Self {
        if !(self.input.is_empty() && c == ' ') {
            self.input.push(c);
        }
        self
    }

    /// Pop a character from the input text.
    pub fn pop(mut self) -> Self {
        self.input.pop();
        self
    }

    /// Return whether the input text is empty.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    /// Return the Input mode containing the LabelState.
    pub fn into_mode(self) -> Mode {
        Mode::Input(InputState::Label(self))
    }
}

impl FilenameState {
    /// Append a character to the input string, not starting with whitespace.
    pub fn append(mut self, c: char) -> Self {
        if !(self.input.is_empty() && c == ' ') {
            self.input.push(c);
        }
        self
    }

    /// Pop a character from the input text.
    pub fn pop(mut self) -> Self {
        self.input.pop();
        self
    }

    /// Return whether the input text is empty.
    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    /// Set the filename status.
    pub fn status(mut self, status: FilenameStatus) -> Self {
        self.status = status;
        self
    }

    /// Return the Input mode containing the FilenameState.
    pub fn into_mode(self) -> Mode {
        Mode::Input(InputState::Filename(self))
    }

    /// Return a reference to the trimmed user input.
    pub fn input(&self) -> &str {
        self.input.trim()
    }
}

impl InputState {
    /// Create an InputState to edit the `label` of the item at `index`.
    pub fn new_edit(label: String, index: usize) -> Self {
        InputState::Label(LabelState {
            input: label,
            action: LabelAction::Edit(index),
        })
    }

    /// Create an InputState to insert item at `position` relative to `index`.
    pub fn new_insert(index: usize, position: InsertPosition) -> Self {
        InputState::Label(LabelState {
            input: String::new(),
            action: LabelAction::Insert(index, position),
        })
    }

    /// Create an InputState to insert item in an empty forest.
    pub fn new_insert_empty() -> Self {
        Self::new_insert(0, InsertPosition::Before)
    }

    /// Create an InputState to rename a file.
    pub fn new_rename(load_state: LoadState) -> Self {
        InputState::Filename(FilenameState {
            input: String::new(),
            action: FilenameAction::Rename(load_state),
            status: FilenameStatus::Empty,
        })
    }

    /// Create an InputState to save a new file.
    pub fn new_save(post_save: PostSaveAction) -> Self {
        InputState::Filename(FilenameState {
            input: String::new(),
            action: FilenameAction::SaveNew(post_save),
            status: FilenameStatus::Empty,
        })
    }

    /// Return whether the user input is valid.
    pub fn is_valid(&self) -> bool {
        match self {
            InputState::Label(label_state) => !label_state.is_empty(),
            InputState::Filename(filename_state) => {
                matches!(filename_state.status, FilenameStatus::Valid)
            }
        }
    }

    /// Return a reference to the user input string.
    pub fn input(&self) -> &str {
        match self {
            InputState::Label(label_state) => &label_state.input,
            InputState::Filename(filename_state) => &filename_state.input,
        }
    }
}

impl SaveState {
    /// Create a SaveState for subsequently loading.
    pub fn new_load() -> Self {
        SaveState { save: true, post_save: PostSaveAction::Load }
    }

    /// Create a SaveState for subsequently quitting.
    pub fn new_quit() -> Self {
        SaveState { save: true, post_save: PostSaveAction::Quit }
    }

    /// Toggle the boolean indicating whether the user intends to save.
    pub fn toggle(mut self) -> Self {
        self.save = !self.save;
        self
    }
}

impl SessionState {
    // Create a SessionState with an empty forest and no saved file.
    fn new() -> Self {
        Self {
            root: Node::Empty,
            maybe_file: None,
        }
    }

    // Mark the session state as modified if a saved file exists.
    fn into_changed(mut self) -> Self {
        if let Some(ref mut open_file) = self.maybe_file {
            open_file.set_changed();
        }
        self
    }

    /// Return whether data was changed in the current session.
    pub fn is_changed(&self) -> bool {
        match &self.maybe_file {
            Some(open_file) => open_file.is_changed(),
            None => !matches!(self.root, Node::Empty),
        }
    }

    /// Change the label of the item at `index` to `label`.
    pub fn edit(mut self, index: usize, label: String) -> Self {
        self.root = self.root.set_label(index, label);
        self.into_changed()
    }

    /// Swap the subtree at `index` with its next sibling.
    pub fn move_forward(mut self, index: usize) -> (Self, usize) {
        let (new_root, new_index) = self.root.move_forward(index);
        self.root = new_root;
        (self.into_changed(), new_index)
    }

    /// Swap the subtree at `index` with its previous sibling.
    pub fn move_backward(mut self, index: usize) -> (Self, usize) {
        let (new_root, new_index) = self.root.move_backward(index);
        self.root = new_root;
        (self.into_changed(), new_index)
    }

    /// Move subtree at `index` to be its parent's next sibling.
    ///
    /// If it has no parent, move it to be the first tree in the forest.
    pub fn promote(mut self, index: usize) -> (Self, usize) {
        let (new_root, new_index) = self.root.promote(index);
        self.root = new_root;
        (self.into_changed(), new_index)
    }

    /// Move subtree at `index` to be its previous sibling's last child.
    pub fn demote(mut self, index: usize) -> (Self, usize) {
        let (new_root, new_index) = self.root.demote(index);
        self.root = new_root;
        (self.into_changed(), new_index)
    }

    /// Move the siblings of the node at `index` to be its children.
    pub fn raise(mut self, index: usize) -> (Self, usize) {
        let (new_root, new_index) = self.root.raise(index);
        self.root = new_root;
        (self.into_changed(), new_index)
    }

    /// Move the children of the node at `index` to be its subsequent siblings.
    pub fn flatten(mut self, index: usize) -> (Self, usize) {
        let (new_root, new_index) = self.root.flatten(index);
        self.root = new_root;
        (self.into_changed(), new_index)
    }

    /// Insert the `label` at `position` relative to `index`.
    pub fn insert(
        mut self,
        index: usize,
        position: InsertPosition,
        label: String,
    ) -> (Self, usize) {
        let (new_root, new_index) = match position {
            InsertPosition::Parent => self.root.insert_parent(index, label),
            InsertPosition::Child => self.root.insert_child(index, label),
            InsertPosition::Before => self.root.insert_before(index, label),
            InsertPosition::After => self.root.insert_after(index, label),
        };
        self.root = new_root;
        (self.into_changed(), new_index)
    }

    /// Delete the item at `index`.
    pub fn delete(mut self, index: usize) -> Self {
        self.root = self.root.delete(index);
        self.into_changed()
    }
}

impl Model {
    /// Create a default Model for when there are no saved files.
    pub fn default() -> Self {
        Model {
            state: SessionState::new(),
            mode: Mode::Confirm(ConfirmState::NewSession),
        }
    }

    /// Create a Model in Load mode containing the `load_state`.
    pub fn load(load_state: LoadState) -> Self {
        Model {
            state: SessionState::new(),
            mode: Mode::Load(load_state),
        }
    }

    /// Return the filename if it exists.
    pub fn get_filename(&self) -> Option<&str> {
        match &self.state.maybe_file {
            Some(open_file) => Some(open_file.get_name()),
            None => None,
        }
    }
}

