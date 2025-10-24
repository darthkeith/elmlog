use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Result},
    path::{Path, PathBuf},
};

use fs2::FileExt;

use crate::{
    model::{
        ConfirmState,
        FilenameAction,
        FilenameState,
        FilenameStatus,
        Model,
        PostSaveAction,
        SessionState,
    },
    zipper::FocusNode,
};

const APP_DIR: &str = "elmlog";

/// The `name` and `path` of a file.
pub struct FileEntry {
    name: String,
    path: PathBuf,
}

/// List of `files` in the app directory and `index` of the current selection.
pub struct LoadState {
    files: Vec<FileEntry>,
    index: usize,
}

/// A file locked for exclusive data access.
///
/// The File is only stored to keep the lock active.
pub struct OpenDataFile {
    pub name: String,
    path: PathBuf,
    _file: File,
}

/// A message indicating an IO action to perform.
pub enum Command {
    None(Model),
    Load,
    InitSession(FileEntry),
    CheckFileExists(FilenameState),
    RenameFile(String, LoadState),
    SaveNew(String, SessionState, PostSaveAction),
    Save(SessionState, PostSaveAction),
    DeleteFile(LoadState),
    Quit,
}

impl FileEntry {
    fn rename(&self, filename: &str) -> Result<Self> {
        let path = app_dir_path().join(filename);
        fs::rename(&self.path, &path)?;
        Ok(FileEntry {
            name: filename.to_string(),
            path,
        })
    }
}

impl LoadState {
    /// Move the selected FileEntry.
    pub fn move_file_entry(mut self) -> FileEntry {
        self.files.swap_remove(self.index)
    }

