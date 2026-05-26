use std::{
    fs::File,
    path::PathBuf
};

use crate::zipper::FocusNode;

/// The `name` and `path` of a file.
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
}

/// List of `files` in the app directory and `index` of the current selection.
pub struct LoadState {
    pub files: Vec<FileEntry>,
    pub index: usize,
}

/// A file locked for exclusive data access.
///
/// `_file` is never accessed and is only stored to keep the lock active.
pub struct OpenDataFile {
    pub name: String,
    pub path: PathBuf,
    pub _file: File,
}

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

impl LoadState {
    /// Extract the selected FileEntry, consuming the instance.
    pub fn extract_selected(mut self) -> FileEntry {
        self.files.swap_remove(self.index)
    }

    /// Decrement the `index`.
    pub fn decrement(self) -> Self {
        if self.index == 0 {
            self
        } else {
            Self {
                index: self.index - 1,
                ..self
            }
        }
    }

    /// Increment the `index`.
    pub fn increment(self) -> Self {
        if self.index + 1 == self.files.len() {
            self
        } else {
            Self {
                index: self.index + 1,
                ..self
            }
        }
    }
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

    /// Set the label of the focused node to the trimmed user input.
    pub fn set_label(self) -> SessionState {
        let Self { input, session, .. } = self;
        let label = input.trim().to_string();
        SessionState {
            focus: session.focus.map(|focus| focus.set_label(label)),
            changed: true,
            ..session
        }
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

    /// Apply a navigation function to the focused node.
    pub fn navigate<F>(self, f: F) -> Self
    where
        F: FnOnce(FocusNode) -> FocusNode,
    {
        Self {
            focus: self.focus.map(f),
            ..self
        }
    }

    /// Apply a function to the focused node and track if a change occurred.
    pub fn map_focus<F>(self, f: F) -> Self
    where
        F: FnOnce(FocusNode) -> FocusNode,
    {
        let old_focus = self.focus.clone();
        let new_focus = self.focus.map(f);
        Self {
            changed:  self.changed || new_focus != old_focus,
            focus: new_focus,
            ..self
        }
    }

    /// Insert a new node as the parent of the focused node.
    pub fn insert_parent(self) -> Self {
        Self {
            focus: self.focus.map(FocusNode::insert_parent),
            ..self
        }
    }

    /// Insert a new child node above the focused node's children.
    pub fn insert_child(self) -> Self {
        Self {
            focus: self.focus.map(FocusNode::insert_child),
            ..self
        }
    }

    /// Insert a new node as the previous sibling of the focused node.
    pub fn insert_prev(self) -> Self {
        Self {
            focus: self.focus.map(FocusNode::insert_prev),
            ..self
        }
    }

    /// Insert a new node as the next sibling of the focused node.
    pub fn insert_next(self) -> Self {
        Self {
            focus: self.focus.map(FocusNode::insert_next),
            ..self
        }
    }

    /// Delete the selected item and optionally mark the state as changed.
    pub fn delete(self, commit_change: bool) -> Self {
        Self {
            focus: self.focus.and_then(FocusNode::delete),
            changed: self.changed || commit_change,
            ..self
        }
    }

    pub fn clone_label(&self) -> Option<String> {
        self.focus.as_ref().map(FocusNode::clone_label)
    }

    /// Return the filename if it exists.
    pub fn get_filename(&self) -> Option<&str> {
        self.maybe_file.as_ref().map(|file| file.name.as_str())
    }

    /// Return the forest and data file path (if present) from the session state.
    ///
    /// The locked File is implicitly dropped to unlock it.
    pub fn unlock_state(self) -> (Option<FocusNode>, Option<PathBuf>) {
        let Self { focus, maybe_file, .. } = self;
        let maybe_path = maybe_file
            .map(|open_file| open_file.path);
        (focus, maybe_path)
    }
}
