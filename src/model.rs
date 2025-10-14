use crate::{
    io::{LoadState, OpenDataFile},
    zipper::{FocusNode, FocusNodeExt},
};

/// Action to be confirmed in Confirm mode.
pub enum ConfirmState {
    NewSession,
    DeleteItem,
    DeleteFile(LoadState),
}

/// Position to insert new node relative to selected node.
pub enum InsertPosition {
    Empty,
    Parent,
    Child,
    Before,
    After,
}

/// Action to perform with the user input label string.
pub enum LabelAction {
    Rename,
    Insert(InsertPosition),
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
    Edit,
    Move,
    Insert,
    Save(SaveState),
}

/// State that is persistent across modes within a given session.
pub struct SessionState {
    pub focus: Option<FocusNode>,
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
    /// Create an InputState to rename the `label` of the selected item.
    pub fn new_rename_label(label: String) -> Self {
        InputState::Label(LabelState {
            input: label,
            action: LabelAction::Rename,
        })
    }

    /// Create an InputState to insert item at `position` relative to the focused node.
    pub fn new_insert(position: InsertPosition) -> Self {
        InputState::Label(LabelState {
            input: String::new(),
            action: LabelAction::Insert(position),
        })
    }

    /// Create an InputState to rename a file.
    pub fn new_rename_file(load_state: LoadState) -> Self {
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
        SaveState {
            save: true,
            post_save: PostSaveAction::Load,
        }
    }

    /// Create a SaveState for subsequently quitting.
    pub fn new_quit() -> Self {
        SaveState {
            save: true,
            post_save: PostSaveAction::Quit,
        }
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
            focus: None,
            maybe_file: None,
        }
    }

    // Mark the session state as modified if a saved file exists.
    fn into_changed(mut self) -> Self {
        if let Some(open_file) = self.maybe_file.as_mut() {
            open_file.set_changed();
        }
        self
    }

    /// Return whether data was changed in the current session.
    pub fn is_changed(&self) -> bool {
        match &self.maybe_file {
            Some(open_file) => open_file.is_changed(),
            None => self.focus.is_some(),
        }
    }

    /// Set the label of the focused node.
    pub fn set_label(mut self, label: String) -> Self {
        self.focus = self.focus.set_label(label);
        self.into_changed()
    }

    /// Move the focused node's subtree to be its parent's next sibling.
    pub fn promote(mut self) -> Self {
        self.focus = self.focus.promote();
        self.into_changed()
    }

    /// Move the focused node's subtree to be its previous sibling's last child.
    pub fn demote(mut self) -> Self {
        self.focus = self.focus.demote();
        self.into_changed()
    }

    /// Swap the focused node's subtree with its previous sibling (if present).
    pub fn swap_prev(mut self) -> Self {
        self.focus = self.focus.swap_prev();
        self.into_changed()
    }

    /// Swap the focused node's subtree with its next sibling (if present).
    pub fn swap_next(mut self) -> Self {
        self.focus = self.focus.swap_next();
        self.into_changed()
    }

    /// Move the siblings of the focused node to be its children.
    pub fn nest(mut self) -> Self {
        self.focus = self.focus.nest();
        self.into_changed()
    }

    /// Insert the focused node's children before its subsequent siblings.
    pub fn flatten(mut self) -> Self {
        self.focus = self.focus.flatten();
        self.into_changed()
    }

    /// Insert `label` at `position` relative to the focused node.
    pub fn insert(mut self, position: InsertPosition, label: String) -> Self {
        self.focus = match position {
            InsertPosition::Empty => Some(FocusNode::new(label)),
            InsertPosition::Parent => self.focus.insert_parent(label),
            InsertPosition::Child => self.focus.insert_child(label),
            InsertPosition::Before => self.focus.insert_prev(label),
            InsertPosition::After => self.focus.insert_next(label),
        };
        self.into_changed()
    }

    /// Delete the selected item.
    pub fn delete(mut self) -> Self {
        self.focus = self.focus.delete();
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