    /// Decrement the `index`.
    pub fn decrement(self) -> Self {
        if self.index == 0 {
            self
        } else {
            LoadState {
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
            LoadState {
                index: self.index + 1,
                ..self
            }
        }
    }

    /// Iterate over the filenames.
    pub fn filename_iter(&self) -> impl Iterator<Item = &str> {
        self.files
            .iter()
            .map(|f| f.name.as_str())
    }

    /// Return the total number of files.
    pub fn size(&self) -> usize {
        self.files.len()
    }

    /// Return the current index.
    pub fn index(&self) -> usize {
        self.index
    }

    // Rename the selected file.
    fn rename(&mut self, filename: &str) -> Result<()> {
        let i = self.index;
        self.files[i] = self.files[i].rename(filename)?;
        Ok(())
    }

    // Delete the currently selected file and remove it from the list.
    // Return None if there are no files left.
    fn delete(mut self) -> Option<Self> {
        let entry = self.files.remove(self.index);
        fs::remove_file(entry.path)
            .expect("Failed to delete file");
        if self.files.is_empty() {
            return None;
        }
        if self.index == self.files.len() {
            self.index -= 1;
        }
        Some(self)
    }
}

// Return the application directory path, creating any missing directories.
fn app_dir_path() -> PathBuf {
    let data_dir = dirs::data_dir()
        .expect("Failed to identify data directory");
    let path = data_dir.join(APP_DIR);
    fs::create_dir_all(&path)
        .expect("Failed to create data directory");
    path
}

// Return the LoadState if there is a least one data file.
fn get_load_state() -> Option<LoadState> {
    let files: Vec<FileEntry> = fs::read_dir(app_dir_path())
        .expect("Unable to read app directory")
        .filter_map(Result::ok)
        .map(|entry| {
            let name = entry
                .file_name()
                .to_string_lossy()
                .into_owned();
            let path = entry.path();
            FileEntry { name, path }
        })
        .collect();
    match files.len() {
        0 => None,
        _ => Some(LoadState { files, index: 0 }),
    }
}

// Lock the `file` for exclusive data access.
fn lock(file: &File) {
    file.try_lock_exclusive()
        .expect("File is currently locked");
}

// Load a forest from a serialized data `file`.
fn load_forest(mut file: &File) -> Option<FocusNode> {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read file");
    bincode::deserialize(&buffer)
        .expect("Failed to deserialize data")
}

// Initialize a Model from a saved file.
fn init_model(file_entry: FileEntry) -> Model {
    let FileEntry { name, path } = file_entry;
    let file = OpenOptions::new()
        .read(true)
        .open(&path)
        .expect("Failed to open file");
    lock(&file);
    let focus = load_forest(&file);
    let open_file = OpenDataFile {
        name,
        path,
        _file: file,
    };
    let state = SessionState {
        focus,
        maybe_file: Some(open_file),
        changed: false,
    };
    Model::Normal(state)
}

// Check whether `filename` exists in the app directory.
fn filename_exists(filename: &str) -> bool {
    let path = app_dir_path().join(filename);
    path.exists()
}

// Return the forest and data file path (if present) from the session state.
// The locked File is implicitly dropped to unlock it.
fn unlock_state(state: SessionState) -> (Option<FocusNode>, Option<PathBuf>) {
    let SessionState { focus, maybe_file, .. } = state;
    let maybe_path = maybe_file
        .map(|open_file| open_file.path);
    (focus, maybe_path)
}

// Set whether the file's permissions are read only.
fn set_read_only(path: &Path, read_only: bool) {
    let mut permissions = File::open(path)
        .expect("Failed to open file")
        .metadata()
        .expect("Failed to extract metadata")
        .permissions();
    permissions.set_readonly(read_only);
    fs::set_permissions(path, permissions)
        .expect("Failed to set file permissions");
}

// Write the forest to an existing file at `path`.
fn write_to_file(focus: &Option<FocusNode>, path: &Path) {
    set_read_only(path, false);
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Failed to write to file");
    lock(&file);
    bincode::serialize_into(&file, focus)
        .expect("Failed to serialize data");
    set_read_only(path, true);
}

// Save the current session `state`.
fn save(state: SessionState) {
    let (focus, maybe_path) = unlock_state(state);
    if let Some(path) = maybe_path {
        write_to_file(&focus, &path);
    }
}

// Save the forest to `filename`.
fn save_new(focus: &Option<FocusNode>, filename: &str) -> Result<()> {
    let path = app_dir_path().join(filename);
    File::create_new(&path)?;
    write_to_file(focus, &path);
    Ok(())
}

/// Execute `command` and return the updated Model.
pub fn execute_command(command: Command) -> Option<Model> {
    let model = match command {
        Command::None(model) => model,
        Command::Load => match get_load_state() {
            Some(load_state) => Model::Load(load_state),
            None => Model::Confirm(ConfirmState::NewSession),
        }
        Command::InitSession(file_entry) => init_model(file_entry),
        Command::CheckFileExists(filename_state) => {
            let status = if filename_exists(filename_state.trimmed()) {
                FilenameStatus::Exists
            } else {
                FilenameStatus::Valid
            };
            Model::FilenameInput(filename_state.set_status(status))
        }
        Command::RenameFile(filename, mut load_state) => {
            let status = if filename_exists(&filename) {
                FilenameStatus::Exists
            } else if load_state.rename(&filename).is_err() {
                FilenameStatus::Invalid
            } else {
                return Some(Model::Load(load_state))
            };
            let filename_state = FilenameState {
                input: filename,
                status,
                action: FilenameAction::Rename(load_state),
            };
            Model::FilenameInput(filename_state)
        }
        Command::SaveNew(filename, session, post_save) => {
            let status = if filename_exists(&filename) {
                FilenameStatus::Exists
            } else if save_new(&session.focus, &filename).is_err() {
                FilenameStatus::Invalid
            } else {
                return match post_save {
                    PostSaveAction::Load => execute_command(Command::Load),
                    PostSaveAction::Quit => None,
                }
            };
            let filename_state = FilenameState {
                input: filename,
                status,
                action: FilenameAction::SaveNew { session, post_save },
            };
            Model::FilenameInput(filename_state)
        }
        Command::Save(state, action) => {
            save(state);
            return match action {
                PostSaveAction::Load => execute_command(Command::Load),
                PostSaveAction::Quit => None,
            }
        }
        Command::DeleteFile(load_state) => match load_state.delete() {
            Some(load_state) => Model::Load(load_state),
            None => Model::Confirm(ConfirmState::NewSession),
        }
        Command::Quit => return None,
    };
    Some(model)
}

