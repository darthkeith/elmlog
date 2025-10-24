use crate::{
    io::{LoadState, OpenDataFile},
    zipper::{FocusNode, FocusNodeExt},
};

/// Persistent state for an active session.
pub struct SessionState {
    pub focus: Option<FocusNode>,
    pub maybe_file: Option<OpenDataFile>,
    pub changed: bool,
}

/// Action to perform after saving.
pub enum PostSaveAction {
    Load,
    Quit,
}

/// User's current save choice and subsequent action.
pub struct SaveState {
    pub save: bool,
    pub post_save: PostSaveAction,
    pub session: SessionState,
}

/// Action to perform with the user input label string.
pub enum LabelAction {
    Insert,
    Rename,
}

/// Current user input label and action to be performed with it.
pub struct LabelState {
    pub input: String,
    pub action: LabelAction,
    pub session: SessionState,
}

/// Status of the user input filename string.
pub enum FilenameStatus {
    Empty,
    Exists,
    Invalid,
    Valid,
}

/// Action to perform with the user input filename string.
pub enum FilenameAction {
    Rename(LoadState),
    SaveNew {
        session: SessionState,
        post_save: PostSaveAction,
    },
}

/// Current user input filename with status and next action to be performed.
pub struct FilenameState {
    pub input: String,
    pub status: FilenameStatus,
    pub action: FilenameAction,
}

/// Action to be confirmed in Confirm mode.
pub enum ConfirmState {
    NewSession,
    DeleteItem(SessionState),
    DeleteFile(LoadState),
}

/// Complete application state, with a variant for each mode.
pub enum Model {
    Load(LoadState),
    Normal(SessionState),
    Insert(SessionState),
    Move(SessionState),
    Save(SaveState),
    LabelInput(LabelState),
    FilenameInput(FilenameState),
    Confirm(ConfirmState),
}

impl LabelState {
    /// Create a LabelState to insert a new item.
    pub fn new_insert(session: SessionState) -> Self {
        Self {
            input: String::new(),
            action: LabelAction::Insert,
            session,
        }
    }

    /// Create a LabelState to rename the `label` of the focused node.
    pub fn new_rename(label: String, session: SessionState) -> Self {
        Self {
            input: label,
            action: LabelAction::Rename,
            session,
        }
    }

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
}

impl FilenameState {
    /// Create a FilenameState to rename a file.
    pub fn new_rename(load_state: LoadState) -> Self {
        Self {
            input: String::new(),
            status: FilenameStatus::Empty,
            action: FilenameAction::Rename(load_state),
        }
    }

    /// Create a FilenameState to save a new file.
    pub fn new_save(session: SessionState, post_save: PostSaveAction) -> Self {
        Self {
            input: String::new(),
            status: FilenameStatus::Empty,
            action: FilenameAction::SaveNew {
                session,
                post_save,
            },
        }
    }

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

    /// Set the filename status.
    pub fn set_status(mut self, status: FilenameStatus) -> Self {
        self.status = status;
        self
    }

    /// Return whether the input is a valid filename.
    pub fn is_valid(&self) -> bool {
        matches!(self.status, FilenameStatus::Valid)
    }

    /// Return a reference to the trimmed user input.
    pub fn trimmed(&self) -> &str {
        self.input.trim()
    }
}

impl SaveState {
    /// Create a SaveState for subsequently loading.
    pub fn new_load(session: SessionState) -> Self {
        Self {
            save: true,
            post_save: PostSaveAction::Load,
            session,
        }
    }

    /// Create a SaveState for subsequently quitting.
    pub fn new_quit(session: SessionState) -> Self {
        Self {
            save: true,
            post_save: PostSaveAction::Quit,
            session,
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
    pub fn new() -> Self {
        Self {
            focus: None,
            maybe_file: None,
            changed: false,
        }
    }

    /// Focus on the parent of the current focused node (if present).
    pub fn focus_parent(self) -> Self {
        Self {
            focus: self.focus.focus_parent(),
            ..self
        }
    }

    /// Focus on the first child of the current focused node (if present).
    pub fn focus_child(self) -> Self {
        Self {
            focus: self.focus.focus_child(),
            ..self
        }
    }

    /// Focus on the previous sibling of the current focused node (if present).
    pub fn focus_prev(self) -> Self {
        Self {
            focus: self.focus.focus_prev(),
            ..self
        }
    }

    /// Focus on the next sibling of the current focused node (if present).
    pub fn focus_next(self) -> Self {
        Self {
            focus: self.focus.focus_next(),
            ..self
        }
    }

    /// Move the focused node's subtree to be its parent's next sibling.
    pub fn promote(self) -> Self {
        Self {
            focus: self.focus.promote(),
            changed: true,
            ..self
        }
    }

    /// Move the focused node's subtree to be its previous sibling's last child.
    pub fn demote(self) -> Self {
        Self {
            focus: self.focus.demote(),
            changed: true,
            ..self
        }
    }

    /// Swap the focused node's subtree with its previous sibling (if present).
    pub fn swap_prev(self) -> Self {
        Self {
            focus: self.focus.swap_prev(),
            changed: true,
            ..self
        }
    }

    /// Swap the focused node's subtree with its next sibling (if present).
    pub fn swap_next(self) -> Self {
        Self {
            focus: self.focus.swap_next(),
            changed: true,
            ..self
        }
    }

    /// Move the siblings of the focused node to be its children.
    pub fn nest(self) -> Self {
        Self {
            focus: self.focus.nest(),
            changed: true,
            ..self
        }
    }

    /// Insert the focused node's children before its subsequent siblings.
    pub fn flatten(self) -> Self {
        Self {
            focus: self.focus.flatten(),
            changed: true,
            ..self
        }
    }

    /// Insert a new node as the parent of the focused node.
    pub fn insert_parent(self) -> Self {
        Self {
            focus: self.focus.insert_parent(),
            changed: true,
            ..self
        }
    }

    /// Insert a new child node above the focused node's children.
    pub fn insert_child(self) -> Self {
        Self {
            focus: self.focus.insert_child(),
            changed: true,
            ..self
        }
    }

    /// Insert a new node as the previous sibling of the focused node.
    pub fn insert_prev(self) -> Self {
        Self {
            focus: self.focus.insert_prev(),
            changed: true,
            ..self
        }
    }

    /// Insert a new node as the next sibling of the focused node.
    pub fn insert_next(self) -> Self {
        Self {
            focus: self.focus.insert_next(),
            changed: true,
            ..self
        }
    }

    /// Delete the selected item.
    pub fn delete(self) -> Self {
        Self {
            focus: self.focus.delete(),
            changed: true,
            ..self
        }
    }

    /// Set the label of the focused node.
    pub fn set_label(self, label: String) -> Self {
        Self {
            focus: self.focus.set_label(label),
            changed: true,
            ..self
        }
    }

    /// Return the filename if it exists.
    pub fn get_filename(&self) -> Option<&str> {
        self.maybe_file.as_ref().map(|file| file.name.as_str())
    }
}

