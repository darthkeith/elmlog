use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Result},
    path::{Path, PathBuf},
};

use fs2::FileExt;

use crate::{
    model::{
        FileEntry,
        LoadState,
        Model,
        OpenDataFile,
        SessionState,
    },
    zipper::FocusNode,
};

const APP_DIR: &str = "elmlog";

// Return the application directory path, creating any missing directories.
fn app_dir_path() -> PathBuf {
    let data_dir = dirs::data_dir()
        .expect("Failed to identify data directory");
    let path = data_dir.join(APP_DIR);
    fs::create_dir_all(&path)
        .expect("Failed to create data directory");
    path
}

// Rename a FileEntry and return the new one.
fn rename_file(file: &FileEntry, filename: &str) -> Result<FileEntry> {
    let path = app_dir_path().join(filename);
    fs::rename(&file.path, &path)?;
    Ok(FileEntry {
        name: filename.to_string(),
        path,
    })
}

/// Rename the selected file in the LoadState to `filename`.
pub fn rename_selected_file(load_state: &mut LoadState, filename: &str) -> Result<()> {
    let i = load_state.index;
    load_state.files[i] = rename_file(&load_state.files[i], filename)?;
    Ok(())
}

/// Delete the currently selected file and remove it from the list.
///
/// Return None if there are no files left.
pub fn delete_selected_file(mut load_state: LoadState) -> Option<LoadState> {
    let entry = load_state.files.remove(load_state.index);
    fs::remove_file(entry.path)
        .expect("Failed to delete file");
    if load_state.files.is_empty() {
        return None;
    }
    if load_state.index == load_state.files.len() {
        load_state.index -= 1;
    }
    Some(load_state)
}

/// Return the LoadState if there is a least one data file.
pub fn get_load_state() -> Option<LoadState> {
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

/// Initialize a Model from a saved file.
pub fn init_model(file_entry: FileEntry) -> Model {
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

/// Check whether `filename` exists in the app directory.
pub fn filename_exists(filename: &str) -> bool {
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

/// Save the current session `state`.
pub fn save(state: SessionState) {
    let (focus, maybe_path) = unlock_state(state);
    if let Some(path) = maybe_path {
        write_to_file(&focus, &path);
    }
}

/// Save the forest to `filename`.
pub fn save_new(focus: &Option<FocusNode>, filename: &str) -> Result<()> {
    let path = app_dir_path().join(filename);
    File::create_new(&path)?;
    write_to_file(focus, &path);
    Ok(())
}
